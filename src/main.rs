// Import dependencies
use actix_files::Files;
use actix_web::{guard, middleware, App, HttpServer};
use std::env;
use std::io;

// Main Actix Web server function
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Retrieve server configuration from environment variables with default values
    let bind_to = env::var("BIND_TO").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let http_dir = env::var("HTTP_DIR").unwrap_or_else(|_| "./public".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let mount = env::var("MOUNT").unwrap_or_else(|_| "/".to_string());
    let index = env::var("INDEX").unwrap_or_else(|_| "index.html".to_string());
    let domain_two = env::var("DOMAIN_TWO").unwrap_or_else(|_| format!("www.{}", domain));
    let mount_two = env::var("MOUNT_TWO").unwrap_or_else(|_| mount.clone());
    let index_two = env::var("INDEX_TWO").unwrap_or_else(|_| index.clone());
    let log_lvl = env::var("LOG_LVL").unwrap_or_else(|_| "info".to_string());

    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_lvl));

    // Output server configuration to the console
    println!("Starting Actix Web server...");
    println!("Listening on: {}:{}", bind_to, port);
    println!("Serving files from directory \"{}\"", http_dir);
    println!(
        "Primary domain is \"{}\" serving \"{}\" at \"{}\"",
        domain, index, mount
    );
    println!(
        "Secondary domain is \"{}\" serving \"{}\" at \"{}\"",
        domain_two, index_two, mount_two
    );

    // Configure and start the Actix Web server
    HttpServer::new(move || {
        App::new()
            // Enable logging middleware
            .wrap(middleware::Logger::default())
            // Service for primary domain
            .service(
                Files::new(&mount, &http_dir)
                    .guard(guard::Host(&domain))
                    .index_file(&index),
            )
            // Service for secondary domain (if configured)
            .service(
                Files::new(&mount_two, &http_dir)
                    .guard(guard::Host(&domain_two))
                    .index_file(&index_two),
            )
    })
    // Configure server to use number of CPU cores for worker threads
    .workers(num_cpus::get())
    // Bind the server to the specified address and port
    .bind(format!("{}:{}", bind_to, port))
    .expect(
        format!(
            "Failed to bind to {}:{} - Ensure port is not in use.",
            bind_to, port
        )
        .as_str(),
    )
    .run()
    .await
}
