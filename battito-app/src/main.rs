mod error;
mod line;

use crate::error::BattitoError;
use battito_lib::pattern::pattern::Pattern;
use battito_lib::pattern::{transform, OutputFormat};
use nannou_osc as osc;
use nannou_osc::rosc::OscMessage;
use nannou_osc::rosc::OscType;
use nannou_osc::{Connected, Sender};
use osc::Receiver;
use std::io;
use structopt::StructOpt;

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

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    subdivision: u32,
    #[structopt(short, long)]
    osc: Option<bool>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let config = Config {
        host: "127.0.0.1".to_string(),
        sender_port: 1234,
        receiver_port: 1235,
    };
    loop {
        match &opt.osc {
            Some(true) => {
                let sender = config.sender();
                let receiver = config.receiver();
                match process(&receiver, &sender, &opt) {
                    Ok(sent_packet) => println!("{:?}", sent_packet),
                    Err(error) => println!("{:?}", error),
                }
            }
            _ => {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                match process_stdin(buffer, &opt) {
                    Ok(sent_packet) => println!("{:?}", sent_packet),
                    Err(error) => println!("{:?}", error),
                }
            }
        }
    }
}

fn process(receiver: &Receiver, sender: &Sender<Connected>, opt: &Opt) -> Result<OscMessage, BattitoError> {
    let (packet, _) = receiver.recv()?;
    let (input_pattern, osc_address) = parse_osc(packet)?;
    let pattern = transform(&input_pattern, Some(opt.subdivision))?;
    let steps = pattern.format_steps(OutputFormat::Max);
    let osc_message = to_osc_message(osc_address, pattern, steps);
    let _ = sender.send(osc_message.clone()).map_err(BattitoError::from)?;

    Ok(osc_message)
}

fn process_stdin(input: String, opt: &Opt) -> Result<String, BattitoError> {
    let pattern = transform(&input, Some(opt.subdivision))?;
    let steps = pattern.format_steps(OutputFormat::Max);
    Ok(steps)
}

fn parse_osc(packet: osc::Packet) -> Result<(String, String), BattitoError> {
    let (osc_address, args) = match packet {
        osc::Packet::Message(OscMessage { addr, args: Some(args) }) => Ok((addr, args)),
        osc::Packet::Bundle(b) => match &b.content[..] {
            [osc::rosc::OscPacket::Message(OscMessage { addr, args: Some(args) })] => Ok((addr.clone(), args.clone())),
            _ => Err(BattitoError::OSCPacketError),
        },
        _ => Err(BattitoError::OSCPacketError),
    }?;
    let input_pattern = match &args[..] {
        [OscType::String(s)] => Ok(s.clone()),
        _ => Err(BattitoError::OSCPacketError),
    }?;

    Ok((input_pattern, osc_address))
}

fn to_osc_message(address: String, pattern: Pattern, steps: String) -> OscMessage {
    OscMessage {
        addr: address,
        args: Some(vec![
            OscType::Int(pattern.length as i32),
            OscType::Int(pattern.subdivision as i32),
            OscType::String(steps),
        ]),
    }
}
