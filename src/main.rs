use actix_files::Files;
use actix_web::{guard, App, HttpServer};
use serde_derive::Deserialize;
use std::fs;

// Top level struct to hold data from TOML file
#[derive(Deserialize)]
struct ConfigToml {
    server_config: ServerConfig,
}

// Inner struct to hold data from [server_config] section
#[derive(Deserialize)]
struct ServerConfig {
    bind_address: String,
    port: u16,
    static_dir: String,
    index_file: String,
    alt_index: String,
    hostname: String,
    alt_hostname: String,
}

// Function to load the server configuration data from TOML file
fn load_config(config_path: &str) -> ServerConfig {
    // Read the contents of the TOML file into a string
    let toml_contents =
        fs::read_to_string(config_path).expect("Unable to read configuration file.");

    // Deserialize the TOML data to top level struct
    let config_toml: ConfigToml =
        toml::from_str(&toml_contents).expect("Invalid configuration file.");

    // Return the inner ServerConfig struct
    return config_toml.server_config;
}

// Main Actix web server function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the configuration from the config.toml file
    let server_config = load_config("./config.toml");

    // Create an Actix web server with the specified configuration
    HttpServer::new(move || {
        App::new()
            // Service for serving static files from configured directory
            .service(
                Files::new("/", &server_config.static_dir)
                    // Guard to restrict access to specified hostname (to prevent hotlinking)
                    .guard(guard::Host(&server_config.hostname))
                    // Index file name
                    .index_file(&server_config.index_file),
            )
            // Redundant service to specify alternate hostname if needed
            .service(
                Files::new("/", &server_config.static_dir)
                    .guard(guard::Host(&server_config.alt_hostname))
                    .index_file(&server_config.alt_index),
            )
    })
    .bind((server_config.bind_address, server_config.port))
    .expect("Server unable to bind to specified address/port.")
    .run()
    .await
}
