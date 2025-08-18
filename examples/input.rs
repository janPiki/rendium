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

            let pos = rd.get_mouse_pos();

            rd.draw(Color::BLACK, |d| {
                d.draw_circle(pos, 100, color, 40);
            });
        });
}
