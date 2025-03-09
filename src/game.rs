mod entity;
mod sound;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use entity::{Config, Direction, Food, Snake};
use rodio::OutputStream;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas};
use sound::{Sound, SoundSystem};

#[derive(Debug)]
pub struct Game {
    pub config: Arc<Config>,      // Game config
    pub snd: Option<SoundSystem>, // Sound system
    pub food: Option<Food>,       // Food position on the screen
    pub snake: Snake,             // Snake
    pub score: u32,               // Score of the game
    pub level: u32,               // Level of the game
    pub speed: u32,               // Speed of the game
}
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
impl Game {
    pub fn new(config: Config, snd: Option<SoundSystem>) -> Game {
        let config = Arc::new(config);
        let snake = Snake::new(config.clone());
        let speed = config.initial_speed;
        Game {
            config,
            snd,
            food: None,
            snake,
            score: 0,
            level: 0,
            speed,
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
        self.level = self.calculate_level();
        self.speed = self.calculate_speed();
        self.snake.eat();
        self.create_food();
        self.score += 1;
        self.play_snd(Sound::Eat);
    }

    /// Handle snake collision
    ///
    /// The snake can't collide with itself
    fn handle_collisions(&mut self) {
        let head = self.snake.head();
        for block in self.snake.body.iter().skip(1) {
            if head == block {
                println!("Game: Collision with the snake body");
                self.play_snd(Sound::GameOver);
            }
        }
    }

    pub fn tick(&mut self) -> u32 {
        self.snake.update();

        println!(
            "Game: Tick (score={} level={} speed={})",
            self.score, self.level, self.speed
        );

        self.handle_food_eat();
        self.handle_collisions();

        self.speed
    }

    pub fn keypress(&mut self, key: Keycode) {
        match key {
            Keycode::Up => self.snake.cd(Direction::Up),
            Keycode::W => self.snake.cd(Direction::Up),
            Keycode::Down => self.snake.cd(Direction::Down),
            Keycode::S => self.snake.cd(Direction::Down),
            Keycode::Left => self.snake.cd(Direction::Left),
            Keycode::A => self.snake.cd(Direction::Left),
            Keycode::Right => self.snake.cd(Direction::Right),
            Keycode::D => self.snake.cd(Direction::Right),
            _ => {}
        }
    }

    pub fn create_food(&mut self) {
        self.food = Some(Food::new(self.config.clone()));
    }

    pub fn eating_food(&mut self) -> bool {
        match self.food {
            None => false,
            Some(ref food) => self.snake.head() == &food.position,
        }
    }

    /// Play a sound
    pub fn play_snd(&self, sound: Sound) {
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
        self.score / self.config.score_per_level
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
        format!("Score: {} Level: {}", self.score, self.level)
    }

}

trait Drawable {
    fn draw(&self, canvas: &mut WindowCanvas);
}
impl Drawable for Snake {
    fn draw(&self, canvas: &mut WindowCanvas) {
        let color = &self.config.snake_color;

        for block in &self.body {
            canvas.set_draw_color(*color);

            let x = block.0 as i32 * self.config.grid_resolution as i32;
            let y = block.1 as i32 * self.config.grid_resolution as i32;

            canvas.fill_rect(Rect::new(
                x,
                y,
                self.config.grid_resolution,
                self.config.grid_resolution,
            ));
        }
    }
}
impl Drawable for Food {
    fn draw(&self, canvas: &mut WindowCanvas) {
        let color = &self.config.food_color;

        canvas.set_draw_color(*color);

        let x = self.position.0 as i32 * self.config.grid_resolution as i32;
        let y = self.position.1 as i32 * self.config.grid_resolution as i32;

        canvas.fill_rect(Rect::new(
            x,
            y,
            self.config.grid_resolution,
            self.config.grid_resolution,
        ));
    }
}

impl Drawable for Game {
    fn draw(&self, canvas: &mut WindowCanvas) {
        let color = &self.config.background_color;
        canvas.set_draw_color(*color);
        canvas.clear();
        self.snake.draw(canvas);
        if let Some(ref food) = self.food {
            food.draw(canvas);
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
pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut font = ttf_context.load_font("resources/COUR.TTF", 20)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let game_config = Config {
        initial_speed: 100,
        initial_size: 8,
        score_per_level: 1,
        size_increase_per_food: 10,
        grid_resolution: 10,
        ..Config::default()
    };

    let window: sdl2::video::Window = video_subsystem
        .window(
            "Snake game",
            game_config.grid_size.0 as u32 * game_config.grid_resolution,
            game_config.grid_size.1 as u32 * game_config.grid_resolution,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Initialize sound system
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Output stream failed to open");
    let snd = sound::SoundSystem::new(stream_handle);

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;

    let game = Arc::new(Mutex::new(Game::new(game_config, Some(snd))));
    game.lock().unwrap().setup();

    let _timer = timer_subsystem.add_timer(0, Box::new(|| game.lock().unwrap().tick()));

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
                } => break 'running,
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
        game.lock().unwrap().draw(&mut canvas);


        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(game.lock().unwrap().get_status_text().as_str())
            .blended(Color::RGBA(255, 255,255, 200))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
    
        let dest = Rect::new(0, 0, surface.width(), surface.height());
    
        canvas.copy(&texture, None, Some(dest))?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

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
            food: None,
            snake: Snake::new(config.clone()),
            score: 10,
            level: 0,
            speed: 100,
        };
        assert_eq!(game.calculate_speed(), 90);
        let game = Game {
            config: config.clone(),
            snd: None,
            food: None,
            snake: Snake::new(config.clone()),
            score: 100,
            level: 0,
            speed: 100,
        };
        assert_eq!(game.calculate_speed(), 10);
        let game = Game {
            config: config.clone(),
            snd: None,
            food: None,
            snake: Snake::new(config.clone()),
            score: 1000,
            level: 0,
            speed: 100,
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
