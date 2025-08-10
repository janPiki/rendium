use rendium::*;

fn main() {
    RendiumBuilder::new()
        .with_size(600, 600)
        .with_title("Rendium Example")
        .run(|rd| {
            rd.draw(Color::BLACK, |d| {
                d.add_vertex([300.0, 100.0, 0.0], Color::RED);
                d.add_vertex([100.0, 500.0, 0.0], Color::GREEN);
                d.add_vertex([500.0, 500.0, 0.0], Color::BLUE);

                d.add_index(0);
                d.add_index(1);
                d.add_index(2);
            });
        });
}
