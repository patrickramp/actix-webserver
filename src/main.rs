// [dev-dependencies]
use actix_files::Files;
use actix_web::{guard, App, HttpServer};
use std::env;

// Main Actix web server function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define server parameters from environment variables
    let bind_to = env::var("BIND_TO").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap();
    let http_dir = env::var("HTTP_DIR").unwrap_or("./public".to_string());
    let domain = env::var("DOMAIN").unwrap_or("localhost".to_string());
    let mount = env::var("MOUNT").unwrap_or("/".to_string());
    let index = env::var("INDEX").unwrap_or("index.html".to_string());
    let domain_two = env::var("DOMAIN_TWO")
        .unwrap_or(("www.".to_owned() + &domain).to_string());
    let mount_two = env::var("MOUNT_TWO").unwrap_or(mount.to_string());
    let index_two = env::var("INDEX_TWO").unwrap_or(index.to_string());

    // Display server configuration to console
    println!("Starting Actix Webserver at {}:{}", bind_to, port);
    println!("Serving files from local directory \"{}\"", http_dir);
    println!("Primary hostname is \"{}\" serving \"{}\" at \"{}\"", domain, index, mount);
    println!("Secondary hostname is {} serving \"{}\" at \"{}\"", domain_two, index_two, mount_two);

    // Create an Actix web server with the specified configuration
    HttpServer::new(move || {
        App::new()
            // Service for serving files from configured directory
            .service(
                Files::new(&mount, &http_dir)
                    // Guard to restrict access to specified hostname (prevent hotlinks)
                    .guard(guard::Host(&domain))
                    // Index file name
                    .index_file(&index),
            )
            // Redundant service for optional secondary hostname(www.*), index file, and mount point
            .service(
                Files::new(&mount_two, &http_dir)
                    .guard(guard::Host(&domain_two))
                    .index_file(&index_two),
            )
    })
    .bind((bind_to, port))
    .expect("Server unable to bind to specified address/port.")
    .run()
    .await
}
