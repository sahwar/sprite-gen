extern crate sprite_gen;
extern crate blit;
extern crate minifb;
extern crate direct_gui;

use sprite_gen::*;
use blit::*;
use minifb::*;
use direct_gui::*;
use direct_gui::controls::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

const GRID_SQUARE_SIZE: usize = 12;

const COLOR_ALWAYS_SOLID: u32 = 0x666666;
const COLOR_ALWAYS_EMPTY: u32 = 0xFAFAFA;
const COLOR_BODY: u32 = 0xFF6666;
const COLOR_BODY2: u32 = 0x6666FF;

fn draw_grid(mask: &[i8], mask_width: usize, buffer: &mut Vec<u32>, pos: (usize, usize), size: (usize, usize)) {
    let width = size.0 * GRID_SQUARE_SIZE;
    let height = size.1 * GRID_SQUARE_SIZE;

    for y in pos.1..pos.1 + height + 1 {
        let mask_y = (y - pos.1) / GRID_SQUARE_SIZE;
        for x in pos.0..pos.0 + width + 1 {
            let index = x + y * WIDTH;
            if (y - pos.1) % GRID_SQUARE_SIZE == 0 || (x - pos.0) % GRID_SQUARE_SIZE == 0 {
                buffer[index] = 0xEEEEEE;
            } else { 
                let mask_x = (x - pos.0) / GRID_SQUARE_SIZE;

                let mask_index = mask_x + mask_y * mask_width;
                buffer[index] = match mask[mask_index] {
                    -1 => COLOR_ALWAYS_SOLID,
                    0 => COLOR_ALWAYS_EMPTY,
                    1 => COLOR_BODY,
                    2 => COLOR_BODY2,
                    _ => unreachable!()
                };
            }
        }
    }
}

fn redraw(mask: &[i8], mut buffer: &mut Vec<u32>) {
    for mut pixel in buffer.iter_mut() {
        *pixel = 0xFFFFFF;
    }

    let options = Options {
        mirror_x: true,
        ..Options::default()
    };

    let sprite_size = (12, 12);
    let sprite_size_padded = (sprite_size.0 + 2, sprite_size.1 + 2);

    for y in 0..HEIGHT / sprite_size_padded.1 {
        for x in 0..WIDTH / 2 / sprite_size_padded.0 {
            let buf = BlitBuffer::from_buffer(&gen_sprite(mask, 6, options), sprite_size.0 as i32, Color::from_u32(0xFFFFFF));
            let pos = ((x * sprite_size_padded.0 + WIDTH / 2) as i32, (y * sprite_size_padded.1) as i32);
            buf.blit(&mut buffer, WIDTH, pos);
        }
    }
}

fn main() {
    let screen_size = (WIDTH as i32, HEIGHT as i32);

    let mut buffer: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];

    let mut mask = [
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1,
        0, 0, 0, 0, 1,-1,
        0, 0, 0, 1, 1,-1,
        0, 0, 0, 1, 1,-1,
        0, 0, 1, 1, 1,-1,
        0, 1, 1, 1, 2, 2,
        0, 1, 1, 1, 2, 2,
        0, 1, 1, 1, 2, 2,
        0, 1, 1, 1, 1,-1,
        0, 0, 0, 1, 1, 1,
        0, 0, 0, 0, 0, 0];

    let options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };
    let mut window = Window::new("Sprite - ESC to exit", WIDTH, HEIGHT, options).expect("Unable to open window");

    let mut gui = Gui::new(screen_size);

    // The color selection buttons
    gui.register(Button::new((10, 10), Color::from_u32(COLOR_ALWAYS_EMPTY)).with_pos(4, 4));
    gui.register(Button::new((10, 10), Color::from_u32(COLOR_ALWAYS_SOLID)).with_pos(4, 16));
    gui.register(Button::new((10, 10), Color::from_u32(COLOR_BODY)).with_pos(4, 28));
    gui.register(Button::new((10, 10), Color::from_u32(COLOR_BODY2)).with_pos(4, 40));

    redraw(&mask, &mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        window.get_mouse_pos(MouseMode::Pass).map(|mouse| {
            cs.mouse_pos = (mouse.0 as i32, mouse.1 as i32);
            cs.mouse_down = window.get_mouse_down(MouseButton::Left);
        });

        gui.update(&cs);

        if window.is_key_down(Key::Space) {
            redraw(&mask, &mut buffer);
        }

        gui.draw_to_buffer(&mut buffer);

        draw_grid(&mask, 6, &mut buffer, (30, 4), (6, 12));

        window.update_with_buffer(&buffer).unwrap();
    }
}
