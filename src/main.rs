use minifb::{Key, MouseMode, Window, WindowOptions};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn main() {
    let mut buffer = [0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X32,
            ..Default::default()
        }
    ).unwrap();
    
    let mut target_fps = 60;
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
        } else if window.get_mouse_down(minifb::MouseButton::Left) {
            if let Some((y, x)) = window.get_mouse_pos(MouseMode::Discard) {
                buffer[x as usize * WIDTH + y as usize] = u32::MAX;
            }
        }
        
        if !paused && tick % simulation_speed == 0 {
            simulate(&mut buffer);
            eprintln!("compute: {:?}", timing.elapsed());
        }

        tick += 1;
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        if !paused {
            eprintln!("framerate: {}", (1f32 / timing.elapsed().as_secs_f32()) / simulation_speed as f32);
            eprintln!("simulation speed: {}", simulation_speed);
        }
    }
}

fn simulate(cells: &mut [u32; WIDTH * HEIGHT]) {
    const N: usize = WIDTH;

    let clone = *cells;
    
    for i in 0..N {
        for j in 0..N {
            let mut neighbors = 0;
            if j > 0 {
                neighbors += clone[i * N + j-1] >> 31;

                if i > 0 {
                    neighbors += clone[(i-1) * N + j-1] >> 31;
                }
                if i < N-1 {
                    neighbors += clone[(i+1) * N + j-1] >> 31;
                }
            }
            if j < N-1 {
                neighbors += clone[i * N + j+1] >> 31;

                if i > 0 {
                    neighbors += clone[(i-1) * N + j+1] >> 31;
                }
                if i < N-1 {
                    neighbors += clone[(i+1) * N + j+1] >> 31;
                }
            }
            if i > 0 {
                neighbors += clone[(i-1) * N + j] >> 31;
            }
            if i < N-1 {
                neighbors += clone[(i+1) * N + j] >> 31;
            }
            let cell = &mut cells[i * N + j];
            if *cell != 0 {
                if neighbors < 2 || neighbors > 3 {
                    *cell = 0;
                } else {
                    *cell = u32::MAX;
                }
            } else if neighbors == 3 {
                *cell = u32::MAX;
            }
        }
    }
}