use aspen_engine::interface;

pub struct AppData {

}

impl AppData {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl interface::Client for AppData {
    fn fixed_update(&mut self, delta: f64) {
        
    }

    fn update(&mut self, delta: f64) {
        
    }
}