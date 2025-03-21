mod game;
mod menu;

enum GameMode {
    Menu,
    Game,
    Exit,
}


fn main() -> Result<(), String> {

    let mut mode = GameMode::Menu;

    loop {
        match mode {
            GameMode::Menu => { 
                mode = menu::run()?;
            }
            GameMode::Game => {
                game::run()?;
                mode = GameMode::Menu;
            }
            GameMode::Exit => {
                break;
            }
        }
    }
    Ok(())
}
