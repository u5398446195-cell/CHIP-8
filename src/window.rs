use minifb::{Key, Scale, Window, WindowOptions};

use crate::chip::Chipe;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub fn window( chip: &mut Chipe) {
    
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "CHIP-8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    ).unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let touches = [
            Key::X,
            Key::Key1,
            Key::Key2,
            Key::Key3,
            Key::Q,
            Key::W,
            Key::E,
            Key::A,
            Key::S,
            Key::D,
            Key::Z,
            Key::C,
            Key::Key4,
            Key::R,
            Key::F,
            Key::V,
        ];
        for (numero, touche) in touches.iter().enumerate() {
            chip.keys[numero] = window.is_key_down(*touche);
        }
        if window.is_key_down(Key::Space) {
            chip.keys[5] = true;
        }

        for _ in 0..15 {
            let opcode = chip.fetch();
            chip.execute(opcode);
        }

        chip.tick_timers();

        for (i, &allume) in chip.display.iter().enumerate() {
            buffer[i] = if allume { 0xFFFFFFFF } else { 0x00000000 };
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
