use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, Drawable, DrawParam, Mesh, Rect, Text};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyCode;
use ggez::mint::Point2;

const MIDDLE_LINE: f32 = 2.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 400.0;

const PADDING: f32 = 40.0;

pub struct PongGame {
    player_1_score: u16,
    player_2_score: u16,

    player_1_pos: Point2<f32>,
    player_2_pos: Point2<f32>,

    ball_pos: Point2<f32>,
    ball_vel: Vec2,
}

impl PongGame {
    pub fn new(_ctx: &mut Context) -> PongGame {
        let (screen_width, screen_height) = _ctx.gfx.drawable_size();
        let (screen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);
        // Load/create resources such as images here.
        PongGame {
            player_1_score: 0,
            player_2_score: 0,
            player_1_pos: Point2::from_slice(&[PADDLE_WIDTH_HALF + PADDING, screen_height_half]),
            player_2_pos: Point2::from_slice(&[screen_width - PADDLE_WIDTH_HALF - PADDING, screen_height_half]),
            ball_pos: Point2::from_slice(&[screen_width_half, screen_height_half]),
            ball_vel: Vec2::new(-BALL_SPEED, BALL_SPEED),
        }
    }
}


impl EventHandler for PongGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let dt = _ctx.time.delta().as_secs_f32();
        let (screen_width, screen_height) = _ctx.gfx.drawable_size();
        let (screen_width_half, screen_height_half) = (screen_width * 0.5, screen_height * 0.5);
        let ball_starting_pos = Point2::from_slice(&[screen_width_half, screen_height_half]);
        fn updated_pos_in_screen(_ctx: &Context, pos: f32, pos_delta: f32, screen_height: f32) -> f32 {
            let updated_pos = pos + pos_delta;
            if updated_pos < PADDLE_HEIGHT_HALF { PADDLE_HEIGHT_HALF } else if updated_pos > (screen_height - PADDLE_HEIGHT_HALF) { screen_height - PADDLE_HEIGHT_HALF } else { updated_pos }
        }

        if _ctx.keyboard.is_key_pressed(KeyCode::W) {
            self.player_1_pos.y = updated_pos_in_screen(_ctx, self.player_1_pos.y, -PLAYER_SPEED * dt, screen_height);
        }

        if _ctx.keyboard.is_key_pressed(KeyCode::S) {
            self.player_1_pos.y = updated_pos_in_screen(_ctx, self.player_1_pos.y, PLAYER_SPEED * dt, screen_height);
        }

        if _ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.player_2_pos.y = updated_pos_in_screen(_ctx, self.player_2_pos.y, -PLAYER_SPEED * dt, screen_height);
        }

        if _ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.player_2_pos.y = updated_pos_in_screen(_ctx, self.player_2_pos.y, PLAYER_SPEED * dt, screen_height);
        }

        self.ball_pos.x += self.ball_vel.x * dt;
        self.ball_pos.y += self.ball_vel.y * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos = ball_starting_pos;
            self.player_2_score += 1;
        }

        if self.ball_pos.x > screen_width {
            self.ball_pos = ball_starting_pos;
            self.player_1_score += 1;
        }

        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        }

        if self.ball_pos.y > screen_height - BALL_SIZE_HALF {
            self.ball_pos.y = screen_height - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let collides_with_player_1 = self.ball_pos.x - BALL_SIZE_HALF < self.player_1_pos.x + PADDLE_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - PADDLE_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + PADDLE_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - PADDLE_HEIGHT_HALF;

        if collides_with_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        }

        let collides_with_player_2 = self.ball_pos.x - BALL_SIZE_HALF < self.player_2_pos.x + PADDLE_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - PADDLE_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + PADDLE_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - PADDLE_HEIGHT_HALF;

        if collides_with_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let screen_width_half = screen_width * 0.5;

        let middle_line_rect = Rect::new(screen_width_half - (MIDDLE_LINE * 0.5), 0.0, MIDDLE_LINE, screen_height);
        let middle_line_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), middle_line_rect, Color::WHITE)?;

        let user_paddle_rect = Rect::new(-PADDLE_WIDTH_HALF, -PADDLE_HEIGHT_HALF, PADDLE_WIDTH, PADDLE_HEIGHT);
        let user_paddle_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), user_paddle_rect, Color::WHITE)?;

        let ball_rect = Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, Color::WHITE)?;

        canvas.draw(&middle_line_mesh, DrawParam::default());
        canvas.draw(&user_paddle_mesh, DrawParam::default().dest(self.player_1_pos));
        canvas.draw(&user_paddle_mesh, DrawParam::default().dest(self.player_2_pos));
        canvas.draw(&ball_mesh, DrawParam::default().dest(self.ball_pos));

        let score_text = Text::new(format!("{}          {}", self.player_1_score, self.player_2_score));
        let score_text_width = score_text.dimensions(ctx).unwrap().w;
        canvas.draw(&score_text, DrawParam::default().dest(Point2::from_slice(&[screen_width_half - (score_text_width * 0.5), 40.0])));

        canvas.finish(ctx)
    }
}