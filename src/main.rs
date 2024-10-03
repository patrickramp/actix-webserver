// Dependencies
use actix_files::Files;
use actix_web::{guard, middleware, App, HttpServer};
use anyhow::{Context, Result};
use std::env;

// Main Actix Web server function
#[actix_web::main]
async fn main() -> Result<()> {
    // Retrieve server configuration from environment variables with default values
    let bind_address = env::var("BIND_TO").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let http_directory = env::var("HTTP_DIR").unwrap_or_else(|_| "./public".to_string());
    let primary_domain = env::var("DOMAIN").unwrap_or("localhost".to_string());
    let primary_mount = env::var("MOUNT").unwrap_or("/".to_string());
    let primary_index = env::var("INDEX").unwrap_or("index.html".to_string());

    // Default to "www" subdomain if not specified
    let secondary_domain =
        env::var("DOMAIN_TWO").unwrap_or_else(|_| format!("www.{}", primary_domain));
    let secondary_mount = env::var("MOUNT_TWO").unwrap_or(primary_mount.clone());
    let secondary_index = env::var("INDEX_TWO").unwrap_or(primary_index.clone());
    let log_level = env::var("LOG_LVL").unwrap_or("info".to_string());

    // Initialize logging based on the environment variable
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

    // Log configuration summary
    log::info!("Starting Actix Web server on {}:{}", bind_address, port);
    log::info!(
        "Serving files from: \"{}\". Primary domain: \"{}\" at mount: \"{}\" (index: \"{}\").",
        http_directory,
        primary_domain,
        primary_mount,
        primary_index
    );
    log::info!(
        "Secondary domain: \"{}\" at mount: \"{}\" (index: \"{}\").",
        secondary_domain,
        secondary_mount,
        secondary_index
    );

    // Configure and start the Actix Web server
    HttpServer::new(move || {
        App::new()
            // Enable logging middleware for request logs
            .wrap(middleware::Logger::default())
            // Set up primary domain service
            .service(
                Files::new(&primary_mount, &http_directory)
                    .guard(guard::Host(&primary_domain))
                    .index_file(&primary_index)
                    .use_last_modified(true)
                    .use_etag(true),
            )
            // Set up secondary domain service, if applicable
            .service(
                Files::new(&secondary_mount, &http_directory)
                    .guard(guard::Host(&secondary_domain))
                    .index_file(&secondary_index)
                    .use_last_modified(true)
                    .use_etag(true),
            )
    })
    .bind(format!("{}:{}", bind_address, port))
    .with_context(|| format!("Failed to bind to {}:{}", bind_address, port))?
    .run()
    .await
    .context("Server encountered an unexpected error.")
}
