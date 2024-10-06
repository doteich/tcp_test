use core::time;
use std::io::{self, Error, Read, Write};
use std::net::TcpStream;
use std::thread::{self, sleep};

pub fn create_client(host: String, port: u16) -> Result<(), Error> {
    let addr = format!("{}:{}", host, port);

    let mut stream = TcpStream::connect(addr)?;

    println!("Successfully connected to {}", format!("{}:{}", host, port));

    let mut inbound_stream = stream.try_clone().unwrap();
    let mut outbound_stream = stream.try_clone().unwrap();

    thread::spawn(move || write(&mut outbound_stream));
    thread::spawn(move || read(&mut inbound_stream));
    keep_alive(&mut stream);

    Ok(())
}

fn write(stream: &mut TcpStream) {
    loop {
        let mut input = String::new();
        print!("enter tcp message (confirm with[enter]) ==>:");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("error with received input: {}", e);
                continue;
            }
        };

        let msg = input.replace("\r\n", "");

        match stream.write(msg.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("error writing to tcp stream: {}", e);
                continue;
            }
        };
        sleep(time::Duration::from_secs(1));
    }
}

fn read(stream: &mut TcpStream) {
    loop {
        let mut buf = vec![0; 1024];

        let bytes = match stream.read(&mut buf) {
            Ok(m) => m,
            Err(e) => {
                println!("error reading from tcp stream: {}", e);
                continue;
            }
        };

        let msg = String::from_utf8_lossy(&buf[..bytes]);
       // let str = msg.into_owned().as_str().replace("\r\n", "");

        println!("==> received new message: {}", msg)
    }
}

fn keep_alive(stream: &mut TcpStream) {
    loop {
        match stream.write("".as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("error writing keepalive to tcp stream: {}", e);
                continue;
            }
        };

        sleep(time::Duration::from_secs(10));
    }
}
