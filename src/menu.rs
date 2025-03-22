use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::sys::Window;
use sdl2::Sdl;
use sdl2::{rect::Rect, render::WindowCanvas};

struct MenuButton {
    text: String,
    rect: Rect,
}

pub enum MenuChoice {
    Play,
    Exit,
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

pub fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<MenuChoice, String> {
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font("resources/COUR.TTF", 20)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    loop {
        let buttons = vec![
            MenuButton::new("Start Game", 100, 100, 200, 75),
            MenuButton::new("Exit", 100, 200, 200, 75),
        ];
        for button in &buttons {
            let mut color = sdl2::pixels::Color::RGBA(255, 255, 255, 200);

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
                Event::MouseButtonDown { x, y, .. } => {
                    for button in &buttons {
                        if button.clicked(x, y) {
                            match button.text.as_str() {
                                "Start Game" => {
                                    return Ok(MenuChoice::Play);
                                }
                                "Exit" => {
                                    return Ok(MenuChoice::Exit);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
