use lazy_static::lazy_static;
use mongodb::Client;


lazy_static! {
    static ref LAZY_CONNECTION: Client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
}

pub struct Connection;

pub trait IConnection {
    fn init(&self) -> &'static Client;
}

impl IConnection for Connection {
    fn init(&self) -> &'static Client {
        lazy_static::initialize(&LAZY_CONNECTION);
        &*LAZY_CONNECTION
    }
}
