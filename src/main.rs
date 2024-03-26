
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use actix_web::HttpRequest;
use actix_web::{web, http::header};
use actix_web::middleware;


use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::Error;

// login test
#[get("/test")]
async fn test(request: HttpRequest) -> impl Responder {
	let req_headers = request.headers();
	let basic_auth_header = req_headers.get("Authorization");
	let basic_auth: &str = basic_auth_header.unwrap().to_str().unwrap();
	println!("{}", basic_auth);

	HttpResponse::Ok().body("true")
}

/*
// index
#[get("/")]
async fn index() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)

}
*/
/*
// index
#[get("/uploader")]
async fn uploader() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}
// index
#[get("/login")]
async fn login() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}
*/

/*
async fn index2() -> impl Responder {
    //let data = fs::read_to_string("/var/www/index.html").expect("Cannot read index file");
    let data = std::fs::read("/app/www/index.html").expect("Cannot read index file");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data)
}
*/


//status
async fn status() -> String {
    "Server is up and running.".to_string()
}




async fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let file = fs::NamedFile::open("/app/www/index.html")?;
    Ok(file)
	
}








#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
		.wrap(middleware::Compress::default())
		.route("/status", web::get().to(status))
		.route("/login", web::get().to(index))
		.route("/uploader", web::get().to(index))
	    .service(test)
		//.service(index)
		//.route("/",web::get().to(index2))
		//.service(uploader)
		//.service(login)
        .service(fs::Files::new("/", "/app/www"))
		.service(web::resource("/{project_id}")
			.route(web::put().to(|| HttpResponse::Ok())))

		.default_service(web::get().to(index))
	    
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
