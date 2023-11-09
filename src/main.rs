use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

mod command;

use command::Command;

use crate::command::{CMD_POST_RECORD, CMD_GET_RECORD, COMMAND_BIN_SIZE};

fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    println!("Server start.");

    let listener = TcpListener::bind("127.0.0.1:33666").unwrap();

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

                        let mut file = File::create(format!("store/{}.voice", get_timestamp())).unwrap();
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
