mod error;
mod line;

use crate::error::Error;
use battito_lib::pattern::pattern::Pattern;
use battito_lib::pattern::{transform, OutputFormat};
use nannou_osc as osc;
use nannou_osc::rosc::OscMessage;
use nannou_osc::rosc::OscType;
use nannou_osc::{Connected, Sender};

pub struct Config {
    host: String,
    sender_port: u16,
    receiver_port: u16,
}

impl Config {
    pub fn sender(&self) -> Sender<Connected> {
        osc::sender()
            .expect("Could not bind to default socket")
            .connect(format!("{}:{}", self.host, self.sender_port))
            .expect("Could not connect to socket at address")
    }

    pub fn receiver(&self) -> Receiver {
        osc::receiver(self.receiver_port).expect("Could not bind to default socket")
    }
}

use osc::Receiver;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    // #[structopt(short, long)]
    // pattern: String,
    #[structopt(short, long)]
    subdivision: u32,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let config = Config {
        host: "127.0.0.1".to_string(),
        sender_port: 1234,
        receiver_port: 1235,
    };
    let sender = config.sender();
    let receiver = config.receiver();
    loop {
        let (p, _) = receiver.recv().unwrap();

        println!("{:?}", p);
        let (address, arg) = match p {
            osc::Packet::Message(m) => (m.addr, m.args.unwrap()[0].clone()),
            osc::Packet::Bundle(b) => match b.content[0].clone() {
                osc::rosc::OscPacket::Message(m) => (m.addr, m.args.unwrap()[0].clone()),
                _ => panic!(),
            },
            _ => panic!(),
        };
        let input = match arg {
            OscType::String(s) => s,
            _ => panic!(),
        };
        let pattern = transform(&input, Some(opt.subdivision)).unwrap();
        let steps = pattern.format_steps(OutputFormat::Max);
        let osc_message = to_osc_message(address, pattern, steps).unwrap();
        println!("{:?}", osc_message);
        sender.send(osc_message).map_err(Error::from);
    }
}

fn to_osc_message(address: String, pattern: Pattern, steps: String) -> Result<OscMessage, Error> {
    Ok(OscMessage {
        addr: address,
        args: Some(vec![
            OscType::Int(pattern.length as i32),
            OscType::Int(pattern.subdivision as i32),
            OscType::String(steps),
        ]),
    })
}
