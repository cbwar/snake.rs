mod entity;

use std::{rc::Rc, time::Duration};

use entity::{Food, Game, Grid, Snake};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas};

trait Drawable {
    fn draw(&self, canvas: &mut WindowCanvas);
}
impl Drawable for Snake {
    fn draw(&self, canvas: &mut WindowCanvas) {
        for block in &self.body {
            canvas.set_draw_color(Color::RGB(0, 255, 0));

            let x = block.0 as i32 * 10;
            let y = block.1 as i32 * 10;

            canvas.fill_rect(Rect::new(x, y, 10, 10));
        }
    }
}
impl Drawable for Food {
    fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        let x = self.position.0 as i32 * 10;
        let y = self.position.1 as i32 * 10;
        canvas.fill_rect(Rect::new(x, y, 10, 10));
    }
}

impl Drawable for Game {
    fn draw(&self, canvas: &mut WindowCanvas) {
        self.snake.draw(canvas);
        self.food.draw(canvas);
    }
}

///
/// Main game loop
///
pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = 10;
    let grid = Rc::new(Grid(80, 60));

    let window: sdl2::video::Window = video_subsystem
        .window("rust-sdl2 demo", grid.0 as u32 * size, grid.1 as u32 * size)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut game = Game::new(grid.clone());

    println!("Game started");
    println!("{:?}", game);

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        game.draw(&mut canvas);
        game.tick();

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
