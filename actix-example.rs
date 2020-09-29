extern crate actix_web;
extern crate sqlx ;
use actix_web::{get , post , web , App , HttpResponse , HttpServer , Responder , HttpRequest , Result };
use std::time::{Duration, Instant};
use std::thread::sleep;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
#[get("/bruh")]
async fn bruh() -> impl Responder {


    HttpResponse::Ok().body("bruh it works")
}

async fn ye () -> impl Responder{
    HttpResponse::Ok().body("you just viewed kek/ye")
}


//just for example
struct AppState {
    app_name: String,
}

#[get("/ex")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}

#[get("/test")]
async fn index_async(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[get("/name/{name}")]
async fn name (web::Path(name) : web::Path<String> ) -> Result<String> {
    Ok(format!("{}" , name) )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://keshav@localhost/actix")
        .await;
    match pool {
        Ok(ref val) => println!("successfully dbed")  ,
        Err(ref e) => println!("{:?} " , e )
    }


        let query   =  sqlx::query("SELECT id FROM bruh ").fetch_all(&pool.unwrap()).await;
         match query {
            Ok(value) =>  {

           let pp : i32 = value[0].get("id");


           println!("{:?}" , pp )
        } ,
        Err(_e)  => println!("err" )
    }



    HttpServer::new(|| {
        App::new()
            .service(bruh)
            .service(
                web::scope("/kek")
                    .route("/ye" , web::get().to(ye)),
                    )
            .service(index)
            .service(index_async)
            .service(name)






    }
    )


    .bind("127.0.0.1:8080")?
    .run()
    .await
}
