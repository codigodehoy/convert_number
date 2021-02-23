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
    HttpResponse::Ok().body(format!("{}", body))
}

fn convert_number(mut number: u128) -> String {
    let mut map = HashMap::new();  
    //  --------- 0 - 19 --------------
    map.insert(0, "".to_string()); 
    map.insert(1, "uno ".to_string()); 
    map.insert(2, "dos ".to_string());
    map.insert(3, "tres ".to_string());  
    map.insert(4, "cuatro ".to_string()); 
    map.insert(5, "cinco ".to_string()); 
    map.insert(6, "seis ".to_string());
    map.insert(7, "siete ".to_string());
    map.insert(8, "ocho ".to_string()); 
    map.insert(9, "nueve ".to_string()); 
    map.insert(10,"diez ".to_string());
  
    //  --------- 20 - 90 --------------  
    map.insert(20,"veinte".to_string());
    map.insert(30, "treinta y ".to_string()); 
    map.insert(40, "cuarenta y ".to_string()); 
    map.insert(50, "cicuenta y ".to_string()); 
    map.insert(60, "sesenta y ".to_string()); 
    map.insert(70, "setenta y ".to_string()); 
    map.insert(80, "ochenta y ".to_string()); 
    map.insert(90, "noventa y ".to_string());   
    // 100 - 200 -300 - 400 - 500 - 600 - 700 - 800 - 900
    map.insert(100, "cien ".to_string()); 
    map.insert(200, "doscientos ".to_string()); 
    map.insert(300, "trecientos ".to_string()); 
    map.insert(400, "cuatrocientos ".to_string()); 
    map.insert(500, "quinientos ".to_string()); 
    map.insert(600, "seicientos ".to_string()); 
    map.insert(700, "setecientos ".to_string()); 
    map.insert(800, "ochocientos ".to_string()); 
    map.insert(900, "novecientos ".to_string());

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
            mil = String::from("mil ");
        }else{
            mil = String::new();
        } 
        if j == 7  {
            millon = String::from("millones ");
        }else{
            millon = String::new();
        } 
        if j == 10  {
            mil2 = String::from("mil ");
        }else{
            mil2 = String::new();
        } 
        if j == 13  {
            billon = String::from("billones ");
        }else{
            billon = String::new();
        } 
        if j == 16  {
            mil3 = String::from("mil ");
        }else{
            mil3 = String::new();
        } 
        if j == 19  {
            trillones = String::from("trillones ");
        }else{
            trillones = String::new();
        } 
        if j == 22  {
            mil4 = String::from("mil ");
        }else{
            mil4 = String::new();
        } 
        if j == 25  {
            cuatrillones = String::from("cuatrillones ");
        }else{
            cuatrillones = String::new();
        }
        if j == 28  {
            mil5 = String::from("mil ");
        }else{
            mil5 = String::new();
        } 
        if j == 31  {
            quintillones = String::from("quintillones ");
        }else{
            quintillones = String::new();
        }
        
        number = number / 10;
        match map.get(&res) {
            Some(valor) => { 
                aux = valor.to_string()+&quintillones+&mil5
                +&cuatrillones+&mil4
                +&trillones+&mil3
                +&billon+&mil2
                +&millon
                +&mil
                +&aux;
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