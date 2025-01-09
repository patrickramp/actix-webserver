// Dependencies
use actix_files::Files;
use actix_web::{guard, middleware, App, HttpServer};
use anyhow::{Context, Result};
use std::env;
use std::path::Path;

// Struct to hold server environment variables
struct EnvVars {
    bind_address: String,
    port: u16,
    http_directory: String,
    primary_domain: String,
    primary_mount: String,
    primary_index: String,
    secondary_domain: String,
    secondary_mount: String,
    secondary_index: String,
    log_level: String,
}

// Main Actix Webserver function
#[actix_web::main]
async fn main() -> Result<()> {
    // Validate and load environment variables
    let server_config = match load_env_vars() {
        Ok(server_config) => server_config,
        Err(e) => {
            log::error!("Failed to load environment variables: {}", e);
            return Err(e);
        }
    };

    // Initialize logger
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(server_config.log_level.as_str()),
    );

    // Log configuration summary
    log::info!("Starting Actix-Webserver...");
    log::info!("Web root directory: \"{}\"", server_config.http_directory);
    log::info!(
        "Primary web domain: {}{}{}",
        server_config.primary_domain,
        server_config.primary_mount,
        server_config.primary_index
    );
    log::info!(
        "Secondary/subdomain: {}{}{}",
        server_config.secondary_domain,
        server_config.secondary_mount,
        server_config.secondary_index
    );

    // Configure and start the Actix Web server
    HttpServer::new(move || {
        App::new()
            // Enable logging middleware for request logs
            .wrap(middleware::Logger::default())
            // Set up primary domain service
            .service(file_server(
                &server_config.primary_mount,
                &server_config.primary_domain,
                &server_config.primary_index,
                &server_config.http_directory,
            ))
            // Set up secondary domain / subdomain service
            .service(file_server(
                &server_config.secondary_mount,
                &server_config.secondary_domain,
                &server_config.secondary_index,
                &server_config.http_directory,
            ))
    })
    .bind((server_config.bind_address.as_str(), server_config.port))
    .context(format!(
        "Failed to bind to {}:{}. Please check if the port is already in use.",
        server_config.bind_address, server_config.port
    ))?
    .run()
    .await
    .context("Server encountered an unexpected error.")
}

fn load_env_vars() -> Result<EnvVars> {
    // Get absolute path of http directory
    let abs_path = Path::new(&env::var("HTTP_DIR").unwrap_or_else(|_| "./public".to_string()))
        .canonicalize()
        .context("Failed to get absolute path of HTTP_DIR.")?;

    // Check if http directory exists
    if !abs_path.is_dir() {
        return Err(anyhow::anyhow!(
            "Web root directory does not exist or is not a directory."
        ));
    }

    // Check if index file exists
    if !abs_path
        .join(&env::var("INDEX").unwrap_or("index.html".to_string()))
        .exists()
    {
        return Err(anyhow::anyhow!(
            "Index file not found in web root directory."
        ));
    }

    // Check if the log level is valid
    let valid_level = env::var("LOG_LVL").unwrap_or("info".to_string());
    if !["trace", "debug", "info", "warn", "error"].contains(&valid_level.as_str()) {
        return Err(anyhow::anyhow!(
            "Invalid log level: {}. Valid options are: trace, debug, info, warn, error.",
            &valid_level
        ));
    }

    Ok(EnvVars {
        bind_address: env::var("BIND_TO").unwrap_or("127.0.0.1".to_string()),
        port: env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .context("Invalid port number.")?,
        http_directory: abs_path
            .to_str()
            .context("Invalid web root path.")?
            .to_string(),
        primary_domain: env::var("DOMAIN").unwrap_or("localhost".to_string()),
        primary_mount: env::var("MOUNT").unwrap_or("/".to_string()),
        primary_index: env::var("INDEX").unwrap_or("index.html".to_string()),
        secondary_domain: env::var("DOMAIN_TWO").unwrap_or(format!(
            "www.{}",
            env::var("DOMAIN").unwrap_or("localhost".to_string())
        )),
        secondary_mount: env::var("MOUNT_TWO")
            .unwrap_or(env::var("MOUNT").unwrap_or("/".to_string())),
        secondary_index: env::var("INDEX_TWO")
            .unwrap_or(env::var("INDEX").unwrap_or("index.html".to_string())),
        log_level: valid_level,
    })
}

// Helper function to configure Actix Web file server
fn file_server(mount: &str, domain: &str, index: &str, directory: &str) -> Files {
    Files::new(mount, directory)
        .guard(guard::Host(domain))
        .index_file(index)
        .use_last_modified(true)
        .use_etag(true)
}
