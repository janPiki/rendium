use rendium::*;

fn main() {
    RendiumBuilder::new()
        .with_size(600, 600)
        .with_title("Rendium Test")
        .run(|rd| {});
}
