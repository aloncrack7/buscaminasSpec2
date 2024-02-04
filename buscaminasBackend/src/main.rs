use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 1];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                match buffer[0] {
                    b'0'=>{
                        break;
                    }
                    b'1'=>{
                        medio(&mut stream);
                    }
                    b'2'=>{
                        dificil(&mut stream);
                    }
                    b'3'=>{
                        ia(&mut stream);
                    }
                    _ =>{
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}

fn medio(stream: &mut TcpStream){
    println!("Medio");
}

fn dificil(stream: &mut TcpStream){
    println!("Dificil");
}

fn ia(stream: &mut TcpStream){
    println!("IA");
}

fn main() {
    println!("Hello, buscaminas!");

    let listener=TcpListener::bind("127.0.0.1:7070").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
