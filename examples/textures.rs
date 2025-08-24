use rendium::{texture::*, types::Color};

fn main() {
    rendium::init()
        .with_size(600, 600)
        .with_title("Example 4")
        .setup(|rd| {
            rd.load_texture("../logo.png", "logo");
        })
        .run(|rd| {
            rd.draw(Color::BLACK, |d| {
                d.draw_texture(
                    "logo",
                    (300.0, 300.0).into(),
                    (128.0, 128.0).into(),
                    Color::WHITE,
                );
            });
        });
}
