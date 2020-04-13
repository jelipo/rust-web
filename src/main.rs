use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2(req: HttpRequest) -> impl Responder {
    println!("{}", req.method().as_str());
    let response = HttpResponse::Ok().body("Hello world again!");
    return response;
}

async fn test() -> String {
    return String::from("Niaho");
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(test))
            .route("/again", web::get().to(index2))
    })
        .bind("127.0.0.1:8088")?
        .run();
    server.await
}
