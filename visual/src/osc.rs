use rosc::{OscMessage, OscPacket, OscType};
use std::{
    collections::HashMap,
    net::{ToSocketAddrs, UdpSocket},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::model::Model;

type OscProps = HashMap<String, OscType>;

pub struct Osc {
    pub receiver: Receiver<OscPacket>,
}

impl Osc {
    pub fn listen<A>(address: A) -> Self
    where
        A: ToSocketAddrs + 'static + Send,
    {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            Osc::osc_receive_thread(sender, address);
        });

        Osc { receiver }
    }

    fn osc_receive_thread(osc_sender: Sender<OscPacket>, address: impl ToSocketAddrs) {
        let socket = UdpSocket::bind(address).unwrap();
        println!("Listening for OSC on {}", socket.local_addr().unwrap());

        loop {
            let mut buf = [0u8; 65507];
            match socket.recv_from(&mut buf) {
                Ok((_size, _addr)) => {
                    let (_, packet) = rosc::decoder::decode_udp(&buf).unwrap();
                    osc_sender.send(packet).unwrap();
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }

    pub fn parse_properties(args: &[OscType]) -> OscProps {
        let mut properties = HashMap::new();
        let mut key = String::new();

        for (index, arg) in args.iter().enumerate() {
            if index % 2 == 0 {
                if let OscType::String(s) = arg {
                    key = s.clone();
                } else {
                    eprintln!(
                        "Warning: Expected String for property key at index {}, but got {:?}",
                        index, arg
                    );
                    key = String::new();
                }
            } else if !key.is_empty() {
                properties.insert(key.clone(), arg.clone());
                key = String::new();
            } else {
                eprintln!(
                    "Warning: Value found at index {} without a preceding key: {:?}",
                    index, arg
                );
            }
        }

        if !key.is_empty() {
            eprintln!("Warning: Key found at the end without a value: {}", key);
        }

        properties
    }
}

impl Model {
    pub fn handle_osc_freq(&mut self, msg: &OscMessage) {
        if let OscType::Blob(a) = &msg.args[0] {
            self.freqscope
                .iter_mut()
                .zip(a.iter().map(|&v| v as i32 - 160))
                .for_each(|(dst, src)| *dst = src);
        }
    }

    pub fn handle_osc_dirt(&mut self, msg: &OscMessage) {
        let osc_properties = Osc::parse_properties(&msg.args);

        if let Some(OscType::String(t)) = osc_properties.get("s") {
            if let Some(scene) = self.scenes.0.get_mut(t) {
                scene.invoke();
            }
        }
    }

    pub fn handle_osc(&mut self) {
        if let Ok(packet) = self.osc.receiver.try_recv() {
            match packet {
                OscPacket::Bundle(bundle) => {
                    if let OscPacket::Message(msg) = &bundle.content[0] {
                        if msg.addr == "/dirt/play" {
                            self.handle_osc_dirt(msg);
                        }
                    }
                }

                OscPacket::Message(msg) => {
                    if msg.addr == "/freq" {
                        self.handle_osc_freq(&msg);
                    }
                }
            };
        }
    }
}
