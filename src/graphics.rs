type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
use sdl2::{pixels::Color, rect::Rect};
use std::collections::HashMap;

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
    color: Color,
    x: i32,
    y: i32,
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Rect {
    let (t, mut r) = build_text(text, &font, &texture_creator, color);

    r.set_x(x);
    r.set_y(y);

    canvas.copy(&t, None, Some(r)).unwrap();

    r
}

pub fn draw_rect(canvas: &mut Canvas, r: &Rect, c: Color) {
    canvas.set_draw_color(c);
    canvas.fill_rect(*r).unwrap();
}

pub fn draw_variables<'a>(
    canvas: &mut Canvas,
    vars: &[(String, String)],
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    for (i, (k, v)) in vars.iter().enumerate() {
        draw_text(
            canvas,
            &k,
            Color::RGB(0xa0, 0xa0, 0xff),
            450,
            10 + (i * 30) as i32,
            font,
            texture_creator,
        );
        draw_text(
            canvas,
            &v,
            Color::RGB(0xa0, 0xa0, 0xff),
            550,
            10 + (i * 30) as i32,
            font,
            texture_creator,
        );
    }
}

pub fn draw_regs<'a>(
    canvas: &mut Canvas,
    vars: &HashMap<String, String>,
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    let sy = 100;
    for (i, (k, v)) in vars.iter().enumerate() {
        let s = format!("{} = {}", k, v);
        draw_text(
            canvas,
            &s,
            Color::RGB(0xa0, 0xa0, 0xff),
            500,
            sy + (i * 20) as i32,
            font,
            texture_creator,
        );
    }
}

pub fn draw_asm<'a>(
    canvas: &mut Canvas,
    asm: &[String],
    font: &sdl2::ttf::Font,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) {
    let mut instructions = String::new();

    for line in asm {
        if !line.is_empty() {
            instructions += line;
            instructions.push('\n');
        }
    }

    if !instructions.is_empty() {
        draw_text(
            canvas,
            &instructions,
            Color::RGB(0xff, 0x80, 0x80),
            350,
            250,
            font,
            texture_creator,
        );
    }
}
