use mongodb::{Client, ThreadedClient};

const HOSTNAME: &str = "localhost";
const PORT: u16 = 27017;

pub struct Connection {
    client: Client
}

impl Connection {
    pub fn new() -> Connection {
        Connection { client: Client::connect(HOSTNAME, PORT)
            .expect("Failed to connect to the mongo database") }
    }
}