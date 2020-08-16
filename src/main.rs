#![allow(dead_code)]

mod scene;
mod game_scene;
mod geometry;
mod asset_cache;
mod sprite;
mod input;
mod constants;

extern crate sdl2;

use sdl2::{event::Event, image::{InitFlag}, keyboard::Keycode};
use constants::*;
use input::InputManager;
use game_scene::GameScene;
use std::time::{Duration, SystemTime};
use scene::Scene;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    canvas
        .set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Error setting canvas logical size");

    let texture_creator = canvas.texture_creator();
    let assets = asset_cache::init(&texture_creator).expect("Failed to load assets");

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut scene = GameScene::new(&assets);
    let mut input_manager = InputManager::new();

    let now = SystemTime::now();
    let mut last_tick_t: Option<f32> = None;

    'running: loop {
        let dt: f32;

        match last_tick_t {
            Some(last_t) => {
                let t = now.elapsed().unwrap().as_millis() as f32;
                dt = (t - last_t) / FPS;
                last_tick_t = Some(t);
            },
            None => {
                let t = now.elapsed().unwrap().as_millis() as f32;
                dt = 1.0;
                last_tick_t = Some(t);
            }
        }

        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode, .. } => {
                    input_manager.process_keydown(keycode.unwrap())
                }
                Event::KeyUp { keycode, .. } => {
                    input_manager.process_keyup(keycode.unwrap())
                }
                _ => {}
            }
        }

        scene.update(input_manager.get_game_inputs(), dt);
        scene.render(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}