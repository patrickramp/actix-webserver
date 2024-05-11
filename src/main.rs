// [dependencies]
use actix_files::Files;
use actix_web::{guard, App, HttpServer};
use std::env;

// Main Actix web server function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define server parameters from environment variables
    let bind_to = env::var("BIND_TO").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let http_dir = env::var("HTTP_DIR").unwrap_or_else(|_| "./public".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let mount = env::var("MOUNT").unwrap_or_else(|_| "/".to_string());
    let index = env::var("INDEX").unwrap_or_else(|_| "index.html".to_string());
    let domain_two = env::var("DOMAIN_TWO").unwrap_or_else(|_| format!("www.{}", domain.clone()));
    let mount_two = env::var("MOUNT_TWO").unwrap_or_else(|_| mount.clone());
    let index_two = env::var("INDEX_TWO").unwrap_or_else(|_| index.clone());

    // Display server configuration to console
    println!(
        "Starting Actix Webserver...\n Listening on: {}:{}",
        bind_to, port
    );
    println!(" Serving files from directory \"{}\"", http_dir);
    println!(
        " Primary domain is \"{}\" serving \"{}\" at \"{}\"",
        domain, index, mount
    );
    println!(
        " Secondary domain is \"{}\" serving \"{}\" at \"{}\"",
        domain_two, index_two, mount_two
    );

    // Create an Actix web server with the specified configuration
    HttpServer::new(move || {
        // Service for serving files from configured directory
        App::new()
            .service(
                Files::new(&mount, &http_dir)
                    // Guard to restrict access to specified hostname
                    .guard(guard::Host(&domain))
                    // Index file name
                    .index_file(&index),
            )
            // Redundant service for optional secondary hostname(www.*),
            // index file, and mount point
            .service(
                Files::new(&mount_two, &http_dir)
                    .guard(guard::Host(&domain_two))
                    .index_file(&index_two),
            )
    })
    // Set number of worker threads to number of cores
    .workers(num_cpus::get())
    .bind(format!("{}:{}", bind_to, port))
    .expect("Server unable to bind to specified address/port.")
    .run()
    .await
}
