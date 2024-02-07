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

                let opcion = buffer[0] as i8;

                if opcion==-1{
                    terminado=true;
                }else{
                    let fila = buffer[1] as u8 as usize;
                    let columna = buffer[2] as u8 as usize;

                    if opcion==0{
                        let casillasDescubiertas=tablero.descubrir_casilla(fila, columna);

                        let numEnvios=[casillasDescubiertas.len() as u8];
                        stream.write(&numEnvios);

                        for infoCasilla in casillasDescubiertas{
                            let fila:u8;
                            let columna:u8;
                            let valor:i8;
                            (fila, columna, valor)=infoCasilla;
                            stream.write(&[fila]);
                            stream.write(&[columna]);
                            let valor: u8 = if valor>=0 {valor as u8} else {255};
                            stream.write(&[valor]);
                        }

                        #[cfg(debug_assertions)]
                        println!("{}", tablero)
                    }else{
                        //TODO poner bandera
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
