use std::sync::Arc;

use sdl2::pixels::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Block(pub u32, pub u32);

#[derive(Debug, Serialize, Deserialize)]
pub enum FoodType {
    Cherry,
    Banana,
    Apple,
}
impl FoodType {
    pub fn texture(&self) -> String {
        match self {
            FoodType::Cherry => "resources/cherry.png".to_string(),
            FoodType::Banana => "resources/banana.png".to_string(),
            FoodType::Apple => "resources/apple.png".to_string(),
        }
    }
    pub fn score(&self) -> u32 {
        match self {
            FoodType::Cherry => 1,
            FoodType::Banana => 3,
            FoodType::Apple => 5,
        }
    }
    pub fn increase(&self) -> u32 {
        match self {
            FoodType::Cherry => 2,
            FoodType::Banana => 5,
            FoodType::Apple => 8,
        }
    }
    pub fn probality(&self) -> u32 {
        match self {
            FoodType::Cherry => 60,
            FoodType::Banana => 30,
            FoodType::Apple => 10,
        }
    }
}
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Snake {
    pub body: Vec<Block>, // Snake body position on the screen
    direction: Direction, // Direction the snake is moving
    eat: u32,             // If the snake has eaten the food / grow the snake
}
impl Snake {
    pub fn new(config: Arc<Config>) -> Snake {
        let eat = config.initial_size - 1;
        Snake {
            body: vec![Block(
                config.starting_position.0,
                config.starting_position.1,
            )],
            direction: Direction::Left,
            eat,
        }
    }

    /// Update the snake position / next tick
    pub fn update(&mut self, config: Arc<Config>) {
        let Block(head_x, head_y) = self.body[0];

        // Create a new head based on the current direction of the snake
        let mut new_head: (i32, i32) = match self.direction {
            Direction::Up => (head_x as i32, head_y as i32 - 1),
            Direction::Down => (head_x as i32, head_y as i32 + 1),
            Direction::Left => (head_x as i32 - 1, head_y as i32),
            Direction::Right => (head_x as i32 + 1, head_y as i32),
        };

        // handle the snake going out of bounds
        if new_head.0 >= config.grid_size.0 as i32 {
            new_head.0 = 0;
        } else if new_head.0 < 0 {
            new_head.0 = config.grid_size.0 as i32 - 1;
        } else if new_head.1 >= config.grid_size.1 as i32 {
            new_head.1 = 0;
        } else if new_head.1 < 0 {
            new_head.1 = config.grid_size.1 as i32 - 1;
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

    pub fn grow(&mut self, count: u32) {
        println!("Snake: Grow by {}", count);
        self.eat += count;
    }

    pub fn head(&self) -> &Block {
        &self.body[0]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Food {
    pub type_: FoodType, // Food type
    pub position: Block, // Food position on the screen
}
impl Food {
    pub fn new(config: Arc<Config>) -> Food {
        // Random food type
        use rand::Rng;

        let mut rng = rand::rng();
        let random_value = rng.random_range(0..100);
        let type_ = if random_value < FoodType::Cherry.probality() {
            FoodType::Cherry
        } else if random_value < FoodType::Cherry.probality() + FoodType::Banana.probality() {
            FoodType::Banana
        } else {
            FoodType::Apple
        };
        // Randomize the position of the food
        let mut rng = rand::rng();
        let x = rng.random_range(0..config.grid_size.0);
        let y = rng.random_range(0..config.grid_size.1);
        Food {
            type_,
            position: Block(x, y),
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
            background_color: Color::RGB(0, 0, 0),
            snake_color: Color::RGB(0, 255, 0),
            food_color: Color::RGB(255, 0, 0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub food: Option<Food>, // Food position on the screen
    pub snake: Snake,       // Snake
    pub score: u32,         // Score of the game
    pub level: u32,         // Level of the game
    pub speed: u32,         // Speed of the game
    pub game_over: bool,    // Game over flag
}

impl GameState {
    pub fn new(config: Arc<Config>) -> GameState {
        let snake = Snake::new(config.clone());
        let speed = config.initial_speed;
        GameState {
            food: None,
            snake,
            score: 0,
            level: 0,
            speed,
            game_over: false,
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
        assert_eq!(snake.head(), &Block(40, 30));
    }

    #[test]
    fn test_snake_update() {
        let config = Arc::new(Config {
            grid_size: (80, 60),
            starting_position: (40, 30),
            initial_size: 2,
            ..Config::default()
        });
        let mut snake = Snake::new(config.clone());
        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body.len(), 1);
        snake.update(config.clone());
        assert_eq!(snake.body[0], Block(39, 30));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body.len(), 2);
        snake.update(config.clone());
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
        let mut snake = Snake::new(config.clone());
        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body.len(), 1);
        snake.cd(Direction::Up);
        snake.update(config.clone());
        assert_eq!(snake.body[0], Block(40, 29));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body.len(), 2);
        snake.cd(Direction::Right);
        snake.update(config.clone());
        assert_eq!(snake.body[0], Block(41, 29));
        assert_eq!(snake.body[1], Block(40, 29));
        assert_eq!(snake.body.len(), 2);
        snake.cd(Direction::Down);
        snake.update(config.clone());
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
            ..Config::default()
        });
        let mut snake = Snake::new(config.clone());

        assert_eq!(snake.body[0], Block(5, 5));
        snake.grow(3);
        assert_eq!(snake.eat, 3);
        snake.update(config.clone());
        assert_eq!(snake.body.len(), 2);
        assert_eq!(snake.body[0], Block(4, 5));
        assert_eq!(snake.body[1], Block(5, 5));
        assert_eq!(snake.eat, 2);
        snake.update(config.clone());
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.body[0], Block(3, 5));
        assert_eq!(snake.body[1], Block(4, 5));
        assert_eq!(snake.body[2], Block(5, 5));
        assert_eq!(snake.eat, 1);
        snake.update(config.clone());
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
        let mut snake = Snake::new(config.clone());

        assert_eq!(snake.body[0], Block(0, 0));
        snake.update(config.clone());
        assert_eq!(snake.body[0], Block(9, 0));
        snake.cd(Direction::Up);
        snake.update(config.clone());
        assert_eq!(snake.body[0], Block(9, 9));
    }
    #[test]
    fn test_create_food() {
        let config = Arc::new(Config {
            grid_size: (10, 10),
            ..Config::default()
        });
        let food = Food::new(config.clone());
        assert_eq!(food.position.0 < config.grid_size.0, true);
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
