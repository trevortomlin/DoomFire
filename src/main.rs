extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn spread_fire(buffer: &mut [usize], from: usize) {

    if buffer[from] == 0 { buffer[from - WIDTH] = 0; return; }
    
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(-3, 3) as usize & 3; 

    let to = from - WIDTH - rand + 1;
   
    buffer[to] = buffer[from] - (rand & 1); 

}

const colors: [u32; 36] = [0x070707,
                           0x1f0707,
                           0x2f0f07,
                           0x470f07,
                           0x571707,
                           0x671f07,
                           0x771f07,
                           0x8f2707,
                           0x9f2f07,
                           0xaf3f07,
                           0xbf4707,
                           0xc74707,
                           0xDF4F07,
                           0xDF5707,
                           0xDF5707,
                           0xD75F07,
                           0xD7670F,
                           0xcf6f0f,
                           0xcf770f,
                           0xcf7f0f,
                           0xCF8717,
                           0xC78717,
                           0xC78F17,
                           0xC7971F,
                           0xBF9F1F,
                           0xBF9F1F,
                           0xBFA727,
                           0xBFA727,
                           0xBFAF2F,
                           0xB7AF2F,
                           0xB7B72F,
                           0xB7B737,
                           0xCFCF6F,
                           0xDFDF9F,
                           0xEFEFC7,
                           0xFFFFFF];

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut fire_buffer: [usize; WIDTH*HEIGHT] = [0; WIDTH*HEIGHT];

    for x in WIDTH*HEIGHT-WIDTH..WIDTH*HEIGHT-1 {
        fire_buffer[x] = 35
    }

    let mut window = Window::new(
        "Doomfire",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
    
        for x in 0..WIDTH {
            for y in 2..HEIGHT {
                spread_fire(&mut fire_buffer, y * WIDTH + x);
            }
        }

        for (i, color_index) in fire_buffer.iter().cloned().enumerate() {
            buffer[i] = colors[color_index];
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
