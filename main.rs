// Import necessary modules
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use oxide_rs::prelude::*;
use std::sync::{Arc, Mutex};

// Define your extension struct
struct WebpageExtension {
    maintenance_mode: Arc<Mutex<bool>>,
    website_dir: Arc<Mutex<String>>,
    main_file: Arc<Mutex<String>>,
}

// Implement the Extension trait for your extension struct
impl Extension for WebpageExtension {
    // Define the name of your extension
    fn name(&self) -> &'static str {
        "[Oxide.webpage]"
    }

    // Define any initialization logic for your extension
    fn init(&mut self, api: &mut Api) {
        println!("[Oxide.webpage] initialized!");

        // Define the web server configuration
        api.web_server(|cfg| {
            cfg.service(web::resource("/{filename:.*}").route(web::get().to(static_file_handler)));
        });

        // Register console commands
        api.register_console_command("webpage.stop", Box::new(|_, _, _, _| {
            println!("[Oxide.webpage] Stopping server...");

            // Logic to stop the server
            std::process::exit(0); // Exit the process gracefully
        }));


        api.register_console_command("webpage.start", Box::new(|_, _, plugin, _| {
            println!("[Oxide.webpage] Starting server...");

            // Logic to start the server
            let mut plugin = plugin.downcast_mut::<WebpageExtension>().unwrap();
            // Start Actix Web server with your configurations
            start_web_server(&mut plugin);

            println!("[Oxide.webpage] Server started successfully.");
        }));

        // Register console commands
        api.register_console_command("webpage.domain.connect", Box::new(|args, _, plugin, _| {
            // Parse domain name from arguments
            let domain_name = match args.get(0) {
            Some(domain) => domain.clone(),
            None => {
                return Err("No domain name provided".to_string());
            }
            };

            // Implement logic to connect the domain
            // Example: Update DNS records, configure routing rules, etc.
            let mut plugin = plugin.downcast_mut::<WebpageExtension>().unwrap();
            connect_domain(&domain_name, &plugin)?;

            Ok(format!("[Oxide.webpage] Connected domain: {}", domain_name))
        }));

        // Define a function to connect the domain
        fn connect_domain(domain_name: &str, plugin: &WebpageExtension) -> Result<(), String> {
            // Perform domain connection logic here
            // Example: Update DNS records, configure routing rules, etc.
            println!("[Oxide.webpage] Connecting domain {}", domain_name);

            // Simulated logic: Print a message indicating successful connection
            println!("[Oxide.webpage] Domain {} connected successfully", domain_name);
            Ok(())
        }


        api.register_console_command("webpage.maintenance.mode", Box::new(|_args, _player, plugin, _reply| {
            // Toggle maintenance mode
            let mut maintenance_mode = plugin.downcast_mut::<WebpageExtension>().unwrap().maintenance_mode.lock().unwrap();
            *maintenance_mode = !*maintenance_mode;
            let maintenance_status = if *maintenance_mode { "enabled" } else { "disabled" };
            println!("[Oxide.webpage] Maintenance mode {}", maintenance_status);
        }));

        api.register_console_command("webpage.info", Box::new(|_args, _player, _plugin, reply| {
            reply("Webpage Extension: Serve static webpages and manage settings.");
        }));

        api.register_console_command("webpage_dir", Box::new(|args, _, plugin, reply| {
            let website_dir = args.get(0).unwrap_or(&"").to_string();
            let main_file = args.get(1).unwrap_or(&"").to_string();
            let mut plugin = plugin.downcast_mut::<WebpageExtension>().unwrap();
            *plugin.website_dir.lock().unwrap() = website_dir;
            *plugin.main_file.lock().unwrap() = main_file;
            reply(format!("Website directory set to: {}, Main file set to: {}", website_dir, main_file));
        }));
    }

    // Define any cleanup logic for your extension
    fn shutdown(&mut self) {
        println!("[Oxide.webpage] shutting down!");
        // You can perform cleanup tasks here
    }
}

// Define the route handler for serving static files
async fn static_file_handler(req: HttpRequest) -> HttpResponse {
    // Parse requested filename
    let filename: String = req.match_info().query("filename").parse().unwrap();
    // Serve the requested file from the website directory
    let website_dir = WEBSITE_DIR.lock().unwrap().clone();
    let file_path = format!("{}/{}", website_dir, filename);
    match web::block(move || {
        std::fs::read_to_string(file_path)
    }).await {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::NotFound().body("File not found"),
    }
}

// Entry point for your extension
fn main() {
    // Create an instance of your extension
    let extension = WebpageExtension {
        maintenance_mode: Arc::new(Mutex::new(false)),
        website_dir: Arc::new(Mutex::new("website".to_string())), // Default website directory
        main_file: Arc::new(Mutex::new("index.html".to_string())), // Default main file
    };

    // Start the Oxide framework and register your extension
    oxide_rs::start_extension(Box::new(extension));
}
