use minifb::{Key, MouseMode, Window, WindowOptions};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn main() {
    let mut buffer = [0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Conwell's Game of Life - By Liam Garriga",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X32,
            ..Default::default()
        },
    )
    .unwrap();

    let target_fps = 60;
    // Limit to max ~60 fps update rate
    window.set_target_fps(target_fps);

    let mut tick = 0;
    let mut simulation_speed = 10;
    let mut paused = false;
    while window.is_open() {
        let timing = std::time::Instant::now();

        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::Yes) {
            simulation_speed = std::cmp::max(1, simulation_speed - 1);
        } else if window.is_key_pressed(Key::Down, minifb::KeyRepeat::Yes) {
            simulation_speed += 1;
        } else if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        } else if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            buffer.fill(0);
        } else if window.get_mouse_down(minifb::MouseButton::Left) {
            if let Some((y, x)) = window.get_mouse_pos(MouseMode::Discard) {
                buffer[x as usize * WIDTH + y as usize] = u32::MAX;
            }
        }

        if !paused && tick % simulation_speed == 0 {
            simulate(&mut buffer);
        }
        
        tick += 1;
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        if !paused && tick % simulation_speed == 0 {
            eprintln!(
                "framerate: {}",
                (1f32 / (timing.elapsed().as_secs_f32()) / simulation_speed as f32)
            );
        }
    }
}

fn simulate(cells: &mut [u32; WIDTH * HEIGHT]) {
    const N: usize = WIDTH;

    let clone = unsafe { std::mem::transmute_copy::<_, [[u32; WIDTH]; HEIGHT]>(cells) };
    let get = |i, j| {
        clone
            .get(i)
            .and_then(|row: &[u32; 32]| row.get(j).copied())
            .unwrap_or_default()
            >> 31
    };
    for i in 0..N {
        for j in 0..N {
            let neighbors: u32 = get(i, j + 1)
                + get(i, j - 1)
                + get(i + 1, j)
                + get(i - 1, j)
                + get(i + 1, j + 1)
                + get(i + 1, j - 1)
                + get(i - 1, j + 1)
                + get(i - 1, j - 1);

            let cell = &mut cells[i * N + j];
            *cell *= (neighbors == 2) as u32;
            *cell |= (neighbors == 3) as u32 * u32::MAX;
        }
    }
}
