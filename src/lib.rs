pub mod renderer;

#[cfg(test)]
mod tests {
    use super::renderer::Renderer;

    #[test]
    fn application() {
        let renderer = Renderer::new();
        println!("test strings are: {:#?}", renderer.image_paths().iter());
    }
}
