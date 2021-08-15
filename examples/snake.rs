use graphics::grfx::canvas::Canvas;
use graphics::grfx::color::Color;
use graphics::grfx::render::*;
use graphics::math::FVec2D;
use graphics::math::Point2D;
use rand::Rng;
use std::collections::VecDeque;
const GRID_SCALE: f32 = 20.0;

fn main() {
    let game_canvas = SnakeGame::new(400, 400, "Snake Game".into());
    game_canvas.render();
}

pub struct SnakeGame {
    width: u32,
    height: u32,
    title: String,
    snake: Snake,
    food: Food,
}

impl SnakeGame {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
            food: Food::new((width / 2) as i32, (height / 2) as i32),
            snake: Snake::new(),
        }
    }
}

impl Render2D for SnakeGame {
    ///  Get window properties height
    fn height(&mut self) -> u32 {
        self.height
    }
    /// Get wWindow properties width
    fn width(&mut self) -> u32 {
        self.width
    }
    /// Get wWindow properties title
    fn title(&mut self) -> String {
        self.title.clone()
    }
    ///
    /// Setup method called when the world is first created
    /// Must be overriden.
    ///
    fn setup(&mut self, _canvas: &mut Canvas) -> bool {
        true
    }

    /// Update method called when the canvas is to be updated
    /// This is called periodically per frame and each frame is drawn individually
    /// Must be overriden/implmented
    fn update(&mut self, canvas: &mut Canvas, input: &InputHelper, delta_t: f32) -> bool {
        if self.snake.state == SnakeState::Alive {
            canvas.fill(Color::BLUE);
            self.snake.show(canvas);
            self.snake.update(canvas, delta_t);
            self.food.show(canvas);
            if input.key_pressed(VirtualKeyCode::W) {
                self.snake.dir = FVec2D::new(0.0, -1.0);
            }
            if input.key_pressed(VirtualKeyCode::A) {
                self.snake.dir = FVec2D::new(-1.0, 0.0);
            }
            if input.key_pressed(VirtualKeyCode::S) {
                self.snake.dir = FVec2D::new(0.0, 1.0);
            }
            if input.key_pressed(VirtualKeyCode::D) {
                self.snake.dir = FVec2D::new(1.0, 0.0);
            }
            if self.snake.consume(&mut self.food) {
                let mut random = rand::thread_rng();
                let column = canvas.width() as i32 / GRID_SCALE as i32;
                let row = canvas.height() as i32 / GRID_SCALE as i32;
                // move food to random location
                let x = (random.gen::<u32>()) % column as u32;
                let y = (random.gen::<u32>()) % row as u32;
                self.food.position = Point2D::new(x as i32, y as i32) * GRID_SCALE as i32;
                // don't draw outside of bounds
                self.food.position.clamp(
                    canvas.width() as i32 - GRID_SCALE as i32,
                    canvas.height() as i32 - GRID_SCALE as i32,
                );
            }
        } else {
            canvas.fill(Color::rgb(219, 196, 193));
            self.snake.show(canvas);
            self.food.show(canvas);
            canvas.draw_string(
                Point2D::new(100, 200),
                "Game Over".into(),
                0.3,
                Color::WHITE,
            );
            let score = format!("Final Score: {}", self.snake.trail.len() - 1);
            canvas.draw_string(Point2D::new(50, 250), score, 0.3, Color::WHITE);
        }
        true
    }
}
#[derive(PartialEq)]
enum SnakeState {
    Alive,
    Dead,
}
struct Snake {
    position: FVec2D,
    dir: FVec2D,
    ticks: f32,
    frame_time: f32,
    trail: VecDeque<Point2D>,
    state: SnakeState,
}

impl Snake {
    fn new() -> Self {
        let mut trail = VecDeque::new();
        let position = FVec2D::default();
        trail.push_front(position.to_i32());
        Self {
            position,
            dir: FVec2D::new(1.0, 0.0),
            frame_time: 0.20,
            ticks: 0.0,
            trail,
            state: SnakeState::Alive,
        }
    }

    fn update(&mut self, canvas: &Canvas, timing: f32) {
        self.ticks += timing;

        if self.ticks >= self.frame_time {
            // rotate the snake body, the last position moves to the one before and so forth
            // the head is always the current position (last)
            let head = self.trail.len() - 1;
            for i in 0..self.trail.len() - 1 {
                self.trail[i] = self.trail[i + 1];
            }
            self.position += self.dir * GRID_SCALE;
            self.trail[head] = self.position.to_i32();
            self.ticks = 0.0;

            // check whether snake hit itself
            for i in 0..self.trail.len() - 1 {
                if self.trail[i] == self.trail[head] {
                    self.state = SnakeState::Dead;
                }
            }

            // Snake is dead if head is out of bounds
            if self.trail[head].x < 0
                || self.trail[head].x > canvas.width() as i32
                || self.trail[head].y < 0
                || self.trail[head].y > canvas.height() as i32
            {
                self.state = SnakeState::Dead;
            }
        }
    }

    fn show(&self, canvas: &mut Canvas) {
        for position in &self.trail {
            canvas.fill_rectangle(position, 20, 20, Color::WHITE);
        }
    }

    // determine if food was eaten, if add this food to the snake body
    fn consume(&mut self, food: &mut Food) -> bool {
        let distance = food.position - self.position.to_i32();
        if distance.length() < 5 {
            self.trail.push_back(self.position.to_i32());
            return true;
        }
        return false;
    }
}

struct Food {
    position: Point2D,
}

impl Food {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point2D::new(x, y),
        }
    }

    fn show(&self, canvas: &mut Canvas) {
        canvas.fill_rectangle(&self.position, 20, 20, Color::rgb(255, 0, 100));
    }
}
