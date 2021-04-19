mod error;

use crate::error::Error;
use battito_lib::interpreter::interpret;
use battito_lib::max::Payload;
use nannou_osc as osc;
use nannou_osc::rosc::OscMessage;
use nannou_osc::{Connected, Sender};
use std::io;

pub struct Config {
    host: String,
    port: i32,
}

impl Config {
    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn sender(&self) -> Sender<Connected> {
        osc::sender()
            .expect("Could not bind to default socket")
            .connect(&self.address())
            .expect("Could not connect to socket at address")
    }
}

fn main() {
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 1234,
    };
    let sender = config.sender();
    loop {
        match run(&sender) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn run(sender: &Sender<Connected>) -> Result<usize, Error> {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input)?;

    let payload = interpret(&input)?;
    let packet = to_osc_message(&payload)?;
    println!("{}", packet.addr);

    sender.send(packet).map_err(Error::from)
}

fn to_osc_message(payload: &Payload) -> Result<OscMessage, Error> {
    Ok(OscMessage {
        addr: serde_json::to_string(payload)?,
        args: None,
    })
}
