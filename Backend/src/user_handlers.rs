use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use database::Database;
use std::error::Error;
use jwt::{encode, Header};
use serde_json;
use serde_json::Value;
use router::Router;

use models::User;
use models::PreUser;
use database::USER_COLLECTION;

pub const SECRET: &str = "fuqufuqwufqwufqwufuphqeffsD";

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

        let login_req_data: Value = try_handler!(serde_json::from_str(&payload), status::BadRequest);

        let username: String;
        if let Some(shaky_username) = login_req_data["username"].as_str() {
            username = String::from(shaky_username);
        } else {
            return Ok(Response::with(status::BadRequest));
        };

        let hashword: String;
        if let Some(shaky_hashword) = login_req_data["password"].as_str() {
            hashword = String::from(shaky_hashword);
        } else {
            return Ok(Response::with(status::BadRequest));
        }

        let opt: Option<User> = lock!(self.database).find_document_with_username::<User>(USER_COLLECTION, &username);
        
        if let Some(user) = opt {
            if user.hashword.eq(&hashword) {
                
                let token: String = encode(&Header::default(), &user, SECRET.as_ref()).unwrap();
                let token_str = format!("{}\"token\":{:?}{}", "{", &token, "}");

                Ok(Response::with((status::Ok, token_str)))
            } else {
                Ok(Response::with(status::Forbidden))
            }
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}

pub struct GetSingleUserHandler {
    database: Arc<Mutex<Database>>
}

impl GetSingleUserHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> GetSingleUserHandler {
        GetSingleUserHandler { database }
    }
}

impl Handler for GetSingleUserHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref username = get_http_param!(req, "username");

        let locked = lock!(self.database);
        if let Some(userdata) = locked.find_document_with_username::<User>(USER_COLLECTION, &username) {
            let payload = try_handler!(json::encode(&userdata), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}