## Simple, multithreaded webserver for serving static HTML pages and files using the Actix framework in Rust. Lightweight and fast with low attack surface, designed for use in conainerized environments.

### Server is configured by passing the following environment variables.: [default]
  - BIND_TO= The IP address you wish the server to listen on. [127.0.0.1 (listen on localhost)] 
  - PORT= Port you wish to use for the webserver. [8080]
  - HTTP_DIR= Root directory you wish to serve public files from. [./public]
  - DOMAIN= Public domain name for your server (example.com) [localhost]
  - MOUNT= Primary web path to serve INDEX file ["/"]
  - INDEX= Default file (in HTTP_DIR) to served when someone visits your domain [index.html]
  - DOMAIN_TWO= Optional secondary domain or subdomain, [www.DOMAIN]
  - MOUNT_TWO= Optional secondary web path for DOMAIN_TWO ["MOUNT"]
  - INDEX_TWO= Optional index file to serve at DOMAIN_TWO/MOUNT_TWO [INDEX]
  - LOG_LVL= Logging level [info]

### Dependencies
#### Actix Web dependencies
actix-files = "0.6.6"
actix-web = "4.9.0"

#### Logging
env_logger = "0.11.6"
log = "0.4.22"

#### Error handling with anyhow
anyhow = "1.0.95"


