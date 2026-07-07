mod render;
mod snake;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{env::current_exe, fs::File, io::Read};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyEventKind, poll},
    execute,
    terminal::{size as terminal_size, EnterAlternateScreen,enable_raw_mode,LeaveAlternateScreen,disable_raw_mode},
};
use rand::prelude::IndexedRandom;
use render::render_game;
use snake::{Direction, Snake};

#[derive(PartialEq)]
enum State {
    Ongoing,
    Over,
    Paused,
}

pub struct Game {
    snake: Snake,
    food: (u16, u16),
    score: u32,
    state: State,
    highscore: u32,
    tick: Duration,
}

impl Game {
    pub fn init() -> Game {
        let snk = snake::Snake::new();
        Game {
            food: spawn_food(&snk),
            snake: snk,
            score: 0_u32,
            state: State::Over,
            highscore: get_current_high_score(),
            tick: Duration::from_millis(140),
        }
    }

    pub fn update(&mut self, ev: &KeyEvent, last_dir: Direction) -> bool {
        if ev.kind == KeyEventKind::Press {
            if self.state == State::Ongoing {
                match ev.code {
                    KeyCode::Up => {
                        if last_dir.opposite() != Direction::Up {
                            self.snake.direction = Direction::Up;
                        }
                    }
                    KeyCode::Char('w') => {
                        if last_dir.opposite() != Direction::Up {
                            self.snake.direction = Direction::Up;
                        }
                    }

                    KeyCode::Down => {
                        if last_dir.opposite() != Direction::Down {
                            self.snake.direction = Direction::Down;
                        }
                    }
                    KeyCode::Char('s') => {
                        if last_dir.opposite() != Direction::Down {
                            self.snake.direction = Direction::Down;
                        }
                    }

                    KeyCode::Left => {
                        if last_dir.opposite() != Direction::Left {
                            self.snake.direction = Direction::Left;
                        }
                    }
                    KeyCode::Char('a') => {
                        if last_dir.opposite() != Direction::Left {
                            self.snake.direction = Direction::Left;
                        }
                    }

                    KeyCode::Right => {
                        if last_dir.opposite() != Direction::Right {
                            self.snake.direction = Direction::Right;
                        }
                    }
                    KeyCode::Char('d') => {
                        if last_dir.opposite() != Direction::Right {
                            self.snake.direction = Direction::Right;
                        }
                    }
                    KeyCode::Char('q') => {
                        self.snake = Snake::new();
                        self.state = State::Over;
                        self.tick = Duration::from_millis(140);
                        save_high_score(&self.highscore);
                    }
                    KeyCode::Char(' ') => {
                        self.state = State::Paused;
                    }
                    _ => {}
                }
            } else if self.state == State::Paused {
                match ev.code {
                    KeyCode::Char(' ') => {
                        self.state = State::Ongoing;
                    }
                    KeyCode::Char('q') => {
                        self.snake = Snake::new();
                        self.state = State::Over;
                        self.tick = Duration::from_millis(140);
                        save_high_score(&self.highscore);
                    }
                    _ => {}
                }
            } else if self.state == State::Over {
                match ev.code {
                    KeyCode::Char(' ') => {
                        self.state = State::Ongoing;
                        self.score = 0;
                    }
                    KeyCode::Char('q') => {
                        save_high_score(&self.highscore);
                        return false;
                    }
                    _ => {}
                }
            }
        }

        true
    }

    pub fn tick(&mut self) {
        if self.state != State::Ongoing {
            return;
        }

        let (c, r) = {
            let back = self.snake.body.back().unwrap();
            (*back).clone()
        };
        let (cols, rows) = terminal_size().unwrap();

        let tail = *self.snake.body.front().unwrap();
        match self.snake.direction {
            Direction::Down => {
                let next_pos = (c, r + 1);
                if r + 1 > rows || (self.snake.hashed_body.contains(&next_pos) && next_pos != tail)
                {
                    self.snake = Snake::new();
                    self.state = State::Over;
                } else {
                    self.snake.hashed_body.insert(next_pos);
                    self.snake.body.push_back(next_pos);
                }
            }

            Direction::Up => {
                if r <= 0 {
                    self.snake = Snake::new();
                    self.state = State::Over;
                } else {
                    let next_pos = (c, r - 1);
                    if self.snake.hashed_body.contains(&next_pos) && next_pos != tail {
                        self.snake = Snake::new();
                        self.state = State::Over;
                    } else {
                        self.snake.hashed_body.insert(next_pos);
                        self.snake.body.push_back(next_pos);
                    }
                }
            }

            Direction::Left => {
                if c <= 0 {
                    self.snake = Snake::new();
                    self.state = State::Over;
                } else {
                    let next_pos = (c - 1, r);
                    if self.snake.hashed_body.contains(&next_pos) && next_pos != tail {
                        self.snake = Snake::new();
                        self.state = State::Over;
                    } else {
                        self.snake.hashed_body.insert(next_pos);
                        self.snake.body.push_back(next_pos);
                    }
                }
            }

            Direction::Right => {
                let next_pos = (c + 1, r);
                if c + 1 > cols || (self.snake.hashed_body.contains(&next_pos) && next_pos != tail)
                {
                    self.snake = Snake::new();
                    self.state = State::Over;
                } else {
                    self.snake.hashed_body.insert(next_pos);
                    self.snake.body.push_back(next_pos);
                }
            }
        }

        if self.state == State::Ongoing {
            if self.food.0 > cols || self.food.1 > rows {
                self.food = spawn_food(&self.snake);
            }
            if self.food == *self.snake.body.back().unwrap() {
                self.food = spawn_food(&self.snake);
                self.score += 1;
                if self.score > 1 && self.tick > Duration::from_millis(50) && self.score % 2 == 0 {
                    self.tick -= Duration::from_millis(1);
                }
                if self.highscore < self.score {
                    self.highscore = self.score;
                }
            } else {
                let tail = self.snake.body.pop_front().unwrap();
                self.snake.hashed_body.remove(&tail);
            }
        }
    }

    pub fn start(&mut self) {
        enable_raw_mode().unwrap();
        execute!(std::io::stdout(), cursor::Hide,EnterAlternateScreen).unwrap();
        render_game(self, terminal_size().unwrap());

        let mut last_tick = Instant::now();
        let mut last_tick_direction = self.snake.direction;

        loop {
            let timeout = self.tick.saturating_sub(last_tick.elapsed());
            if poll(timeout).unwrap() {
                let event = crossterm::event::read().unwrap();
                if let Event::Key(key_event) = event {
                    let running = self.update(&key_event, last_tick_direction);
                    if !running {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= self.tick {
                self.tick();
                render_game(self, terminal_size().unwrap());
                last_tick = Instant::now();
                last_tick_direction = self.snake.direction;
            }
        }
        disable_raw_mode().unwrap();
        
        execute!(std::io::stdout(), cursor::Show,LeaveAlternateScreen).unwrap();
    }
}

pub fn spawn_food(snake: &Snake) -> (u16, u16) {
    let mut available_positions: Vec<(u16, u16)> = Vec::new();
    let term_size = terminal_size().unwrap();

    for c in 1..term_size.0 {
        for r in 1..term_size.1 {
            let is_on_snake = snake.hashed_body.contains(&(c, r));

            if !is_on_snake {
                available_positions.push((c, r));
            }
        }
    }

    let mut rng = rand::rng();

    match available_positions.choose(&mut rng) {
        Some(&pos) => pos,
        None => (0, 0),
    }
}

fn save_high_score(score:&u32){
    if &get_current_high_score() >= score {
        return;
    }
    let exe = current_exe().unwrap();
    let exe_path = exe.parent().expect("Can not find exe path");

    let open = File::open(exe_path.join("high.score"));
    if let Ok(mut f) = open {
        f.write(score.to_string().as_bytes()).unwrap();
    } else {
        let create = File::create(exe_path.join("high.score"));
        match create {
            Ok(mut f) => {
                f.write(score.to_string().as_bytes()).unwrap();
            }
            Err(e) => panic!("{e}"),
        }
    }
}

fn get_current_high_score() -> u32{
    let exe = current_exe().unwrap();
    let exe_path = exe.parent().expect("Can not find exe path");
    
    let open = File::open(exe_path.join("high.score"));
    let mut buf = String::new();
    if let Ok(mut f) = open {
        f.read_to_string(&mut buf).unwrap();
        if buf.is_empty(){
            return 0;
        }else {
            return buf.trim().parse::<u32>().unwrap();
        }
    }
    0
}
