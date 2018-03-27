use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use iron::headers::ContentType;
use rustc_serialize::json;
use database::Database;
use std::error::Error;
use iron::headers::{AccessControlAllowOrigin, AccessControlAllowCredentials, AccessControlAllowHeaders, AccessControlAllowMethods};
use unicase::UniCase;
use iron::method::Method;
use jwt::{encode, decode, Header, Algorithm, Validation};

use news_post_handlers::*;
use models::*;
use database::USER_COLLECTION;

const SECRET: &str = "fuqufuqwufqwufqwufuphqeffsD";

macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ($e:expr) => {$e.lock().unwrap()}
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            },
            None => return Ok(Response::with(status::InternalServerError))
        }
    }
}

pub struct Handlers {
    pub news_post_handler: NewsPostHandler,
    pub news_post_feed_handler: NewsPostFeedHandler,
    pub news_post_post_handler: NewsPostPostHandler,
    pub user_created_handler: UserCreateHandler,
    pub login_request_handler: LoginRequestHandler,
}

impl Handlers {
    pub fn new(database: Database) -> Handlers {
        let db = Arc::new(Mutex::new(database));
        Handlers {
            news_post_handler: NewsPostHandler::new(db.clone()),
            news_post_post_handler: NewsPostPostHandler::new(db.clone()),
            news_post_feed_handler: NewsPostFeedHandler::new(db.clone()),
            user_created_handler: UserCreateHandler::new(db.clone()),
            login_request_handler: LoginRequestHandler::new(db.clone()),
        }
    }
}

pub struct UserCreateHandler {
    database: Arc<Mutex<Database>>
}

impl UserCreateHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> UserCreateHandler {
        UserCreateHandler { database }
    }
}

impl Handler for UserCreateHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let user: PreUser = try_handler!(json::decode(&payload), status::BadRequest);

        let opt = lock!(self.database).find_document_with_username::<User>(USER_COLLECTION, &user.username); // do not do this in the if let, or there will be deadlock

        if let Some(_user) = opt {
            Ok(Response::with((status::Conflict, "That username is already in use")))
        } else { // the user was not found, thus the username is available
            let our_guy = User { 
                username: user.username,
                hashword: user.hashword,
                admin: false,
                date_created: user.date_created,
            };
            lock!(self.database).add_user(our_guy);
            Ok(Response::with((status::Created, payload)))
        }
    }
}

pub struct LoginRequestHandler {
    database: Arc<Mutex<Database>>,
}

impl LoginRequestHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> LoginRequestHandler {
        LoginRequestHandler { database }
    }
}

impl Handler for LoginRequestHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let login_req_data: LoginRequestData = try_handler!(json::decode(&payload), status::BadRequest);

        let opt: Option<PreUser> = lock!(self.database).find_document_with_username::<PreUser>(USER_COLLECTION, &login_req_data.username);
        
        if let Some(user) = opt {
            let hashword = &login_req_data.hashword;
            if user.hashword.eq(hashword) {
                let token = encode(&Header::default(), &login_req_data, SECRET.as_ref()).unwrap();

                let data = json!({"token":token});

                Ok(Response::with((status::Ok, data.to_string())))
            } else {
                Ok(Response::with((status::Forbidden, "The password provided was incorrect")))
            }
        } else {
            Ok(Response::with((status::NotFound, "The username provided was invalid")))
        }
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}

pub struct CorsAfterMiddleWare;

impl AfterMiddleware for CorsAfterMiddleWare {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(AccessControlAllowOrigin::Any);
        res.headers.set(AccessControlAllowCredentials);
        res.headers.set(AccessControlAllowHeaders(vec![
            UniCase("Content-Type".to_owned()),
            UniCase("Authorization".to_owned()),
            UniCase("X-Requested-With".to_owned()),
        ]));
        res.headers.set(AccessControlAllowMethods(vec![
            Method::Get,
            Method::Post,
        ]));
        Ok(res)
    }
}