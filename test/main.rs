mod client;

use client::AppData;
use aspen_engine::application::Application;

fn main() { 
    let app_data = AppData::new();

    let application = Application::new(
        app_data,
        true,
    );

    application.run()
}

// 5390 1094
// 809 217 1611