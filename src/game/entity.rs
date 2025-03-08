use std::sync::Arc;

use sdl2::{keyboard::Keycode, pixels::Color};

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Block(pub i32, pub i32);

#[derive(Debug, PartialEq)]
pub struct Grid(pub i32, pub i32);

#[derive(Debug, PartialEq)]
pub struct Style {
    pub background: Color,
    pub snake: Color,
    pub food: Color,
}
impl Style {
    pub fn default() -> Style {
        Style {
            background: Color::RGB(0, 0, 0),
            snake: Color::RGB(0, 255, 0),
            food: Color::RGB(255, 0, 0),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub grid: Arc<Grid>,   // Grid size
    pub style: Arc<Style>, // Game style
    pub food: Food,        // Food position on the screen
    pub snake: Snake,      // Snake
    pub score: u32,        // Score of the game
}

impl Game {
    pub fn new(grid: Arc<Grid>, style: Arc<Style>) -> Game {
        let snake = Snake::new(grid.clone());
        let food = Food::new(grid.clone());
        Game {
            grid,
            style,
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
#[derive(Debug)]
pub struct Snake {
    pub body: Vec<Block>, // Snake body position on the screen
    direction: Direction, // Direction the snake is moving
    eat: bool,            // If the snake has eaten the food
}
impl Snake {
    fn new(grid: Arc<Grid>) -> Snake {
        Snake {
            body: vec![
                Block(grid.0 / 2, grid.1 / 2),
                Block(grid.0 / 2 + 1, grid.1 / 2),
                Block(grid.0 / 2 + 2, grid.1 / 2),
            ],
            direction: Direction::Left,
            eat: false,
        }
    }

    /// Update the snake position / next tick
    fn update(&mut self, grid: Arc<Grid>) {
        let Block(head_x, head_y) = self.body[0];

        // Create a new head based on the current direction of the snake
        let mut new_head: Block = match self.direction {
            Direction::Up => Block(head_x, head_y - 1),
            Direction::Down => Block(head_x, head_y + 1),
            Direction::Left => Block(head_x - 1, head_y),
            Direction::Right => Block(head_x + 1, head_y),
        };

        // handle the snake going out of bounds
        if new_head.0 >= grid.0 {
            new_head.0 = 0;
        } else if new_head.0 < 0 {
            new_head.0 = grid.0 - 1;
        } else if new_head.1 >= grid.1 {
            new_head.1 = 0;
        } else if new_head.1 < 0 {
            new_head.1 = grid.1 - 1;
        }

        self.body.insert(0, new_head); // Insert new head

        if self.eat == false {
            self.body.pop(); // Remove the last element if the snake has not eaten
        }
        self.eat = false;
    }

    /// Change the direction of the snake
    /// The snake can't go in the opposite direction
    fn cd(&mut self, direction: Direction) {
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

    fn eat(&mut self) {
        self.eat = true;
        println!("Snake: Eating something");
    }
}

#[derive(Debug)]
pub struct Food {
    pub position: (i32, i32), // Food position on the screen
}
impl Food {
    fn new(grid: Arc<Grid>) -> Food {
        // Randomize the position of the food
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..grid.0);
        let y = rng.gen_range(0..grid.1);
        Food { position: (x, y) }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_snake_new() {
        let grid = Arc::new(Grid(80, 60));
        let snake = Snake::new(grid);
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.direction, Direction::Left);
    }

    #[test]
    fn test_snake_update() {
        let grid = Arc::new(Grid(80, 60));
        let mut snake = Snake::new(grid.clone());

        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body[1], Block(41, 30));
        assert_eq!(snake.body[2], Block(42, 30));
        assert_eq!(snake.body.len(), 3);

        snake.update(grid.clone());

        assert_eq!(snake.body[0], Block(39, 30));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body[2], Block(41, 30));
    }
    #[test]
    fn test_snake_cd() {
        let grid = Arc::new(Grid(80, 60));
        let mut snake = Snake::new(grid.clone());

        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body[1], Block(41, 30));
        assert_eq!(snake.body[2], Block(42, 30));
        assert_eq!(snake.body.len(), 3);

        snake.cd(Direction::Up);
        snake.update(grid.clone());

        assert_eq!(snake.body[0], Block(40, 29));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body[2], Block(41, 30));
    }

    #[test]
    fn test_snake_eat() {
        let grid = Arc::new(Grid(80, 60));
        let mut snake = Snake::new(grid.clone());

        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body[1], Block(41, 30));
        assert_eq!(snake.body[2], Block(42, 30));

        snake.eat();
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.body[0], Block(40, 30));
        assert_eq!(snake.body[1], Block(41, 30));
        assert_eq!(snake.body[2], Block(42, 30));

        snake.update(grid.clone());
        assert_eq!(snake.body.len(), 4);
        assert_eq!(snake.body[0], Block(39, 30));
        assert_eq!(snake.body[1], Block(40, 30));
        assert_eq!(snake.body[2], Block(41, 30));
        assert_eq!(snake.body[3], Block(42, 30));
    }

    #[test]
    fn test_snake_mode_edge() {
        let grid = Arc::new(Grid(10, 10));
        let mut snake = Snake::new(grid.clone());

        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.body[0], Block(5, 5));
        assert_eq!(snake.body[1], Block(6, 5));
        assert_eq!(snake.body[2], Block(7, 5));
        snake.update(grid.clone());
        snake.update(grid.clone());
        snake.update(grid.clone());
        snake.update(grid.clone());
        snake.update(grid.clone());
        snake.update(grid.clone());
        assert_eq!(snake.body[0], Block(9, 5));
        assert_eq!(snake.body[1], Block(0, 5));
        assert_eq!(snake.body[2], Block(1, 5));
    }
    #[test]
    fn test_create_food() {
        let grid = Arc::new(Grid(10, 10));
        let food = Food::new(grid.clone());
        assert_eq!(food.position.0 >= 0, true);
        assert_eq!(food.position.0 < grid.0, true);
        assert_eq!(food.position.1 >= 0, true);
        assert_eq!(food.position.1 < grid.1, true);
    }
    #[test]
    fn test_create_food_with_empty_screen() {
        let grid = Arc::new(Grid(1, 1));
        let food = Food::new(grid.clone());
        assert_eq!(food.position.0, 0);
        assert_eq!(food.position.1, 0);
    }

    #[test]
    fn test_calculate_game_level() {
        let grid = Arc::new(Grid(10, 10));
        let style = Arc::new(Style::default());
        let mut game = Game::new(grid, style);
        assert_eq!(game.calculate_level(), 0);
        game.score = 5;
        assert_eq!(game.calculate_level(), 0);
        game.score = 10;
        assert_eq!(game.calculate_level(), 1);
        game.score = 50;
        assert_eq!(game.calculate_level(), 5);
    }
}
