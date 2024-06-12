use std::sync::atomic;

use game_of_life::Chunk;
use minifb::{Key, MouseMode, Window, WindowOptions};

const N: usize = 32;

// SAFETY: AtomicU32 is repr(transparent) with u32
static BUFFER: [atomic::AtomicU32; N * N] = unsafe { std::mem::transmute([0u32; N * N]) };

fn main() {
    let mut window = Window::new(
        "Conwell's Game of Life - By Liam Garriga",
        N,
        N,
        WindowOptions {
            scale: minifb::Scale::X32,
            ..Default::default()
        },
    )
    .unwrap();

    let window_fps = 500;
    window.set_target_fps(window_fps);

    let (paused_reader, paused_updater) = single_value_channel::channel_starting_with(false);
    let (simfps_reader, simfps_updater) = single_value_channel::channel_starting_with(30);
    // let (simulation_reader, simulation_updater) = single_value_channel::channel_starting_with();

    std::thread::spawn(|| simulation_update(paused_reader, simfps_reader));
    window_update(window, window_fps, paused_updater, simfps_updater);

    std::process::exit(0);
}

fn window_update(mut window: minifb::Window, window_fps: usize, paused_updater: single_value_channel::Updater<bool>, simfps_updater: single_value_channel::Updater<usize>) {
    let mut paused = false;
    let mut simulation_fps = 30;
    
    while window.is_open() {
        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::Yes) {
            simulation_fps = std::cmp::min(window_fps, simulation_fps + 1);
            let _ = simfps_updater.update(simulation_fps);
        } else if window.is_key_pressed(Key::Down, minifb::KeyRepeat::Yes) {
            simulation_fps = std::cmp::max(1, simulation_fps - 1);
            let _ = simfps_updater.update(simulation_fps);
        } else if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
            let _ = paused_updater.update(paused);
        } else if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            BUFFER
                .iter()
                .for_each(|i| i.store(0, atomic::Ordering::Release));
        } else if window.get_mouse_down(minifb::MouseButton::Left) {
            if let Some((y, x)) = window.get_mouse_pos(MouseMode::Discard) {
                BUFFER[x as usize * N + y as usize].store(u32::MAX, atomic::Ordering::Release);
            }
        }

        // SAFETY: AtomicU32 is repr(transparent) with u32;
        //         We only read the buffer, and don't care if it's partially incorrect
        window
            .update_with_buffer(
                unsafe { std::mem::transmute::<&[atomic::AtomicU32], &[u32]>(BUFFER.as_slice()) },
                N,
                N,
            )
            .unwrap();
    }
}

fn simulation_update(mut paused: single_value_channel::Receiver<bool>, mut simulation_fps: single_value_channel::Receiver<usize>) {
    let mut frametiming = std::time::Instant::now();
    let mut prev_time = std::time::Instant::now();
    loop {
        let simulation_fps = *simulation_fps.latest();

        let delta = prev_time.elapsed();
        let rate = std::time::Duration::from_secs_f32(1. / simulation_fps as f32);
        if delta < rate {
            let sleep_time = rate - delta;
            std::thread::sleep(sleep_time);
        }

        prev_time = std::time::Instant::now();

        if *paused.latest() {
            simulate(&BUFFER);
            let elapsed = frametiming.elapsed();
            eprintln!("{:.0}fps | {elapsed:.2?}", (1f32 / elapsed.as_secs_f32()));
            frametiming = std::time::Instant::now();
        }
    }
}

fn simulate(cells: &[atomic::AtomicU32; N * N]) {
    // SAFETY: AtomicU32 is repr(transparent) with u32;
    let clone = unsafe { std::mem::transmute_copy::<_, [u32; N * N]>(cells) }; // copy
    let get = |i| clone.get(i % (N * N)).copied().unwrap_or_default() >> 31;

    for (i, cell) in cells.iter().enumerate() {
        let neighbors: u32 = get(i + 1)
            + get(i.wrapping_sub(1))
            + get(i + N)
            + get(i.wrapping_sub(N))
            + get(i.wrapping_sub(N - 1))
            + get(i.wrapping_sub(N + 1))
            + get(i + N + 1)
            + get(i + N - 1);
        let mut c = cell.load(atomic::Ordering::Acquire);
        c *= (neighbors == 2) as u32;
        c |= (neighbors == 3) as u32 * u32::MAX;
        cell.store(c, atomic::Ordering::Release);
    }
}
