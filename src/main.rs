use menu::MenuChoice;
use sdl2::render::WindowCanvas;

mod game;
mod menu;

enum ScreenState {
    Menu,
    Game,
}

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // let timer_subsystem = sdl_context.timer().unwrap();

    let window: sdl2::video::Window = video_subsystem
        .window("Snake game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
  
    // start on the menu screen
    let mut screen = ScreenState::Menu;
    let mut choice: Option<MenuChoice> = None;
    let mut continue_game = false;

    loop {
        match screen {
            ScreenState::Menu => {
                choice = Some(menu::run(&sdl_context, &mut canvas)?);
            }
            ScreenState::Game => {
                game::run(&sdl_context, &mut canvas, continue_game)?;
                screen = ScreenState::Menu;
                choice = None;
            }
        }

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
