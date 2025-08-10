use rendium::*;

fn main() {
    RendiumBuilder::new()
        .with_size(600, 600)
        .with_title("Rendium Test")
        .run(|rd| {
            rd.draw(Color::BLUE, |d| {
                d.add_vertex(Vertex {
                    position: [-0.5, -0.5, 0.0], color:
                })
            })
        });
}
