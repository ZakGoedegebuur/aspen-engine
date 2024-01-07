pub mod renderer;
pub mod error;

fn main() {
    let renderer = renderer::Renderer::new();
    match &renderer {
        Ok(_) => {},
        Err(err) => {
            match msgbox::create(
                "Crash", 
                format!("Fatal error during renderer creation: {}\nDescription: {}", err.message, err.description).as_str(), 
                msgbox::IconType::Error
            ) {
                Ok(_) => {},
                Err(err) => {
                    println!("message box creation error: {}", err.to_string());
                    return;
                }
            }
        }
    }

    println!("renderer: {:#?}", renderer);
}