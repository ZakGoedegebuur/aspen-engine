pub mod renderer;
pub mod error;

#[cfg(test)]
mod tests {
    use super::renderer::Renderer;

    #[test]
    fn application() {
        let renderer = Renderer::new();
    }
}
