use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use mint::Point2;
use random_number::random;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGTH: f32 = 500.0;
const PLAYER_SIZE: Size = Size { width: 10.0, heigth: 100.0 };
const PLAYER_SPEED: f32 = 0.1;
const BALL_SIZE: f32 = 30.0;
const BALL_SPEED: f32 = 1.0;

struct Point {
    x: f32,
    y: f32,
}

struct Size {
    width: f32,
    heigth: f32,
}

struct CurrentDisplay {
    player1_points: i32,
    player2_points: i32,
}

fn main() {
    let window_conf = ggez::conf::WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGTH,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };

    let window_setup = ggez::conf::WindowSetup {
        title:  "game".to_owned(),
        samples: ggez::conf::NumSamples::One,
        vsync: false,
        icon: "".to_owned(),
        srgb: false,
    };


    let (mut ctx, event_loop) = ContextBuilder::new("game", "he")
        .window_mode(window_conf)
        .window_setup(window_setup)
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    player1_pos: Point,
    player2_pos: Point,
    ball_pos: Point,
    current_points: CurrentDisplay,
    colision_side: String,
}

impl Default for MyGame {
    fn default() -> Self {
        Self { 
            player1_pos: Point { x: 0.0, y: WINDOW_HEIGTH / 2.0 },
            player2_pos: Point { x: WINDOW_WIDTH - PLAYER_SIZE.width, y: WINDOW_HEIGTH / 2.0 },
            ball_pos: Point { x: WINDOW_WIDTH / 2.0, y: WINDOW_HEIGTH / 2.0 },
            current_points: CurrentDisplay { player1_points: 0, player2_points: 0 },
            colision_side: String::from("right"),
        }
    }
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.        

        MyGame::default()
    }
    
    pub fn check_ball(&mut self) {
        if self.ball_pos.x < self.player1_pos.x - PLAYER_SIZE.width {
            self.current_points.player2_points += 1;
            reset_game(self);
        }
        
        if self.ball_pos.x > self.player2_pos.x + PLAYER_SIZE.width {
            self.current_points.player1_points += 1;
            reset_game(self);
        }

        if self.player1_pos.x < self.ball_pos.x && self.ball_pos.x < self.player1_pos.x + PLAYER_SIZE.width && self.player1_pos.y < self.ball_pos.y + PLAYER_SIZE.heigth && self.ball_pos.y < self.player1_pos.y + PLAYER_SIZE.heigth {
            self.colision_side = "right".to_string();
            self.randomizer_y();
        }
        if self.player2_pos.x < self.ball_pos.x && self.ball_pos.x < self.player2_pos.x + PLAYER_SIZE.width && self.player2_pos.y < self.ball_pos.y + PLAYER_SIZE.heigth && self.ball_pos.y < self.player2_pos.y + PLAYER_SIZE.heigth {
            self.colision_side = "left".to_string();
            self.randomizer_y();
        }
    }

    pub fn randomizer_y(&mut self) {
        let rand: i32 = random!(1..=30);

        let val = if rand < 15 { -rand } else { rand } as f32;
        
        if self.ball_pos.y > (WINDOW_HEIGTH - BALL_SIZE) {
            self.ball_pos.y = self.ball_pos.y - rand as f32;
        } else {
            self.ball_pos.y = self.ball_pos.y + val;
        }
    }
}

fn reset_game(state: &mut MyGame) {
    state.player1_pos = Point { x: 0.0, y: WINDOW_HEIGTH / 2.0 };
    state.player2_pos = Point { x: WINDOW_WIDTH - PLAYER_SIZE.width, y: WINDOW_HEIGTH / 2.0 };
    state.ball_pos = Point { x: WINDOW_WIDTH / 2.0, y: WINDOW_HEIGTH / 2.0 };
}

fn check_bounds_y(player: &f32) -> bool {
    const WINDOW_INIT: f32 = 0.0;

    if *player > (WINDOW_HEIGTH - PLAYER_SIZE.heigth) || *player < WINDOW_INIT {
        return true;
    }
    return false;
}


impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::W) && !check_bounds_y(&self.player1_pos.y) {
            if !check_bounds_y(&self.player1_pos.y) {
                self.player1_pos.y -= PLAYER_SPEED;
            } else {
                self.player1_pos.y += PLAYER_SPEED;
            }
        } else if _ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::S) {
            if !check_bounds_y(&self.player1_pos.y) {
                self.player1_pos.y += PLAYER_SPEED;
            } else {
                self.player1_pos.y -= PLAYER_SPEED;
            }
        }

        if !check_bounds_y(&self.player2_pos.y) {
            self.player2_pos.y = self.ball_pos.y;
        }

        MyGame::check_ball(self);
        
        if self.colision_side == "right" {
            self.ball_pos.x += BALL_SPEED;
        }
        if self.colision_side == "left" {
            self.ball_pos.x -= BALL_SPEED;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        graphics::Canvas::set_screen_coordinates(&mut canvas, graphics::Rect { x: 0.0, y: 0.0, w: WINDOW_WIDTH, h: WINDOW_HEIGTH });
        
        graphics::Canvas::draw(&mut canvas, &graphics::Text::new(format!("{} | {}", self.current_points.player1_points, self.current_points.player2_points)), graphics::DrawParam::default().dest(Point2 { x: WINDOW_WIDTH / 2.0, y: 10.0 }));
        graphics::Canvas::draw(&mut canvas, &graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect { x: self.player1_pos.x, y: self.player1_pos.y, w: PLAYER_SIZE.width, h: PLAYER_SIZE.heigth }, graphics::Color::WHITE)?, graphics::DrawParam::new());
        graphics::Canvas::draw(&mut canvas, &graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect { x: self.player2_pos.x, y: self.player2_pos.y, w: PLAYER_SIZE.width, h: PLAYER_SIZE.heigth }, graphics::Color::WHITE)?, graphics::DrawParam::default());
        graphics::Canvas::draw(&mut canvas, &graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect { x: self.ball_pos.x, y: self.ball_pos.y, w: BALL_SIZE, h: BALL_SIZE }, graphics::Color::WHITE)?, graphics::DrawParam::default());

        // Draw code here...
        graphics::Canvas::finish(canvas, ctx)
    }
}