mod entity;
mod sound;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use entity::{Block, Direction, Food, Grid, Snake, Style};
use rodio::OutputStream;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas};
use sound::{Sound, SoundSystem};

#[derive(Debug)]
pub struct Game {
    pub grid: Arc<Grid>,          // Grid size
    pub style: Arc<Style>,        // Game style
    pub snd: Option<SoundSystem>, // Sound system
    pub food: Food,               // Food position on the screen
    pub snake: Snake,             // Snake
    pub score: u32,               // Score of the game
}

impl Game {
    pub fn new(grid: Arc<Grid>, style: Arc<Style>, snd: Option<SoundSystem>) -> Game {
        let snake = Snake::new(grid.clone());
        let food = Food::new(grid.clone());

        Game {
            grid,
            style,
            snd,
            food,
            snake,
            score: 0,
        }
    }

    pub fn tick(&mut self) -> u32 {
        self.snake.update(self.grid.clone());

        let level = self.calculate_level();
        let speed = self.calculate_speed();
        let score = self.score;

        println!("Game: Tick (score={score} level={level} speed={speed})");
        if self.snake.body[0] == Block(self.food.position.0, self.food.position.1) {
            self.snake.eat();
            self.food = Food::new(self.grid.clone());
            self.score += 1;
            self.play_snd(Sound::Eat);
        }
        speed
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
        self.score / 10
    }

    ///
    /// Calculate the speed of the game based on the score
    /// The speed is increased every level
    /// The starting speed is 70 and the maximum speed is 10

    fn calculate_speed(&self) -> u32 {
        let level = self.calculate_level();
        let speed = 70 - (level * 10);
        if speed < 10 {
            10
        } else {
            speed
        }
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut WindowCanvas, color: Option<&Color>);
}
impl Drawable for Snake {
    fn draw(&self, canvas: &mut WindowCanvas, color: Option<&Color>) {
        let default_color = Color::RGB(0, 255, 0);
        let color = color.unwrap_or(&default_color);

        for block in &self.body {
            canvas.set_draw_color(*color);

            let x = block.0 as i32 * 10;
            let y = block.1 as i32 * 10;

            canvas.fill_rect(Rect::new(x, y, 10, 10));
        }
    }
}
impl Drawable for Food {
    fn draw(&self, canvas: &mut WindowCanvas, color: Option<&Color>) {
        let default_color = Color::RGB(255, 0, 0);
        let color = color.unwrap_or(&default_color);

        canvas.set_draw_color(*color);

        let x = self.position.0 as i32 * 10;
        let y = self.position.1 as i32 * 10;
        canvas.fill_rect(Rect::new(x, y, 10, 10));
    }
}

impl Drawable for Game {
    fn draw(&self, canvas: &mut WindowCanvas, color: Option<&Color>) {
        let default_color = Color::RGB(0, 0, 0);
        let color = color.unwrap_or(&default_color);
        canvas.set_draw_color(*color);
        canvas.clear();
        self.snake.draw(canvas, Some(&self.style.snake));
        self.food.draw(canvas, Some(&self.style.food));
    }
}

///
/// Main game loop
///
pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let size = 10;
    let grid = Arc::new(Grid(80, 60));

    let window: sdl2::video::Window = video_subsystem
        .window("rust-sdl2 demo", grid.0 as u32 * size, grid.1 as u32 * size)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();

    // Initialize sound system
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Output stream failed to open");
    let snd = sound::SoundSystem::new(stream_handle);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;

    let style = Arc::new(Style::default());
    let game = Arc::new(Mutex::new(Game::new(
        grid.clone(),
        style.clone(),
        Some(snd),
    )));

    println!("Game started");
    println!("{:?}", game);

    let _timer = timer_subsystem.add_timer(0, Box::new(|| game.lock().unwrap().tick()));

    game.lock().unwrap().play_snd(Sound::Start);

    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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
        game.lock().unwrap().draw(&mut canvas, None);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_calculate_game_level() {
        let grid = Arc::new(Grid(10, 10));
        let style = Arc::new(Style::default());
        let mut game = Game::new(grid, style, None);
        assert_eq!(game.calculate_level(), 0);
        game.score = 5;
        assert_eq!(game.calculate_level(), 0);
        game.score = 10;
        assert_eq!(game.calculate_level(), 1);
        game.score = 50;
        assert_eq!(game.calculate_level(), 5);
    }
}
