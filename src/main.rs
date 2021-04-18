mod error;
mod max;
pub mod parser;
mod primitives;
mod sequence;
mod measure;
mod parsed_measure;
mod utils;
mod parsed;

use nannou_osc as osc;

use crate::error::Error;
use crate::parser::parse;
use nannou_osc::{Connected, Sender};
use std::io;

pub(crate) const VELOCITY_DEFAULT: u32 = 100;
pub(crate) const DURATION_DEFAULT: u32 = 100;

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
        match interpret(&sender) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn interpret(sender: &Sender<Connected>) -> Result<usize, Error> {
    let mut input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut input)?;
    let parsed_sequence = parse(&*input)?;
    // println!("{:?}", parsed_sequence);
    let sequence = parsed_sequence.to_sequence();
    let max_message = sequence.to_max_message();
    let packet = max_message.to_osc_message()?;
    println!("{}", packet.addr);

    sender.send(packet).map_err(Error::from)
}
