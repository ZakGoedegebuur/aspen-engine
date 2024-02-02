use std::error::Error;

use aspen_engine::Engine;

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new("log/log.json".to_string())?;
    engine.use_graphics()?;
    engine.run()?;

    Ok(())
}