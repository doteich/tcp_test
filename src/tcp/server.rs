use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

pub fn create_mock_server() {
    let tcp_result = TcpListener::bind("127.0.0.1:8080");

    let server = match tcp_result {
        Ok(t) => t,
        Err(e) => {
            panic!("{e}")
        }
    };

    for stream in server.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        thread::spawn(move || handle_connection(stream));
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buf = vec![0; 1024];

        match stream.read(&mut buf) {
            Ok(msg_length) => {
                if msg_length == 0 {
                    // Connection closed by the client
                    println!("Connection closed.");
                    return;
                }
                stream.write(&buf[..msg_length]).unwrap();
            }
            Err(e) => {
                print!("{}", e);
                return;
            }
        }
    }
}
