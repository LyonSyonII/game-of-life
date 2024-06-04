use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X32,
            ..Default::default()
        }
    ).unwrap();
    
    let mut target_fps = 0;
    // Limit to max ~60 fps update rate
    window.set_target_fps(target_fps);
    
    let mut pos = 0usize;
    while window.is_open() {
        let timing = std::time::Instant::now();

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                buffer[i * WIDTH + j] = ((i * WIDTH + j) == pos) as u32 * u32::MAX;
            }
        }
        pos += 1;
        if pos > WIDTH * HEIGHT {
            pos = 0;
        }

        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
            target_fps += 60;
            window.set_target_fps(target_fps);
        } else if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
            target_fps = std::cmp::max(60, target_fps - 60);
            window.set_target_fps(target_fps);
        }
        
        eprintln!("compute: {:?}", timing.elapsed());
        
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        eprintln!("framerate: {}", 1f32 / timing.elapsed().as_secs_f32());
    }
}

struct SyncUnsafeCell<T>(std::cell::UnsafeCell<T>);

unsafe impl<T> Send for SyncUnsafeCell<T> where T: Send {}
unsafe impl<T> Sync for SyncUnsafeCell<T> where T: Sync {}

// enviar un &'static [u8]
// conté una tupla amb els arguments passats a log

/* #[macro_export]
macro_rules! log {
    ($fmt:literal) => {
        
    };
    ($fmt:literal, $($arg:expr)+, $($ty:ty)+) => {
        {
            static THING: SyncUnsafeCell<($(Option<$ty>),+)> = SyncUnsafeCell(std::cell::UnsafeCell::new(($(Option::<$ty>::None),+)));
            
            unsafe { 
                *THING.0.get() = ($(Some($arg)),+);
            }
            let format: fn() = || {
                let thing = unsafe { *THING.0.get() };
                println!($fmt);
            };
            log.send(format).unwrap();
        }
    }
} */