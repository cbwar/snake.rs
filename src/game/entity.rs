use std::sync::Arc;

use sdl2::pixels::Color;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Block(pub u32, pub u32);

// #[derive(Debug)]
// pub struct Color {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }
// impl Color {
//     pub fn new(r: u8, g: u8, b: u8) -> Color {
//         Color { r, g, b }
//     }
// }

#[derive(Debug)]
pub struct Snake {
    pub body: Vec<Block>,    // Snake body position on the screen
    pub config: Arc<Config>, // Game config
    direction: Direction,    // Direction the snake is moving
    eat: u32,                // If the snake has eaten the food / grow the snake
}
impl Snake {
    pub fn new(config: Arc<Config>) -> Snake {
        let eat = config.initial_size - 1;
        Snake {
            body: vec![Block(
                config.starting_position.0,
                config.starting_position.1,
            )],
            config,
            direction: Direction::Left,
            eat,
        }
    }

    /// Update the snake position / next tick
    pub fn update(&mut self) {
        let Block(head_x, head_y) = self.body[0];

        // Create a new head based on the current direction of the snake
        let mut new_head: (i32, i32) = match self.direction {
            Direction::Up => (head_x as i32, head_y as i32 - 1),
            Direction::Down => (head_x as i32, head_y as i32 + 1),
            Direction::Left => (head_x as i32 - 1, head_y as i32),
            Direction::Right => (head_x as i32 + 1, head_y as i32),
        };

        // handle the snake going out of bounds
        if new_head.0 >= self.config.grid_size.0 as i32 {
            new_head.0 = 0;
        } else if new_head.0 < 0 {
            new_head.0 = self.config.grid_size.0 as i32 - 1;
        } else if new_head.1 >= self.config.grid_size.1 as i32 {
            new_head.1 = 0;
        } else if new_head.1 < 0 {
            new_head.1 = self.config.grid_size.1 as i32 - 1;
        }

        let head_block = Block(new_head.0 as u32, new_head.1 as u32);

        self.body.insert(0, head_block); // Insert new head

        if self.eat == 0 {
            self.body.pop(); // Remove the last element if the snake has not eaten
        } else {
            self.eat -= 1;
        }
    }

    /// Change the direction of the snake
    /// The snake can't go in the opposite direction
    pub fn cd(&mut self, direction: Direction) {
        if self.direction == Direction::Up && direction == Direction::Down {
            return;
        }
        if self.direction == Direction::Down && direction == Direction::Up {
            return;
        }
        if self.direction == Direction::Left && direction == Direction::Right {
            return;
        }
        if self.direction == Direction::Right && direction == Direction::Left {
            return;
        }
        println!("Snake: Direction changed to {:?}", direction);
        self.direction = direction;
    }

    pub fn eat(&mut self) {
        self.eat += self.config.size_increase_per_food;
        println!("Snake: Eating something");
    }
}

#[derive(Debug)]
pub struct Food {
    pub position: (u32, u32), // Food position on the screen
    pub config: Arc<Config>,  // Game config
}
impl Food {
    pub fn new(config: Arc<Config>) -> Food {
        // Randomize the position of the food
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..config.grid_size.0);
        let y = rng.gen_range(0..config.grid_size.1);
        Food {
            position: (x, y),
            config,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub grid_size: (u32, u32),         // Grid size
    pub grid_resolution: u32,          // Grid resolution in pixels
    pub initial_speed: u32,            // Initial speed of the snake
    pub maximum_speed: u32,            // Maximum speed of the snake
    pub initial_size: u32,             // Initial size of the snake
    pub starting_position: (u32, u32), // Starting position of the snake
    pub speed_increase_per_level: u32, // Speed increase per level
    pub score_per_level: u32,          // Score per level
    pub size_increase_per_food: u32,   // Size increase per food
    pub background_color: Color,       // Background color
    pub snake_color: Color,            // Snake color
    pub food_color: Color,             // Food color
}

impl Config {
    pub fn default() -> Config {
        Config {
            grid_size: (80, 60),
            grid_resolution: 10,
            initial_speed: 100,
            maximum_speed: 30,
            initial_size: 3,
            starting_position: (40, 30),
            speed_increase_per_level: 10,
            score_per_level: 10,
            size_increase_per_food: 1,
            background_color: Color::RGB(0, 0, 0),
            snake_color: Color::RGB(0, 255, 0),
            food_color: Color::RGB(255, 0, 0),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_snake_new() {
        let config = Arc::new(Config {
            initial_size: 4,
            ..Config::default()
        });
        let snake = Snake::new(config);
        assert_eq!(snake.body.len(), 1);
        assert_eq!(snake.direction, Direction::Left);
        assert_eq!(snake.eat, 3);
    }

    #[test]
    fn test_snake_update() {
        let config = Arc::new(Config {
            initial_size: 2,
            ..Config::default()
        });
        let mut snake = Snake::new(config);
        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body.len(), 1);
        snake.update();
        assert_eq!(snake.body[0], Block(39, 30));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body.len(), 2);
        snake.update();
        assert_eq!(snake.body[0], Block(38, 30));
        assert_eq!(snake.body[1], Block(39, 30));
        assert_eq!(snake.body.len(), 2);
    }
    #[test]
    fn test_snake_cd() {
        let config = Arc::new(Config {
            initial_size: 2,
            ..Config::default()
        });
        let mut snake = Snake::new(config);
        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body.len(), 1);
        snake.cd(Direction::Up);
        snake.update();
        assert_eq!(snake.body[0], Block(40, 29));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body.len(), 2);
        snake.cd(Direction::Right);
        snake.update();
        assert_eq!(snake.body[0], Block(41, 29));
        assert_eq!(snake.body[1], Block(40, 29));
        assert_eq!(snake.body.len(), 2);
        snake.cd(Direction::Down);
        snake.update();
        assert_eq!(snake.body[0], Block(41, 30));
        assert_eq!(snake.body[1], Block(41, 29));
        assert_eq!(snake.body.len(), 2);
    }

    #[test]
    fn test_snake_eat() {
        let config = Arc::new(Config {
            grid_size: (10, 10),
            starting_position: (5, 5),
            initial_size: 1,
            size_increase_per_food: 3,
            ..Config::default()
        });
        let mut snake = Snake::new(config);

        assert_eq!(snake.body[0], Block(5, 5));
        snake.eat();
        assert_eq!(snake.eat, 3);
        snake.update();
        assert_eq!(snake.body.len(), 2);
        assert_eq!(snake.body[0], Block(4, 5));
        assert_eq!(snake.body[1], Block(5, 5));
        assert_eq!(snake.eat, 2);
        snake.update();
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.body[0], Block(3, 5));
        assert_eq!(snake.body[1], Block(4, 5));
        assert_eq!(snake.body[2], Block(5, 5));
        assert_eq!(snake.eat, 1);
        snake.update();
        assert_eq!(snake.body.len(), 4);
        assert_eq!(snake.body[0], Block(2, 5));
        assert_eq!(snake.body[1], Block(3, 5));
        assert_eq!(snake.body[2], Block(4, 5));
        assert_eq!(snake.body[3], Block(5, 5));
        assert_eq!(snake.eat, 0);
    }

    #[test]
    fn test_snake_mode_edge() {
        let config = Arc::new(Config {
            grid_size: (10, 10),
            starting_position: (0, 0),
            initial_size: 1,
            ..Config::default()
        });
        let mut snake = Snake::new(config);

        assert_eq!(snake.body[0], Block(0, 0));
        snake.update();
        assert_eq!(snake.body[0], Block(9, 0));
        snake.cd(Direction::Up);
        snake.update();
        assert_eq!(snake.body[0], Block(9, 9));

    }
    #[test]
    fn test_create_food() {
        let config = Arc::new(Config {
            grid_size: (10, 10),
            ..Config::default()
        });
        let food = Food::new(config.clone());
        assert_eq!(food.position.0 >= 0, true);
        assert_eq!(food.position.0 < config.grid_size.0, true);
        assert_eq!(food.position.1 >= 0, true);
        assert_eq!(food.position.1 < config.grid_size.1, true);
    }
    #[test]
    fn test_create_food_with_empty_screen() {
        let config = Arc::new(Config {
            grid_size: (1, 1),
            ..Config::default()
        });
        let food = Food::new(config.clone());
        assert_eq!(food.position.0, 0);
        assert_eq!(food.position.1, 0);
    }
}
