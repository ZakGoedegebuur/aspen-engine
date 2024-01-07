#[derive(Debug)]
pub struct Renderer {
    image_paths: Vec<String>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            image_paths: vec!["hello".to_owned(), "renderer!".to_owned()]
        }
    }

    pub fn image_paths(&self) -> &Vec<String> {
        &self.image_paths
    }
}