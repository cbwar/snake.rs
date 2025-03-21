use std::time::Duration;

use crate::GameMode;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{rect::Rect, render::WindowCanvas};

struct MenuButton {
    text: String,
    rect: Rect,
}

impl MenuButton {
    pub fn new(text: &str, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            text: text.to_string(),
            rect: Rect::new(x, y, w, h),
        }
    }
    fn clicked(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y))
    }
    fn hovered(&self, x: i32, y: i32) -> bool {
        if self.rect.contains_point((x, y)) {
            true
        } else {
            false
        }
    }
}

pub fn run() -> Result<GameMode, String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let timer_subsystem = sdl_context.timer().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut font = ttf_context.load_font("resources/COUR.TTF", 20)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let window: sdl2::video::Window = video_subsystem
        .window("Snake game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Initialize sound system
    // let (_stream, stream_handle) =
    //     OutputStream::try_default().expect("Output stream failed to open");
    // let snd = sound::SoundSystem::new(stream_handle);

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;

    let mut mode: GameMode = GameMode::Menu;
    'running: loop {
        let buttons = vec![
            MenuButton::new("Start Game", 100, 100, 200, 75),
            MenuButton::new("Exit", 100, 200, 200, 75),
        ];
        for button in &buttons {

            let mut color =sdl2::pixels::Color::RGBA(255, 255, 255, 200);

            if button.hovered(event_pump.mouse_state().x(), event_pump.mouse_state().y()) {
                color = sdl2::pixels::Color::RGBA(255, 255, 90, 200);
            }

            let surface = font
                .render(button.text.as_str())
                .blended(color)
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            canvas.copy(&texture, None, button.rect)?;
        }



        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    mode = GameMode::Exit;
                    break 'running;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    for button in &buttons {
                        if button.clicked(x, y) {
                            match button.text.as_str() {
                                "Start Game" => {
                                    mode = GameMode::Game;
                                    break 'running;
                                }
                                "Exit" => {
                                    mode = GameMode::Exit;
                                    break 'running;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit { .. }
        //         | Event::KeyDown {
        //             keycode: Some(Keycode::Escape),
        //             ..
        //         } => {
        //             break 'running;
        //         }
        //     }
        // }
        // The rest of the game loop goes here...

        // render a surface, and convert it to a texture bound to the canvas
        // let surface = font
        //     .render(game.lock().unwrap().get_status_text().as_str())
        //     .blended(Color::RGBA(255, 255, 255, 200))
        //     .map_err(|e| e.to_string())?;
        //     .create_texture_from_surface(&surface)
        //     .map_err(|e| e.to_string())?;

        // let dest = Rect::new(0, 0, surface.width(), surface.height());

        // canvas.copy(&texture, None, Some(dest))?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(mode)

    // println!("Menu");
    // println!("1. Start Game");
    // println!("2. Exit");
    // let mut input = String::new();
    // std::io::stdin()
    //     .read_line(&mut input)
    //     .map_err(|e| e.to_string())?;

    // match input.trim() {
    //     "1" => Ok(GameMode::Game),
    //     "2" => Ok(GameMode::Exit),
    //     _ => Ok(GameMode::Menu),
    // }
}
