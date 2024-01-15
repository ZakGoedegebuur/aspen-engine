use aspen_engine::{
    Engine,
};

fn main() {
    let engine = match Engine::new(23 as u64) {
        Ok(val) => val,
        Err(err) => {
            println!("\n{}\n", err);
            return;
        }
    };
    engine.run();
}