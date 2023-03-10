/* This ia an actix Microservice that has multiple routes:
A. type: "/" that returns a message : "Hello, random best movie around the world!"
B. type: "/movie" that returns a random best movie in the list of the world top 10 best movies
C. type: "/version" that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use webdocker::get_solar_distance;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, get the solar distance of bodies in solar system!")
}

//create a function that returns the version of the service
#[get("/Mercury")]
async fn mercury() -> impl Responder {
    let planet_name = "Mercury";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}

#[get("/Venus")]
async fn venus() -> impl Responder {
    let planet_name = "Venus";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}
#[get("/Earth")]
async fn earth() -> impl Responder {
    let planet_name = "Earth";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}
#[get("/Mars")]
async fn mars() -> impl Responder {
    let planet_name = "Mars";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}
#[get("/Jupiter")]
async fn jupiter() -> impl Responder {
    let planet_name = "Jupiter";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}
#[get("/Saturn")]
async fn saturn() -> impl Responder {
    let planet_name = "Saturn";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}
#[get("/Uranus")]
async fn uranus() -> impl Responder {
    let planet_name = "Uranus";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}
#[get("/Neptune")]
async fn neptune() -> impl Responder {
    let planet_name = "Neptune";
    println!(
        "The distance of {} from the sun is {} AU",
        planet_name,
        get_solar_distance(planet_name)
    );
    HttpResponse::Ok().body(get_solar_distance(planet_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(mercury)
            .service(venus)
            .service(earth)
            .service(mars)
            .service(jupiter)
            .service(saturn)
            .service(uranus)
            .service(neptune)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
