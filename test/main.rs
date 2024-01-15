use aspen_engine::Engine;

fn main() {
    let mut engine = match Engine::new(0 as u64, "log/log.json".to_owned()) {
        Ok(val) => val,
        Err(err) => {
            println!("\n{}\n", err);
            return;
        }
    };

    match engine.open_window() {
        Ok(_) => (),
        Err(err) => {
            println!("\n{}\n", err);
            return;
        }
    };

    match engine.open_window() {
        Ok(_) => (),
        Err(err) => {
            println!("\n{}\n", err);
            return;
        }
    };
    
    engine.run();
}