use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::error::Error;
use actix_files::{NamedFile, Files};

use handlebars::Handlebars;
use serde_json::json;
// Serving a response
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

// Serving Static Pages
async fn index() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/index.html")?)
}

// Serving Dynamic Page  - Handlebars loads and compiles the templates before using them.
async fn index2(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name" : "Catdex",
        "cats": [
            {
                "name": "British short hair",
                "image_path":
                    "/static/image/british-short-hair.jpg"
            },
            {
                "name": "Persian",
                "image_path": "/static/image/persian.jpg"
            },
            {
                "name": "Ragdoll",
                "image_path": "/static/image/ragdoll.jpg"
            }
        ]
    });

    let body = hb.render("index2", &data).unwrap(); // use the hb.render() call to render the template.

    HttpResponse::Ok().body(body)
}


#[actix_web::main] // Tells Actix to execute main() function in a special runtime called actix_rt which is built on Tokio runtime (async)
async fn main() -> std::io::Result<()> {
    println!("Listing on port 8080");
    
    //  And it caches the compiled templates so they don't need to be recompiled every time you use them. 
    //  Therefore, we can initialize the Handlebars template engine in the main() function.
    let mut handlebars = Handlebars::new();
    handlebars
        // This function is guarded by a dir_source feature, which we enabled in Cargo.toml (Listing 2-7).
        .register_templates_directory(".html", "./static/") // register all the tempalte with .html extension in the static folder
        .unwrap();

    // To avoid recompiling the templates in each thread, we need a way to share this Handlebars instance across threads. 
    // To share states between threads, you can use the web::Data provided by Actix
    let handlebars_ref = web::Data::new(handlebars); 

    HttpServer::new( move || { // The App factory closure now needs to take ownership of the cloned web::Data object, so you need to add move for the closure.
        App::new()
            .route("/hello", web::get().to(hello))
            .route("/", web::get().to(index))
            .app_data(handlebars_ref.clone()) // The web::Data object is provided to the App builder by the .app_data() function. 
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/dynamic_index", web::get().to(index2))
    })
    .bind("127.0.0.1:8080")? // binds web server to port on said IP Address
    .run()
    .await
}