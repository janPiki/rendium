use rendium::shapes::*;
use rendium::*;

fn main() {
    rendium::init()
        .with_size(700, 700)
        .with_title("Example 2")
        .run(|rd| {
            rd.draw(Color::BLACK, |d| {
                d.draw_rect(250, 250, 200, 200, Color::BLUE);
                d.draw_circle(100, 100, 100, Color::WHITE, 20);
                d.draw_line(100, 600, 600, 600, 10, Color::RED);
            });
        });
}
