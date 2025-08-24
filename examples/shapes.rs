use rendium::shapes::*;
use rendium::types::Color;

fn main() {
    rendium::init()
        .with_size(700, 700)
        .with_title("Example 2")
        .run(|rd| {
            rd.draw(Color::BLACK, |d| {
                d.draw_rect((250.0, 250.0).into(), 200, 200, Color::BLUE);
                d.draw_circle((100.0, 100.0).into(), 100, Color::WHITE, 20);
                d.draw_line(
                    (100.0, 600.0).into(),
                    (600.0, 600.0).into(),
                    10,
                    Color(255, 0, 0, 100),
                );
            });
        });
}
