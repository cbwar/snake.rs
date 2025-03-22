use menu::MenuChoice;
use sdl2::render::WindowCanvas;

mod game;
mod menu;

enum ScreenState {
    Menu,
    Game,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window: sdl2::video::Window = video_subsystem
        .window("Snake game", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: WindowCanvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // start on the menu screen
    let mut screen = ScreenState::Menu;
    let mut continue_game = false;

    loop {
        let choice = match screen {
            ScreenState::Menu => Some(menu::run(&sdl_context, &mut canvas)?),
            ScreenState::Game => {
                game::run(&sdl_context, &mut canvas, continue_game)?;
                screen = ScreenState::Menu;
                None
            }
        };

        match choice {
            Some(MenuChoice::NewGame) => {
                screen = ScreenState::Game;
                continue_game = false;
            }
            Some(MenuChoice::Continue) => {
                screen = ScreenState::Game;
                continue_game = true;
            }
            Some(MenuChoice::Exit) => {
                break;
            }
            None => {}
        }
    }
    Ok(())
}
