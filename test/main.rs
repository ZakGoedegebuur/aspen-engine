use std::error::Error;
use aspen_engine::AppBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = AppBuilder::new()?;
    builder.use_graphics()?;
    builder.add_update_func(|_| println!("update func 1"));
    builder.add_update_func(|_| println!("update func 2"));
    let engine = builder.build()?;

    engine.run()?;

    Ok(())
}