use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Info {
    number: u128,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("David Cadena!")
}

async fn index(info: web::Json<Info>) -> impl Responder {
    let body = convert_number(info.number);
    // print!("{} ", aux);
    HttpResponse::Ok().body(format!("{}", body))
}

fn convert_number(mut number: u128) -> String {
    let mut map = HashMap::new();  
    //  --------- 0 - 19 --------------
    map.insert(0, String::new());
    map.insert(1, String::from("uno"));
    map.insert(2,String::from("dos"));
    map.insert(3,String::from("tres"));
    map.insert(4,String::from("cuatro"));
    map.insert(5,String::from("cinco"));
    map.insert(6,String::from("seis"));
    map.insert(7,String::from("siete"));
    map.insert(8,String::from("ocho"));
    map.insert(9,String::from("nueve"));
    map.insert(10,String::from("diez"));
    map.insert(11,String::from("once"));
    map.insert(12,String::from("doce"));
    map.insert(13,String::from("trece"));
    map.insert(14,String::from("catorce"));
    map.insert(15,String::from("quince"));
    map.insert(16,String::from("dieciséis"));
    map.insert(17,String::from("diecisiete"));
    map.insert(18,String::from("dieciocho"));
    map.insert(19,String::from("diecinueve"));
    //  --------- 20 - 29 --------------  
    map.insert(20,String::from("veinte"));
    map.insert(21,String::from("veintiuno"));
    map.insert(22,String::from("veintidós"));
    map.insert(23,String::from("veintitrés"));
    map.insert(24,String::from("veinticuatro"));
    map.insert(25,String::from("veinticinco"));
    map.insert(26,String::from("Veintiséis"));
    map.insert(27,String::from("veintisiete"));
    map.insert(28,String::from("veintiocho"));
    map.insert(29,String::from("veintinueve"));
    // --- 30 - 40 - 50 - 60 - 70 - 80 - 90 
    map.insert(30,String::from("treinta y"));
    map.insert(40,String::from("cuarenta y"));
    map.insert(50,String::from("cicuenta y"));
    map.insert(60,String::from("sesenta y"));
    map.insert(70,String::from("setenta y"));
    map.insert(80,String::from("ochenta y"));
    map.insert(90,String::from("noventa y"));
    // 100 - 200 -300 - 400 - 500 - 600 - 700 - 800 - 900
    map.insert(100,String::from("cien"));
    map.insert(200,String::from("doscientos"));
    map.insert(300,String::from("trecientos"));
    map.insert(400,String::from("cuatrocientos"));
    map.insert(500,String::from("quinientos"));
    map.insert(600,String::from("seicientos"));
    map.insert(700,String::from("setecientos"));
    map.insert(800,String::from("ochocientos"));
    map.insert(900,String::from("novecientos"));

     // 
    let mut mil = String::new();
    let mut millon = String::new();
    let mut mil2 = String::new();
    let mut billon = String::new();
    let mut mil3 = String::new(); 
    let mut trillones = String::new();
    let mut mil4 = String::new(); 
    let mut cuatrillones = String::new();
    let mut mil5 = String::new(); 
    let mut quintillones = String::new();

    let mut i = 0;
    let mut j = 0;
    
    // let base = 10;
    let base:u128 = 10; 
    // let mut numero = 408965367951020;
    let  mut aux = String::new();
    while number != 0 {
        let res: u128;
        res = (number % 10 ) * (base.pow(i));
        i = i + 1;
        j +=1;
        if i > 2 {
            i = 0;
        } 

        // --------MiL - Quintillones -----------
        if j == 4  {
            mil = String::from("mil");
        }else{
            mil = String::new();
        } 
        if j == 7  {
            millon = String::from("millones");
        }else{
            millon = String::new();
        } 
        if j == 10  {
            mil2 = String::from("mil");
        }else{
            mil2 = String::new();
        } 
        if j == 13  {
            billon = String::from("billones");
        }else{
            billon = String::new();
        } 
        if j == 16  {
            mil3 = String::from("mil");
        }else{
            mil3 = String::new();
        } 
        if j == 19  {
            trillones = String::from("trillones");
        }else{
            trillones = String::new();
        } 
        if j == 22  {
            mil4 = String::from("mil");
        }else{
            mil4 = String::new();
        } 
        if j == 25  {
            cuatrillones = String::from("cuatrillones");
        }else{
            cuatrillones = String::new();
        }
        if j == 28  {
            mil5 = String::from("mil");
        }else{
            mil5 = String::new();
        } 
        if j == 31  {
            quintillones = String::from("quintillones");
        }else{
            quintillones = String::new();
        }
        
        number = number / 10;
        match map.get(&res) {
            Some(valor) => { 
                aux = valor.to_string()+
                " "+&quintillones+
                " "+&mil5+
                " "+&cuatrillones+
                " "+&mil4+
                " "+&trillones+
                " "+&mil3+
                " "+&billon+
                " "+&mil2+
                " "+&millon+
                " "+&mil+
                ""+&aux;
            }
            None => print!("No existe ese valor")
        }  
        
    }
    return aux;

}



async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
            .route("/app", web::post().to(index))
            // .service(echo)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}