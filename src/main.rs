mod aspen;
mod error;

fn main() {
    let aspen_fw = match aspen::Aspen::new() {
        Ok(val) => val,
        Err(err) => {
            match msgbox::create(
                "Crash", 
                format!("Error: {}\n\nDescription: {}", err.message, err.description).as_str(), 
                msgbox::IconType::Error
            ) {
                Ok(_) => return,
                Err(err) => {
                    println!("message box creation error: {}", err.to_string());
                    return;
                }
            }
        }
    };

    aspen_fw.run();

   // println!("renderer: {:?}", renderer);
}