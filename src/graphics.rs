type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
use sdl2::pixels::Color;

//pub struct GraphicsContext<'a, 'b> {
//    font: sdl2::ttf::Font<'a, 'b>,
//    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
//    canvas: Canvas,
//}

pub fn build_text<'a>(
    text: &str,
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    color: Color,
) -> (sdl2::render::Texture<'a>, sdl2::rect::Rect) {
    let surface = font.render(text).blended_wrapped(color, 700).unwrap();

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let sdl2::render::TextureQuery { width, height, .. } = texture.query();

    let rect = sdl2::rect::Rect::new(10, 0, width, height);

    (texture, rect)
}

pub fn draw_text<'a>(
    canvas: &mut Canvas,
    text: &str,
    x: i32,
    y: i32,
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    let (t, mut r) = build_text(text, &font, &texture_creator, Color::RGB(0xa0, 0xa0, 0xff));

    r.set_x(x);
    r.set_y(y);

    canvas.copy(&t, None, Some(r)).unwrap();
}

pub fn draw_variables<'a>(
    canvas: &mut Canvas,
    vars: &Vec<(String, String)>,
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    for (i, (k, v)) in vars.iter().enumerate() {
        draw_text(canvas, &k, 300, 10 + (i * 30) as i32, font, texture_creator);
        draw_text(canvas, &v, 450, 10 + (i * 30) as i32, font, texture_creator);
    }
}
