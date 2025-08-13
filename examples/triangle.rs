use rendium::*;

fn main() {
    rendium::init()
        .with_size(700, 700)
        .with_title("Example 1")
        .run(|rd| {
            rd.draw(Color::BLACK, |d| {
                d.add_vertex([350.0, 200.0, 0.0], Color::RED);
                d.add_vertex([150.0, 500.0, 0.0], Color::GREEN);
                d.add_vertex([550.0, 500.0, 0.0], Color::BLUE);

                d.add_index(0);
                d.add_index(1);
                d.add_index(2);
            });
        });
}
