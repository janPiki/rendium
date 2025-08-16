use rendium::input::{Input, Key};
use rendium::shapes::DrawShape;
use rendium::types::Color;

fn main() {
    rendium::init()
        .with_size(600, 600)
        .with_title("Example 3")
        .run(|rd| {
            let color = if rd.is_key_down(Key::Space) {
                Color::BLUE
            } else {
                Color::RED
            };

            rd.draw(Color::BLACK, |d| {
                d.draw_circle((300.0, 300.0).into(), 100, color, 40);
            });
        });
}
