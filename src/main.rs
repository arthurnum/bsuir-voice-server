use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

mod command;
mod voice_list_item;

use crate::command::*;
use crate::voice_list_item::VoiceListItem;

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    println!("Server start.");

    println!("Read store.");
    let mut records_list: Vec<VoiceListItem> = Vec::with_capacity(128);
    for entry in fs::read_dir("store").unwrap() {
        let file = entry.unwrap();
        let filename = file.file_name().into_string().unwrap();
        match filename.find(".voice") {
            None => {},
            Some(ext) => {
                println!("\t{filename:}");
                let ts = str::parse::<u64>(&filename[0..ext]).unwrap();
                records_list.push(
                    VoiceListItem { timestamp: ts }
                );
            }
        }
    }
    records_list.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let addr: String = String::from("127.0.0.1:33666");
    let listener = TcpListener::bind(addr).unwrap();

    'main: loop {
        match listener.accept() {
            Err(msg) => println!("Accept error: {:}", msg),

            Ok((mut connection, addr)) => {
                println!("New client: {addr:}");

                let mut cmd_buf = vec![0u8; COMMAND_BIN_SIZE];
                connection.read_exact(&mut cmd_buf).unwrap();
                let cmd = bincode::deserialize::<Command>(&cmd_buf).unwrap();
                println!("{:?}", cmd);

                match cmd.id {
                    CMD_INDEX => {
                        println!("Index request.");
                        let data = bincode::serialize(&records_list).unwrap();
                        connection.write_all(&data).unwrap();
                        connection.flush().unwrap();
                    },

                    CMD_POST_RECORD => {
                        println!("Post record request.");
                        println!("Income length: {:}", cmd.data_len);
                        let mut record_buf: Vec<u8> = Vec::new();

                        connection.read_to_end(&mut record_buf).unwrap();
                        println!("Done read. {:}", record_buf.len());

                        let x: u64 = bincode::deserialize(&record_buf).unwrap();
                        println!("Test record deserialised len. {:}", x);

                        let su = bincode::deserialize::<Vec<i16>>(&record_buf).unwrap();
                        println!("Result record len {:}", su.len());
                        println!();

                        let mut file = File::create(format!("store/{}.voice", cmd.timestamp)).unwrap();
                        file.write_all(&record_buf).unwrap();
                    },

                    CMD_GET_RECORD => {
                        println!("Get record request.");

                        let mut file = File::open(format!("store/{}.voice", cmd.timestamp)).unwrap();
                        let mut data = Vec::new();
                        file.read_to_end(&mut data).unwrap();
                        println!("Data len: {}", data.len());

                        connection.write_all(&data).unwrap();
                        connection.flush().unwrap();
                    },

                    _ => {}
                }
            }
        }
    }
}
