use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

mod buscaminas;

struct AppState{
    tableros: Mutex<HashMap<String, buscaminas::Tablero>>
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Opcion{
    Seleccionar=0,
    Bandera=1,
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

    //Gestionar IA

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

    if let Some(opcion) = 
        web::Query::<HashMap<String, String>>::from_query(req.query_string())
            .ok()
            .and_then(|query| query.get("opcion").cloned()){
        match opcion.as_str() {
            "seleccionar" => {
                if let (Some(fila), Some(columna)) = 
                    (obtener_parametro(&req, "fila"), obtener_parametro(&req, "columna")) {
                    let casillas_descubiertas = tablero.descubrir_casilla(fila, columna);
                    return HttpResponse::Ok().json(casillas_descubiertas);
                } else {
                    return HttpResponse::PreconditionFailed().body("Error: Missing 'fila' or 'columna' parameter.");
                }
            }
            "seleccionarVarios" => {
                if let (Some(fila), Some(columna)) = 
                    (obtener_parametro(&req, "fila"), obtener_parametro(&req, "columna")) {
                    let casillas_descubiertas = tablero.descubrirCasillas(fila, columna);
                    return HttpResponse::Ok().json(casillas_descubiertas);
                } else {
                    return HttpResponse::PreconditionFailed().body("Error: Missing 'fila' or 'columna' parameter.");
                }
            }
            "bandera" => {
                // poner bandera
                let response = "Poniendo bandera...";

                if let (Some(fila), Some(columna)) = (obtener_parametro(&req, "fila"), obtener_parametro(&req, "columna")) {
                    tablero.poner_bandera(fila, columna);

                    return HttpResponse::Ok().body(response);
                } else {
                    return HttpResponse::PreconditionFailed().body("Error: Missing 'fila' or 'columna' parameter.");
                }
            }
            _ => {
                return HttpResponse::PreconditionFailed().body("Error: Invalid option.");
            }
        }
    } else {
        return HttpResponse::PreconditionFailed().body("Error: Invalid option.");
    }
}

fn obtener_parametro(req: &HttpRequest, parametro: &str) -> Option<usize> {
    web::Query::<HashMap<String, String>>::from_query(req.query_string())
        .ok()
        .and_then(|query| query.get(parametro)
        .and_then(|p| p.parse::<usize>().ok()))
}

async fn salir(data: web::Data<Arc<AppState>>, req: HttpRequest) -> impl Responder {
    println!("Salir");

    let cliente = obtenerCliente(&req);
    let mut tableros = data.tableros.lock().unwrap();

    if !tableros.contains_key(&cliente) {
        return HttpResponse::PreconditionFailed().body("Error: El cliente no tiene ningun tablero inicilizado");
    }
    tableros.remove(&cliente);

    HttpResponse::Ok().body("Saliendo...")
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
            .service(web::resource("/salir").to(salir))
    }).bind(("127.0.0.1", 7070))?.run().await
}
