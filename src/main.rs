use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::time::{SystemTime, UNIX_EPOCH};

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

                let mut xbuf: Vec<u8> = vec![0; 8];

                connection.read_exact(xbuf.as_mut_slice()).unwrap();

                if xbuf[0] == 102 {
                    println!("Get record request.");

                    //1699386157.voice
                    let mut file = File::open(format!("store/{}.voice", 1699386157)).unwrap();
                    let mut data = Vec::new();
                    file.read_to_end(&mut data).unwrap();
                    println!("Data len: {}", data.len());

                    connection.write_all(&data).unwrap();
                    connection.flush().unwrap();

                    continue;
                }

                let length: usize = bincode::deserialize(&xbuf).unwrap();

                println!("Income length: {length:}");

                let mut record_buf: Vec<u8> = Vec::new();
                let mut l: usize = 0;
                let mut read = true;

                let mut reader = BufReader::new(connection);

                let mut buf = vec![0u8; 1024];
                while read {
                    if (length - l) < 1024 {
                        buf.resize(length - l, 0);
                        read = false;
                    } else {
                        buf.resize(1024, 0);
                    }
                    let read_size = buf.len();
                    reader.read_exact(buf.as_mut_slice()).unwrap();
                    record_buf.append(&mut buf);
                    l += read_size;

                    println!("\t total reading {:}", l);
                }

                println!("Done read. {:}", record_buf.len());

                let x: u64 = bincode::deserialize(&record_buf).unwrap();
                println!("Test serde len. {:}", x);

                let su = bincode::deserialize::<Vec<i16>>(&record_buf).unwrap();
                println!("Result len {:}", su.len());

                let mut file = File::create(format!("store/{}.voice", get_timestamp())).unwrap();
                file.write_all(&record_buf);
            }
        }
    }
}
