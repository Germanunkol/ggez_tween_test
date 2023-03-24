use ggez::{Context, ContextBuilder, GameResult};
use ggez::*;
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

mod ball;

struct State {
    main_ball : ball::Ball,
}

fn main() {
    let state = State { main_ball: ball::Ball::new() };
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("generative_art", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}

/*struct MyGame {
    // Your state here...
    main_ball: ball::Ball,
    screen: graphics::ScreenImage,
}

impl MyGame {
    pub fn new( ctx: &mut Context) -> MyGame {

        let screen =
            graphics::ScreenImage::new(ctx, graphics::ImageFormat::Rgba8UnormSrgb, 1., 1., 1);

        // Load/create resources such as images here.
        MyGame {
            // ...
            main_ball: ball::Ball::new( ctx ),
            screen,
        }
    }
}*/


impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
    
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        const DESIRED_FPS: u32 = 30;

        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            
            //let smooth_dt = ctx.time.delta().as_secs_f32();
            self.main_ball.update( seconds, screen_w as f32, screen_h as f32 );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.main_ball.draw( ctx, &mut canvas );
        canvas.finish(ctx)?;
        Ok(())
    }
}
