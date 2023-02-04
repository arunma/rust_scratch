use std::{convert::Infallible, io};

use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    //let auth_scope = web::scope("/auth").service(home).service(login);

    /* HttpServer::new(|| {
        App::new().service(web::scope("/app").route("/index.html", web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await */

    //let auth_scope = web::scope("/auth").service(home).service(login);
    //let auth_scope = web::scope("/app").route("/index.html", web::get().to(index));
    /* let auth_scope = web::scope("/app").route("/index.html", web::get().to(index));
    HttpServer::new(|| scope)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await */

    Ok(())
}

async fn hello() -> io::Result<()> {
    /*   let auth_scope = web::scope("/app").route("/index.html", web::get().to(index));

    let server1 = HttpServer::new(|| App::new().service(auth_scope))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await; //cannot be sent between threads safely */
    /*
       let app_entry = {
           let auth_scope = web::scope("/app").route("/index.html", web::get().to(index));
           App::new().service(auth_scope)
       };

       let server2 = HttpServer::new(|| app_entry)
           .bind(("127.0.0.1", 8080))?
           .run()
           .await; //cannot be sent between threads safely
    */
    let closure = || {
        let auth_scope = web::scope("/app").route("/index.html", web::get().to(index));
        App::new().service(auth_scope)
    };

    let server3 = HttpServer::new(closure)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await; // This is fine

    Ok(())
}
