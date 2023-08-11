use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};

const PADDLE_WIDTH: i32 = 12;
const PADDLE_HEIGHT: i32 = 2;

const RECTANGLE_WIDTH: i32 = 15;
const RECTANGLE_HEIGHT: i32 = 2;

struct Brick {
    pub position: Vec2, // top left corner
    pub size: Vec2,
    pub broken: bool,
}

struct GameState {
    pub dimensions: Vec2,
    pub paddle_position: Vec2,
    pub paddle_size: Vec2,
    pub ball_position: Vec2,
    pub ball_speed: Vec2,
    pub bricks: Vec<Brick>,
}

impl GameState {
    fn new(win_size: Vec2) -> Self {
        let dimensions = Vec2::xy(5 + (RECTANGLE_WIDTH + 5) * 5, win_size.y);

        let paddle_position = Vec2::xy(5, win_size.y - 5);
        let paddle_size = Vec2::xy(PADDLE_WIDTH, PADDLE_HEIGHT);

        let ball_position = paddle_position - Vec2::xy(0, 5);
        let ball_speed = Vec2::xy(1, -1);

        let mut bricks = Vec::new();
        for i in 0..4 {
            for j in 0..5 {
                bricks.push(Brick {
                    position: Vec2::xy(5 + (RECTANGLE_WIDTH + 5) * j, 5 + i * RECTANGLE_HEIGHT),
                    size: Vec2::xy(RECTANGLE_WIDTH, RECTANGLE_HEIGHT),
                    broken: false,
                })
            }
        }

        Self {
            dimensions,
            paddle_position,
            paddle_size,
            ball_position,
            ball_speed,
            bricks,
        }
    }

    fn update(&mut self) {
        self.ball_position += self.ball_speed;

        // Keep paddle in bounds
        if self.paddle_position.x + self.paddle_size.x >= self.dimensions.x - 1 {
            self.paddle_position.x = self.dimensions.x - self.paddle_size.x - 1;
        }
        if self.paddle_position.x <= 1 {
            self.paddle_position.x = 1;
        }

        // Keep ball in bounds
        if self.ball_position.x >= self.dimensions.x {
            self.ball_speed *= Vec2::xy(-1, 1);
        }
        if self.ball_position.x <= 1 {
            self.ball_speed *= Vec2::xy(-1, 1);
        }
        if self.ball_position.y <= 1 {
            self.ball_speed *= Vec2::xy(1, -1);
        }

        // Ball and brick collision
        for brick in &mut self.bricks {
            if !brick.broken
                && self.ball_position.x >= brick.position.x
                && self.ball_position.x <= brick.position.x + brick.size.x
                && self.ball_position.y >= brick.position.y
                && self.ball_position.y <= brick.position.y + brick.size.y
            {
                brick.broken = true;
                self.ball_speed *= Vec2::xy(1, -1);
            }
        }

        // Ball and paddle collision
        if self.ball_position.x >= self.paddle_position.x
            && self.ball_position.x <= self.paddle_position.x + self.paddle_size.x
            && self.ball_position.y >= self.paddle_position.y
            && self.ball_position.y <= self.paddle_position.y + self.paddle_size.y
        {
            self.ball_speed *= Vec2::xy(1, -1);
        }
    }
}

fn main() {
    let mut app = App::default();

    let win_size = app.window().size();
    let mut state = GameState::new(win_size);

    let mut key_events = Vec::new();
    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            key_events.push(*key_event);
            if let KeyEvent::Pressed(Key::Q) = key_event {
                app_state.stop();
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::H => state.paddle_position += Vec2::xy(-1, 0),
                Key::L => state.paddle_position += Vec2::xy(1, 0),
                _ => (),
            }
        }

        if app_state.step() % 2 == 0 {
            state.update();
        }

        if state.ball_position.y >= state.dimensions.y {
            app_state.stop();
        }

        let mut pencil = Pencil::new(window.canvas_mut());

        // Draw borders
        pencil.set_origin(Vec2::zero()).draw_rect(
            &RectCharset::double_lines(),
            Vec2::zero(),
            state.dimensions,
        );

        // Draw paddle
        pencil.set_foreground(Color::Xterm(3)).draw_rect(
            &RectCharset::simple_round_lines(),
            state.paddle_position,
            state.paddle_size,
        );

        // Draw bricks
        for brick in &state.bricks {
            if !brick.broken {
                pencil
                    .set_origin(Vec2::zero())
                    .set_foreground(Color::Xterm(1))
                    .draw_rect(&RectCharset::simple_lines(), brick.position, brick.size);
            }
        }

        // Draw ball
        pencil
            .set_foreground(Color::Xterm(2))
            .set_style(Style::Bold)
            .draw_char('o', state.ball_position);
    });
}
