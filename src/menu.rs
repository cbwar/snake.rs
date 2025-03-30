use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;
use sdl2::{rect::Rect, render::WindowCanvas};
use snake::savegame::savegame_exists;

struct MenuButton {
    id: u32,
    text: String,
    rect: Rect,
    enabled: bool,
}

pub enum MenuChoice {
    Continue,
    NewGame,
    Exit,
}

impl MenuButton {
    pub fn new(id: u32, text: &str, x: i32, y: i32, w: u32, h: u32, enabled: bool) -> Self {
        Self {
            id,
            text: text.to_string(),
            rect: Rect::new(x, y, w, h),
            enabled,
        }
    }
    fn clicked(&self, x: i32, y: i32) -> bool {
        self.rect.contains_point((x, y)) && self.enabled
    }
    fn hovered(&self, x: i32, y: i32) -> bool {
        if self.rect.contains_point((x, y)) && self.enabled {
            true
        } else {
            false
        }
    }
}

pub fn run(sdl_context: &Sdl, canvas: &mut WindowCanvas) -> Result<MenuChoice, String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("resources/COUR.TTF", 20)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();

    loop {
        let buttons = vec![
            MenuButton::new(10, "Continue Game", 100, 100, 200, 75, savegame_exists()),
            MenuButton::new(20, "New Game", 100, 200, 200, 75, true),
            MenuButton::new(30, "High scores", 100, 300, 200, 75, false),
            MenuButton::new(90, "Exit", 100, 400, 200, 75, true),
        ];
        for button in &buttons {
            let mut color = sdl2::pixels::Color::RGBA(255, 255, 255, 200);

            if button.hovered(event_pump.mouse_state().x(), event_pump.mouse_state().y()) {
                color = sdl2::pixels::Color::RGBA(255, 255, 90, 200);
            } 
            
            if button.enabled == false {
                color = sdl2::pixels::Color::RGBA(90, 90, 90, 200);
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
                    return Ok(MenuChoice::Exit);
                }

                Event::MouseButtonDown { x, y, .. } => {
                    for button in &buttons {
                        if button.clicked(x, y) {
                            match button.id {
                                10 => {
                                    return Ok(MenuChoice::Continue);
                                }
                                20 => {
                                    return Ok(MenuChoice::NewGame);
                                }
                                90 => {
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
