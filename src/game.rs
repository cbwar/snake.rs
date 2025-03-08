mod entity;

use std::{rc::Rc, sync::{Arc, Mutex}, time::Duration};

use entity::{Food, Game, Grid, Snake, Style};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas};

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

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut style = Arc::new(Style::default());
    let mut game = Arc::new(Mutex::new(Game::new(grid.clone(), style.clone())));

    println!("Game started");
    println!("{:?}", game);

    let _timer = timer_subsystem.add_timer(
        0,
        Box::new(|| {
            game.lock().unwrap().tick()
        }),
    );

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
