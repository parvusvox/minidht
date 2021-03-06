#[allow(dead_code)]
#[allow(unused_variables)]
extern crate env_logger;

use std::io;
use std::env;

mod node;
mod key;
mod rpc;
mod routing;
mod constants;

use crate::key::*;
use crate::node::*;
use crate::routing::*;


use clap::{Arg, App};

fn interactive(bootstrap:Option<NodeInfo>) {
    let input = io::stdin();
    let handle = Node::start(String::from("test_net"),
        Key::random(),
        "127.0.0.1:0",
        bootstrap);
    let mut dummy_info = NodeInfo {
        net_id: String::from("test_net"),
        addr: String::from("asdfasdg"),
        id: Key::random(),
    };

    loop {
        let mut buffer = String::new();
        if input.read_line(&mut buffer).is_err() {
            break;
        }
        let args = buffer.trim_end().split(' ').collect::<Vec<_>>();
        match args[0].as_ref() {
            "p" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.ping(dummy_info.clone()));
            }
            "s" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.store(dummy_info.clone(), String::from(args[3]), String::from(args[4])));
            }
            "fn" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.find_node(dummy_info.clone(), Key::from(String::from(args[3]))));
            }
            "fv" => {
                dummy_info.addr = String::from(args[1]);
                dummy_info.id = Key::from(String::from(args[2]));
                println!("{:?}", handle.find_value(dummy_info.clone(), String::from(args[3])));
            }
            "ln" => {
                println!("{:?}", handle.lookup_nodes(Key::from(String::from(args[1]))));
            }
            "lv" => {
                println!("{:?}", handle.lookup_values(String::from(args[1])));
            }
            "put" => {
                println!("{:?}", handle.put(String::from(args[1]), String::from(args[2])));
            }
            "get" => {
                println!("{:?}", handle.get(String::from(args[1])));
            }
            _ => {
                println!("no match");
            }
        }
    }
}

fn main() {
    let matches = App::new("MiniDHT")
        .version("0.1.0")
        .author("Ian Kim <ian@ianmkim.com>")
        .about("A toy Kademlia DHT in Rust")
        .arg(Arg::new("bootstrap")
            .short('b')
            .long("bootstrap")
            .value_name("<IP>:<port>:<Node ID>")
            .about("Bootstraps instance from IP port and Node ID triple")
            .takes_value(true))
        .arg(Arg::new("bootstrap-file")
            .short('f')
            .long("bootstrap-file")
            .value_name("<filename>")
            .about("Bootstraps instance from file <not yet implemented>")
            .takes_value(true))
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .value_name("<verbosity>")
            .about("Configures verbosity of log output")
            .takes_value(true))
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .about("launches node in interactive mode"))
        .get_matches();

    env::set_var("RUST_LOG", "info");
    if matches.is_present("verbose") {
        match matches.value_of("verbose").unwrap(){
            "0" => {
                println!("Verbosity set to 0");
                env::set_var("RUST_LOG", "");
            }, "1" => {
                println!("Verbosity set to 1");
            }, "2" => {
                println!("Verbosity set to 2");
            }, _ => {
                println!("Verbosity set to 2");
            }
        }
    }
    env_logger::init();

    let mut triple = Vec::new();
    if matches.is_present("bootstrap"){
        triple = matches.value_of("bootstrap").unwrap().split(":").collect::<Vec<_>>();
    }
    let bootstrap = if triple.len() == 3 {
        Some(NodeInfo {
            id: Key::from(String::from(triple[2])),
            addr: String::from(triple[0].to_owned()+ ":" + triple[1]),
            net_id: String::from("test_net"),
        })
    } else { None};

    if matches.is_present("interactive") {
        interactive(bootstrap);
    } else {
        let _handle = Node::start(String::from("test_net"),
            Key::random(),
            "127.0.0.1:0",
            bootstrap);
        loop{}
    }
}

#[cfg(test)]
mod unittest;
