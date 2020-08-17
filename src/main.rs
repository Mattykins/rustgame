#![allow(dead_code)]

mod game;
mod engine;

extern crate sdl2;

use sdl2::{event::Event, image::{InitFlag}, keyboard::Keycode};
use constants::*;
use std::time::{SystemTime};
use engine::*;
use game::*;
use game_scene::GameScene;
use input::InputManager;
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
        .present_vsync()
        .build()
        .unwrap();

    canvas
        .set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("Error setting canvas logical size");

    let texture_creator = canvas.texture_creator();
    let assets = assets::init(&texture_creator).expect("Failed to load assets");

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut scene = GameScene::new(&assets);
    let mut input_manager = InputManager::new();

    let now = SystemTime::now();
    let mut last_tick_t: Option<u128> = None;

    'running: loop {
        let dt: f64;

        match last_tick_t {
            Some(last_t) => {
                let t = now.elapsed().unwrap().as_millis();
                dt = (t as f64 - last_t as f64) / FPS;
                last_tick_t = Some(t);
            },
            None => {
                let t = now.elapsed().unwrap().as_millis();
                dt = 1.0;
                last_tick_t = Some(t);
            }
        }

        canvas.set_draw_color((0, 0, 0));
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

        scene.update(input_manager.collect_game_inputs(), last_tick_t.unwrap(), dt);
        scene.render(&mut canvas);

        canvas.present();
    }
}