pub trait Client {
    fn fixed_update(&mut self, delta: f64);
    fn update(&mut self, delta: f64);
}