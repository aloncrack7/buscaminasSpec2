use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

// use crate::buscaminas::Tablero;
mod buscaminas;

fn handle_client(mut stream: TcpStream) {
    let mut terminado=false;

    while !terminado {
        let mut buffer = [0; 1];
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    terminado=true;
                }

                let opcion=buffer[0] as i8;
                match opcion {
                    -1=>{
                        terminado=true;
                    }
                    1=>{
                        medio(&mut stream);
                    }
                    2=>{
                        dificil(&mut stream);
                    }
                    3=>{
                        ia();
                    }
                    _ =>{
                        terminado=true;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                terminado=true;
            }
        }
    }

    println!("Connection closed");
}

fn medio(stream: &mut TcpStream){
    println!("Medio");

    let mut tablero = 
        buscaminas::Tablero::new(16, 16, 40);

    jugar(stream, &mut tablero);
}

fn dificil(stream: &mut TcpStream){
    println!("Dificil");

    let mut tablero = 
        buscaminas::Tablero::new(30, 16, 99);

    jugar(stream, &mut tablero);
}

fn ia(){
    println!("IA");

    let mut tablero = 
        buscaminas::Tablero::new(30, 16, 99);
}

fn jugar(stream: &mut TcpStream, tablero: &mut buscaminas::Tablero){
    let mut terminado=false;

    while !terminado{
        let mut buffer = [0; 3];

        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    terminado=true;
                }

                let fila = buffer[0] as i8;
                if fila==-1{
                    terminado=true;
                }else{
                    let fila = fila as usize;

                    let columna = buffer[1] as i8 as usize;
                    let accion = buffer[2] as i8;

                    if accion==0{
                        tablero.descubrir_casilla(fila, columna);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                terminado=true;
            }
            
        }

        terminado=terminado || tablero.get_estado()==buscaminas::EstadoPartida::Ganada || 
            tablero.get_estado()==buscaminas::EstadoPartida::Perdida;
    }
}

fn main() {
    println!("Hello, buscaminas!");

    let listener=TcpListener::bind("127.0.0.1:7070").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
