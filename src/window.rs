use minifb::{Key, Scale, Window, WindowOptions};

use crate::chip::Chipe;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub fn window( chip: &mut Chipe) {
    
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Ma fenêtre Rust",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    ).unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Tous les pixels deviennent rouges
        //for pixel in buffer.iter_mut() {
           // *pixel = 0xFF0000;
        //}
        for i in 0..10 {
             let opcode = chip.fetch();
            chip.execute(opcode);
        }
        for (i, &allume) in chip.display.iter().enumerate() {
            buffer[i] = if allume { 0xFFFFFFFF } else { 0x00000000 };
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}