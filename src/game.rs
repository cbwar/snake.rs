use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::OutputStream;
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    Sdl,
};
use snake::{
    entity::{Config, Direction, Food, GameState, Snake},
    savegame::{load_game_state, save_game_state},
};
use snake::{
    savegame::delete_save,
    sound::{Sound, SoundSystem},
};

///
/// TODO: Handle collision with the snake body
/// TODO: The snake grows 3 blocks when eating food
/// TODO: Add walls to the game (map?)
/// TODO: Handle collision with the walls
/// TODO: Game over screen
/// TODO: Pause screen
/// TODO: Restart game
/// TODO: Save high score
/// TODO: Game menu screed
///

pub struct Game {
    pub state: GameState,         // Game state
    pub config: Arc<Config>,      // Game config
    pub snd: Option<SoundSystem>, // Sound system
}

impl Game {
    fn new(config: Arc<Config>, snd: Option<SoundSystem>, continue_game: bool) -> Self {
        let mut state = GameState::new(config.clone());
        if continue_game == true {
            state = load_game_state().unwrap_or(state);
        }
        Game {
            state,
            config: Arc::clone(&config),
            snd,
        }
    }

    pub fn setup(&mut self) {
        println!("Game: Setup");
        println!("Game: Config={:?}", self.config);
        self.play_snd(Sound::Start);
        self.create_food();
    }

    ///
    /// Handle food collision
    ///
    fn handle_food_eat(&mut self) {
        if !self.eating_food() {
            return;
        }

        if let Some(ref food) = self.state.food {
            self.state.snake.grow(food.type_.increase());
            self.state.score += food.type_.score();
            self.play_snd(Sound::Eat);
            self.state.level = self.calculate_level();
            self.state.speed = self.calculate_speed();

            self.create_food();
        }
    }

    /// Handle snake collision
    ///
    /// The snake can't collide with itself
    fn handle_collisions(&mut self) {
        let head = self.state.snake.head();
        for block in self.state.snake.body.iter().skip(1) {
            if head == block {
                println!("Game: Collision with the snake body");
                self.state.game_over = true;
                self.play_snd(Sound::GameOver);
                delete_save().expect("Failed to delete save game");
            }
        }
    }

    pub fn tick(&mut self) -> u32 {
        self.state.snake.update(self.config.clone());

        println!(
            "Game: Tick (score={} level={} speed={})",
            self.state.score, self.state.level, self.state.speed
        );

        self.handle_food_eat();
        self.handle_collisions();

        if self.state.game_over {
            return 0;
        }
        self.state.speed
    }

    pub fn keypress(&mut self, key: Keycode) {
        match key {
            Keycode::Up => self.state.snake.cd(Direction::Up),
            Keycode::W => self.state.snake.cd(Direction::Up),
            Keycode::Down => self.state.snake.cd(Direction::Down),
            Keycode::S => self.state.snake.cd(Direction::Down),
            Keycode::Left => self.state.snake.cd(Direction::Left),
            Keycode::A => self.state.snake.cd(Direction::Left),
            Keycode::Right => self.state.snake.cd(Direction::Right),
            Keycode::D => self.state.snake.cd(Direction::Right),
            _ => {}
        }
    }

    pub fn create_food(&mut self) {
        self.state.food = Some(Food::new(self.config.clone()));
    }

    pub fn eating_food(&mut self) -> bool {
        match self.state.food {
            None => false,
            Some(ref food) => self.state.snake.head() == &food.position,
        }
    }

    /// Play a sound
    pub fn play_snd(&self, sound: Sound) {
        println!("Game: Play sound {:?}", sound);
        match self.snd {
            None => return,
            Some(ref snd) => snd.play_snd(sound).expect("Failed to play sound"),
        }
    }

    ///
    /// Calculate the level of the game based on the score
    /// A level is gained every 10 points
    ///
    fn calculate_level(&self) -> u32 {
        self.state.score / self.config.score_per_level
    }

    ///
    /// Calculate the speed of the game based on the score
    /// The speed is increased every level
    ///
    fn calculate_speed(&self) -> u32 {
        let level = self.calculate_level();
        let speed: i32 = self.config.initial_speed as i32
            - (level as i32 * self.config.speed_increase_per_level as i32);
        if speed < self.config.maximum_speed as i32 {
            self.config.maximum_speed
        } else {
            speed as u32
        }
    }

    ///
    /// Get the status text of the game
    ///
    fn get_status_text(&self) -> String {
        format!("Score: {} Level: {}", self.state.score, self.state.level)
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut WindowCanvas, config: Arc<Config>);
}
impl Drawable for Snake {
    fn draw(&self, canvas: &mut WindowCanvas, config: Arc<Config>) {
        let color = &config.snake_color;

        for block in &self.body {
            canvas.set_draw_color(*color);

            let x = block.0 as i32 * config.grid_resolution as i32;
            let y = block.1 as i32 * config.grid_resolution as i32;

            let _ = canvas.fill_rect(Rect::new(
                x,
                y,
                config.grid_resolution,
                config.grid_resolution,
            ));
        }
    }
}
impl Drawable for Food {
    fn draw(&self, canvas: &mut WindowCanvas, config: Arc<Config>) {
        let x = self.position.0 as i32 * config.grid_resolution as i32;
        let y = self.position.1 as i32 * config.grid_resolution as i32;
        let t = canvas.texture_creator();
        let filename = self.type_.texture();
        let tex = t.load_texture(filename).unwrap();
        let r = Rect::new(x, y, config.grid_resolution, config.grid_resolution);
        canvas.copy(&tex, None, r).unwrap();
    }
}

impl Drawable for Game {
    fn draw(&self, canvas: &mut WindowCanvas, config: Arc<Config>) {
        let color = &config.background_color;
        canvas.set_draw_color(*color);
        canvas.clear();
        self.state.snake.draw(canvas, config.clone());
        if let Some(ref food) = self.state.food {
            food.draw(canvas, config.clone());
        }
    }
}

// impl From<Color> for sdl2::pixels::Color {
//     fn from(color: Color) -> sdl2::pixels::Color {
//         sdl2::pixels::Color::RGB(color.r, color.g, color.b)
//     }
// }

///
/// Main game loop
///
pub fn run(
    sdl_context: &Sdl,
    canvas: &mut WindowCanvas,
    continue_game: bool,
) -> Result<(), String> {
    // let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let mut font = ttf_context.load_font("resources/COUR.TTF", 20)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let game_config = Config {
        initial_speed: 100,
        initial_size: 8,
        score_per_level: 10,
        grid_resolution: 10,
        ..Config::default()
    };
    let game_config = Arc::new(game_config);

    // let window: sdl2::video::Window = video_subsystem
    //     .window(
    //         "Snake game",
    //         game_config.grid_size.0 as u32 * game_config.grid_resolution,
    //         game_config.grid_size.1 as u32 * game_config.grid_resolution,
    //     )
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Initialize sound system
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Output stream failed to open");
    let snd = SoundSystem::new(stream_handle);

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    // let mut i = 0;

    let game = Arc::new(Mutex::new(Game::new(
        game_config.clone(),
        Some(snd),
        continue_game,
    )));
    game.lock().unwrap().setup();

    let _timer;
    if !game.lock().unwrap().state.game_over {
        _timer = timer_subsystem.add_timer(0, Box::new(|| game.lock().unwrap().tick()));
    }

    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if !game.lock().unwrap().state.game_over {
                        save_game_state(&game.lock().unwrap().state)
                            .expect("Failed to save game state");
                    }
                    break 'running;
                }
                _ => {
                    if let Event::KeyDown { keycode, .. } = event {
                        if let Some(key) = keycode {
                            game.lock().unwrap().keypress(key);
                        }
                    }
                }
            }
        }
        // The rest of the game loop goes here...
        game.lock().unwrap().draw(canvas, game_config.clone());

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(game.lock().unwrap().get_status_text().as_str())
            .blended(Color::RGBA(255, 255, 255, 200))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let dest = Rect::new(0, 0, surface.width(), surface.height());
        canvas.copy(&texture, None, Some(dest))?;

        if game.lock().unwrap().state.game_over {
            let surface = font
                .render("Game Over")
                .blended(Color::RGBA(255, 255, 255, 200))
                .map_err(|e| e.to_string())?;
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let dest = Rect::new(380, 280, surface.width(), surface.height());
            canvas.copy(&texture, None, Some(dest))?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_speed() {
        let config = Arc::new(Config {
            initial_speed: 100,
            maximum_speed: 10,
            speed_increase_per_level: 10,
            score_per_level: 10,
            ..Config::default()
        });
        let game = Game {
            config: config.clone(),
            snd: None,
            state: GameState {
                food: None,
                snake: Snake::new(config.clone()),
                score: 10,
                level: 0,
                speed: 100,
                game_over: false,
            },
        };
        assert_eq!(game.calculate_speed(), 90);
        let game = Game {
            config: config.clone(),
            snd: None,
            state: GameState {
                food: None,
                snake: Snake::new(config.clone()),
                score: 100,
                level: 0,
                speed: 100,
                game_over: false,
            },
        };
        assert_eq!(game.calculate_speed(), 10);
        let game = Game {
            config: config.clone(),
            snd: None,
            state: GameState {
                food: None,
                snake: Snake::new(config.clone()),
                score: 1000,
                level: 0,
                speed: 100,
                game_over: false,
            },
        };
        assert_eq!(game.calculate_speed(), 10);
    }

    // #[test]
    // fn test_calculate_game_level() {
    //     let grid = Arc::new(Grid(10, 10));
    //     let style = Arc::new(Style::default());
    //     let mut game = Game::new(grid, style, None);
    //     assert_eq!(game.calculate_level(), 0);
    //     game.score = 5;
    //     assert_eq!(game.calculate_level(), 0);
    //     game.score = 10;
    //     assert_eq!(game.calculate_level(), 1);
    //     game.score = 50;
    //     assert_eq!(game.calculate_level(), 5);
    // }
}
