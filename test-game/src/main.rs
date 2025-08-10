use rendium::*;

fn main() {
    RendiumBuilder::new()
        .with_size(600, 600)
        .with_title("Rendium Test")
        .run(|rd| {
            rd.draw(Color::BLUE, |d| {
                d.add_vertex(Vertex {
                    position: [-0.0868241, 0.49240386, 0.0],
                    color: [0.5, 0.0, 0.5],
                });
                d.add_vertex(Vertex {
                    position: [-0.49513406, 0.06958647, 0.0],
                    color: [0.0, 0.5, 0.5],
                });
                d.add_vertex(Vertex {
                    position: [-0.21918549, -0.44939706, 0.0],
                    color: [0.5, 0.5, 0.0],
                });
                d.add_vertex(Vertex {
                    position: [0.35966998, -0.3473291, 0.0],
                    color: [0.5, 0.5, 0.5],
                });
                d.add_vertex(Vertex {
                    position: [0.44147372, 0.2347359, 0.0],
                    color: [0.0, 0.0, 0.0],
                });

                d.add_index(0);
                d.add_index(1);
                d.add_index(4);
                d.add_index(1);
                d.add_index(2);
                d.add_index(4);
                d.add_index(2);
                d.add_index(3);
                d.add_index(4);
            })
        });
}
