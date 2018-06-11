use std::sync::{Arc, Mutex};
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
use lettre::smtp::authentication::Credentials;

use models::User;
use models::Email;
use database::USER_COLLECTION;
use database::EMAIL_REQUEST_COLLECTION;
use database::PUBLIC_IP_AND_HOST;

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

fn grab_email_credentials() -> (String, String)
{
    let f = File::open("email_credentials.txt").expect("Failed to open email_credentials.txt");
    let file = BufReader::new(&f);
    let mut lines_itr = file.lines().map(|l| l.unwrap());
    (lines_itr.next().expect("Failed to find email in email_credentials.txt"), 
        lines_itr.next().expect("Failed to find password in email_credentials.txt"))
}

/// Is called by the actual Minecraft server to send an email to a user so that they may register.
impl Handler for UserCreateHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let email: Email = try_handler!(json::decode(&payload), status::BadRequest);

        let opt = lock!(self.database).get_all_documents::<User>(USER_COLLECTION, Some(doc!{ "uuid" => &email.uuid }), None); // do not do this in the if let, or there will be deadlock
        let opt2 = lock!(self.database).get_all_documents::<User>(EMAIL_REQUEST_COLLECTION, Some(doc!{ "uuid" => &email.uuid }), None);

        if opt.len() > 0 {
            Ok(Response::with((status::Conflict, "You are already registered!")))
        } else if opt2.len() > 0 {
            Ok(Response::with((status::Conflict, "This account has already attempted to register with a different email. If you think this is an error, please contact support.")))
        } else { // the user was not found, thus the username is available

            let (client_email, client_password) = grab_email_credentials();

            lock!(self.database).add_email_request(email.clone());
            // this function will need to be changed
            let link = format!("{}/register/{}", PUBLIC_IP_AND_HOST, email.linkUuid);
            let content = format!("Navigate to this link to complete the registration process: <a href='{}'>{}</a>", 
                &link, &link);

            let builder = match EmailBuilder::new()
                .to(email.email)
                .from(client_email.clone())
                .subject("Medieval Lords Registration")
                .html(content)
                .build() {
                    Ok(s) => s,
                    Err(e) => panic!("{:?}", e),
            };

            let c:Credentials = Credentials::new(client_email, client_password);    
            let mut transport:SmtpTransport = SmtpTransport::simple_builder("smtp.gmail.com")
                .expect("Failed to create transport")
                .credentials(c).build();

            let result = transport.send(&builder);

            if result.is_ok() {
                Ok(Response::with((status::Created, payload)))
            } else {
                println!("Mailer send result not okay: {:?}", result);
                Ok(Response::with(status::BadRequest))
            }

        }
    }
}

pub struct GetUserRegisterFormHandler {
    database: Arc<Mutex<Database>>,
}

impl GetUserRegisterFormHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> GetUserRegisterFormHandler {
        GetUserRegisterFormHandler { database }
    }
}

/// Validate the url and pass information to the front end, such as email, username, etc. Prepare to prompt user for password.
impl Handler for GetUserRegisterFormHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let link_uuid = get_http_param!(req, "linkUuid");

        let opt: Option<Email> = lock!(self.database).find_one_document::<Email>(EMAIL_REQUEST_COLLECTION, Some(doc!{"linkUuid" : link_uuid}), None);

        if let Some(email) = opt {
            let payload = try_handler!(json::encode(&email), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with((status::Conflict, "Failed to validate url.")))
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

/// This post request will check the url for the proper link uuid and then attempt to register the user
/// with the information provided from the front end.
impl Handler for UserRegisterHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let link_uuid = get_http_param!(req, "linkUuid");

        let opt = lock!(self.database).get_all_documents::<Email>(EMAIL_REQUEST_COLLECTION, Some(doc!{ "linkUuid" => link_uuid }), None);

        if opt.len() < 1 {
            return Ok(Response::with((status::Conflict, "Your link was invalid or an error has occured.")));
        }

        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let user: User;
        if let Ok(user_data) = serde_json::from_str::<User>(&payload) {
            user = user_data;
        } else {
            return Ok(Response::with((status::BadRequest, "Failed to parse user data!")))
        };

        lock!(self.database).add_user(user);
        lock!(self.database).erase_from_collection_where(EMAIL_REQUEST_COLLECTION, doc!{ "linkUuid" => link_uuid });

        Ok(Response::with((status::Ok, "{}")))
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

pub struct SearchUsersHandler {
    database: Arc<Mutex<Database>>,
}

impl SearchUsersHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> SearchUsersHandler {
        SearchUsersHandler { database }
    }
}

impl Handler for SearchUsersHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let search_term = String::from(get_http_param!(req, "term")).to_lowercase();

        let users = lock!(self.database).get_all_documents::<User>(USER_COLLECTION, None, None);

        let mut results = Vec::new();
        for u in &users // O(n)
        {
            if u.username.to_lowercase().contains(&search_term)
            {
                results.push(u);
            }
        }

        if results.len() == 0
        {
            return Ok(Response::with((status::Conflict, "No search results")));
        }

        let data = try_handler!(json::encode(&results), status::BadRequest);
        Ok(Response::with((status::Ok, data)))
    }
}