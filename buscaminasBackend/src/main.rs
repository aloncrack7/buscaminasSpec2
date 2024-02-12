use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::seq::index;

mod buscaminas;

struct AppState{
    tableros: Mutex<HashMap<String, buscaminas::Tablero>>
}

fn obtenerCliente(req: &HttpRequest) -> String{
    req.connection_info().peer_addr().unwrap_or_else(|| "unknown").to_owned()
}

async fn medio(data: web::Data<Arc<AppState>>, req: HttpRequest) -> impl Responder {
    println!("Medio");
    let cliente=obtenerCliente(&req);
    let mut tableros = data.tableros.lock().unwrap();

    if !tableros.contains_key(&cliente){
        tableros.insert(cliente.clone(), buscaminas::Tablero::new(16, 16, 40));
    }
    let mut tablero = tableros.get_mut(&cliente).unwrap();

    let mut tablero=buscaminas::Tablero::new(16, 16, 40);

    format!("Medio, {}", cliente)
}

async fn dificil(data: web::Data<Arc<AppState>>, req: HttpRequest) -> impl Responder{
    println!("Dificil");
    let cliente=obtenerCliente(&req);
    let mut tableros = data.tableros.lock().unwrap();

    if !tableros.contains_key(&cliente){
        tableros.insert(cliente.clone(), buscaminas::Tablero::new(16, 16, 40));
    }
    let mut tablero = tableros.get_mut(&cliente).unwrap();

    let mut tablero=buscaminas::Tablero::new(16, 16, 40);

    format!("Dificil, {}", cliente)
}

async fn ia(data: web::Data<Arc<AppState>>, req: HttpRequest) -> impl Responder{
    println!("IA");
    let cliente=obtenerCliente(&req);
    let mut tableros = data.tableros.lock().unwrap();

    if !tableros.contains_key(&cliente){
        tableros.insert(cliente.clone(), buscaminas::Tablero::new(16, 16, 40));
    }
    let mut tablero = tableros.get_mut(&cliente).unwrap();

    let mut tablero=buscaminas::Tablero::new(16, 16, 40);

    format!("IA, {}", cliente)
}
 
async fn jugar(data: web::Data<Arc<AppState>>, req: HttpRequest) -> impl Responder {
    println!("Jugar");

    let cliente = obtenerCliente(&req);
    let mut tableros = data.tableros.lock().unwrap();

    if !tableros.contains_key(&cliente) {
        return HttpResponse::PreconditionFailed().body("Error: El cliente no tiene ningun tablero inicilizado");
    }
    let mut tablero = tableros.get_mut(&cliente).unwrap();

    if let Some(opcion) = web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .ok()
        .map(|query| query.get("opcion").and_then(|c| c.parse::<i32>().ok())) {
        match opcion {
            Some(-1) => {
                // salir
                let response = "Saliendo...";
                return HttpResponse::Ok().body(response);
            }
            Some(0) => {
                if let (Some(fila), Some(columna)) = (
                    web::Query::<HashMap<String, String>>::from_query(req.query_string())
                        .ok()
                        .map(|query| query.get("fila").and_then(|f| f.parse::<usize>().ok())),
                    web::Query::<HashMap<String, String>>::from_query(req.query_string())
                        .ok()
                        .map(|query| query.get("columna").and_then(|c| c.parse::<usize>().ok())),
                ) {
                    let casillas_descubiertas = tablero.descubrir_casilla(fila.unwrap(), columna.unwrap());

                    let response = format!(
                        "Descubriendo... Casillas descubiertas: {:?}",
                        casillas_descubiertas
                    );

                    return HttpResponse::Ok().body(response);
                } else {
                    return HttpResponse::PreconditionFailed().body("Error: Missing 'fila' or 'columna' parameter.");
                }
            }
            Some(1) => {
                // poner bandera
                let response = "Poniendo bandera...";
                return HttpResponse::Ok().body(response);
            }
            _ => {
                return HttpResponse::PreconditionFailed().body("Error: Invalid option.");
            }
        }
    } else {
        return HttpResponse::PreconditionFailed().body("Error: Invalid option.");
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Hello, buscaminas!");

    let app_state = Arc::new(AppState{
        tableros: Mutex::new(HashMap::new())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(web::resource("/medio").to(medio))
            .service(web::resource("/dificil").to(dificil))
            .service(web::resource("/ia").to(ia))
            .service(web::resource("/jugar").to(jugar))
    }).bind(("127.0.0.1", 7070))?.run().await
}
