use std::rc::Rc;

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

#[derive(Debug)]
pub struct Game {
    pub grid: Rc<Grid>, // Grid size
    pub food: Food,     // Food position on the screen
    pub snake: Snake,   // Snake
    pub speed: u32,     // Speed of the game
}

impl Game {
    pub fn new(grid: Rc<Grid>) -> Game {
        let snake = Snake::new(grid.clone());
        let food = Food::new(grid.clone());
        Game {
            grid,
            food,
            snake,
            speed: 1000,
        }
    }
    pub fn tick(&mut self) {
        self.snake.update(self.grid.clone());
        if self.snake.body[0] == Block(self.food.position.0, self.food.position.1) {
            self.snake.eat();
            self.food = Food::new(self.grid.clone());
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
    fn new(grid: Rc<Grid>) -> Snake {
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
    fn update(&mut self, grid: Rc<Grid>) {
        let Block(head_x, head_y) = self.body[0];

        // Create a new head based on the current direction of the snake
        let mut new_head: Block = match self.direction {
            Direction::Up => Block(head_x, head_y - 1),
            Direction::Down => Block(head_x, head_y + 1),
            Direction::Left => Block(head_x - 1, head_y),
            Direction::Right => Block(head_x + 1, head_y),
        };

        // handle the snake going out of bounds
        if new_head.0 > grid.0 {
            new_head.0 = 0;
        } else if new_head.0 < 0 {
            new_head.0 = grid.0;
        } else if new_head.1 > grid.1 {
            new_head.1 = 0;
        } else if new_head.1 < 0 {
            new_head.1 = grid.1;
        }

        self.body.insert(0, new_head); // Insert new head

        if self.eat == false {
            self.body.pop(); // Remove the last element if the snake has not eaten
        }
        self.eat = false;
    }

    /// Change the direction of the snake
    fn cd(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn eat(&mut self) {
        self.eat = true;
    }

    fn get_body(&self) -> &Vec<Block> {
        &self.body
    }
}

#[derive(Debug)]
pub struct Food {
    pub position: (i32, i32), // Food position on the screen
}
impl Food {
    fn new(grid: Rc<Grid>) -> Food {
        // Todo randomize the position of the food
        Food { position: (5, 5) }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_snake_new() {
        let grid = Rc::new(Grid(80, 60));
        let snake = Snake::new(grid);
        assert_eq!(snake.body.len(), 3);
        assert_eq!(snake.direction, Direction::Left);
    }

    #[test]
    fn test_snake_update() {
        let grid = Rc::new(Grid(80, 60));
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
        let grid = Rc::new(Grid(80, 60));
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
        let grid = Rc::new(Grid(80, 60));
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
}
