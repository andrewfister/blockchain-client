extern crate argparse;
extern crate zmq;
extern crate yaml_rust;

mod command_socket;

use argparse::{ArgumentParser, StoreTrue};
use command_socket::CommandSocket;
use yaml_rust::yaml;
use std::io::Read;
use std::fs::File;

struct Options {
    status: bool,
    version: bool,
    version_value: String,
    server_location: String,
}

fn main() {
    let mut options = Options {
        status: false,
        version: false,
        version_value: String::new(),
        server_location: String::new(),
    };

    parse_options(&mut options);

    let mut config_file = File::open("config.yml").unwrap();
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).unwrap();

    let config = yaml::YamlLoader::load_from_str(&config_str).unwrap();

    for config_item in &config {
        match config_item {
            &yaml::Yaml::Hash(ref hash) => {
                for (key, item_value) in hash {
                    let key_text = key.as_str().unwrap();
                    let item_text = String::from(item_value.as_str().unwrap());

                    if key_text.eq("server_location") {
                        options.server_location = item_text;
                    } else if key_text.eq("version") {
                        options.version_value = item_text;
                    }
                }
            },
            _ => {
                println!("Unexpected item in config file!");
            },
        }
    }

    if options.status {
        get_status();
    } else if options.version {
        println!("{}", options.version_value);
    }
}

fn parse_options(options: &mut Options) {
    let mut parser = ArgumentParser::new();

    parser.set_description("Block Chain Command Line");
    parser.refer(&mut options.status).add_option(&["-s", "--status"], StoreTrue,
                                         "Display Status of Block Chain Server");
    parser.refer(&mut options.version).add_option(&["-v", "--version"], StoreTrue,
                                                 "Display Version of Block Chain Server");
    parser.parse_args_or_exit();
}

fn get_status() {
    let ctx = zmq::Context::new();
    let command_socket = CommandSocket {
        ctx: ctx,
        server_location: String::from("/home/andrew/Projects/blockchain-server"),
    };

    command_socket.send_command(String::from("status"));
}