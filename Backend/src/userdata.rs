use std::str;
use iron::{Request};
use jwt::{decode, Validation};

use models::User;
use login_handlers::SECRET;

pub fn extract_token_data_from_header(req: &mut Request) -> Option<User> {
    if let Some(auth_token) = req.headers.get_raw("Authorization") {
        if auth_token.len() == 2 {

            let raw_token = match str::from_utf8(&auth_token[1]) {
                Ok(t) => t,
                Err(_e) => return None,
            };

            let token = match decode::<User>(&raw_token, SECRET.as_ref(), &Validation::default()) {
                Ok(t) => t,
                Err(_e) => return None,
            };

            let user: User = token.claims;
            Some(user)
        } else {
            None
        }
    } else {
        None
    }
}