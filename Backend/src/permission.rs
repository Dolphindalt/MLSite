use std::sync::{Arc, Mutex};
use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use iron::{status, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use database::Database;
use std::error::Error;
use jwt::{encode, Header};
use serde_json;
use serde_json::Value;
use router::Router;
use bson;

/// Current permission names
/// 
/// DeletePost
/// DeleteThread
/// EditPosts
/// PinThread
/// LockThread
/// 

/// Only groups listed in permissions.txt should be passed to this function
fn grab_permission_credentials(group: &str) -> Option<HashMap<String, u8>> {
    let group_string = String::from(group);
    let mut permission: HashMap<String, u8> = HashMap::new();
    let f = File::open("permissions.txt").expect("Failed to open permissions folder");
    let file = BufReader::new(&f);
    let mut lines_itr = file.lines().map(|l| l.unwrap());
    loop {
        if let Some(line) = lines_itr.next() {
            if line == group_string {
                loop {
                    if let Some(line_loop) = lines_itr.next() {
                        if line_loop == group_string {
                            break;
                        } else {
                            let split: Vec<&str> = line_loop.split(":").collect();
                            let mut key = split[0].to_string();
                            key.make_ascii_lowercase();
                            permission.insert(key, split[1].parse::<u8>().unwrap());
                        }
                    } else {
                        return None;
                    }
                }
                break;
            }
        } else {
            return None;
        }
    }
    Some(permission)
}