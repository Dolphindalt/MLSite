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
use lettre_email::EmailBuilder;
use lettre::{EmailTransport, SmtpTransport};

use models::User;
use models::Email;
use database::USER_COLLECTION;

pub const SECRET: &str = "fuqufuqwufqwufqwufuphqeffsD";
const STAFF_RANKS: [&str; 6] = [ "Owner", "Developer", "Builder", "Admin", "SMod", "Mod" ];

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

        let email: Email = try_handler!(json::decode(&payload), status::BadRequest);

        let opt = lock!(self.database).get_all_documents::<User>(USER_COLLECTION, Some(doc!{ "uuid" => &email.uuid }), None); // do not do this in the if let, or there will be deadlock

        if opt.len() > 0 {
            Ok(Response::with((status::Conflict, "You are already registered")))
        } else { // the user was not found, thus the username is available
            lock!(self.database).add_email_request(email.clone());
            // this function will need to be changed
            let content = format!("Navigate to this link to complete the registration process: <a>localhost:4200/register/{}</a>", email.linkUuid);

            let builder = match EmailBuilder::new()
                .to(email.email)
                .from("noreply@go.playminecraft.org")
                .subject("Medieval Lords Registration")
                .html(content)
                .build() {
                    Ok(s) => s,
                    Err(e) => panic!("{:?}", e),
                };

            let mut mailer = SmtpTransport::builder_unencrypted_localhost().unwrap().build();
            let result = mailer.send(&builder);

            if result.is_ok() {
                Ok(Response::with((status::Created, payload)))
            } else {
                println!("Mailer send result not okay: {:?}", result);
                Ok(Response::with(status::BadRequest))
            }

        }
    }
}

pub struct UserRegisterHandler {
    database: Arc<Mutex<Database>>,
}

impl UserRegisterHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> UserRegisterHandler {
        UserRegisterHandler{ database }
    }
}

impl Handler for UserRegisterHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let post_id = get_http_param!(req, "uuid");
        
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let user: User;
        if let Ok(user_data) = serde_json::from_str::<User>(&payload) {
            user = user_data;
        } else {
            return Ok(Response::with(status::BadRequest))
        };

        lock!(self.database).add_user(user);

        Ok(Response::with(status::Ok))
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

        let uuid: String;
        if let Some(shaky_uuid) = login_req_data["uuid"].as_str() {
            uuid = String::from(shaky_uuid);
        } else {
            return Ok(Response::with(status::BadRequest));
        };

        let hashword: String;
        if let Some(shaky_hashword) = login_req_data["password"].as_str() {
            hashword = String::from(shaky_hashword);
        } else {
            return Ok(Response::with(status::BadRequest));
        }

        let opt: Option<User> = lock!(self.database).find_one_document::<User>(USER_COLLECTION, Some(doc!{"uuid" : uuid }), None);
        
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
        let uuid: &str = get_http_param!(req, "uuid");

        let locked = lock!(self.database);
        if let Some(userdata) = locked.find_one_document::<User>(USER_COLLECTION, Some(doc!{ "uuid" => uuid }), None) {
            let payload = try_handler!(json::encode(&userdata), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}

pub struct GetStaffUsersHandler {
    database: Arc<Mutex<Database>>,
}

impl GetStaffUsersHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> GetStaffUsersHandler {
        GetStaffUsersHandler { database }
    }
}

impl Handler for GetStaffUsersHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let locked = lock!(self.database);
        let mut data = Vec::new();
        for (_, elem) in STAFF_RANKS.iter_mut().enumerate() {
            let rank = String::from(elem.clone());
            &data.push(locked.get_all_documents::<User>(USER_COLLECTION, Some(doc!{ "rank" => rank }), None));
        }
        let payload = try_handler!(json::encode(&data));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct GetRegexUsersHandler {
    database: Arc<Mutex<Database>>,
}

impl GetRegexUsersHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> GetRegexUsersHandler {
        GetRegexUsersHandler { database }
    }
}

impl Handler for GetRegexUsersHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let search_data: Value = try_handler!(serde_json::from_str(&payload), status::BadRequest);

        let search_string: &str;
        if let Some(shaky_search) = search_data["data"].as_str() {
            search_string = shaky_search;
        } else {
            return Ok(Response::with(status::Ok));
        };

        let formated_search_string = format!("/{}/", search_string);

        let docs = lock!(self.database).get_all_documents::<User>(USER_COLLECTION, Some(doc!{ "username" => formated_search_string }), None);
        let data = try_handler!(json::encode(&docs), status::BadRequest);

        Ok(Response::with((status::Ok, data)))
    }
}