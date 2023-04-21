use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics;
use ggez::{Context, GameResult};

mod ball;

const PHYSICS_SIMULATION_FPS: u32 = 50;

struct GameState {
    main_ball : ball::Ball,
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        while ctx.time.check_update_time(PHYSICS_SIMULATION_FPS) {
            let physics_delta_time = 1.0 / f64::from(PHYSICS_SIMULATION_FPS);
            //self.simulate(physics_delta_time);

            //let smooth_dt = ctx.time.delta().as_secs_f32();
            self.main_ball.update( physics_delta_time as f32, screen_w as f32, screen_h as f32 );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        //graphics::clear(ctx, graphics::WHITE);
        self.draw_fps_counter(ctx, &mut canvas)?;
        self.main_ball.draw( ctx, &mut canvas );
        //graphics::present(ctx)?;
        canvas.finish( ctx )?;
        Ok(())
    }
}

impl GameState {

    pub fn draw_fps_counter(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult<()> {
        let fps = ctx.time.fps();
        let delta = ctx.time.delta();
        let stats_display = graphics::Text::new(format!("FPS: {}, delta: {:?}", fps, delta));
        println!(
            "[draw] ticks: {}\tfps: {}\tdelta: {:?}",
            ctx.time.ticks(),
            fps,
            delta,
        );
        let drawparams = graphics::DrawParam::default();
        canvas.draw(
            &stats_display,
            drawparams
        );

        Ok(())
    }

    pub fn draw_circle(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, x: f32) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(x, 0.0),
            100.0,
            2.0,
            graphics::Color::RED,
        )?;
        let drawparams = graphics::DrawParam::default();
        canvas.draw(&circle, drawparams);

        Ok(())
    }

    /*pub fn simulate(&mut self, time: f64) {
        let distance = self.velocity_x as f64 * time;
        println!("[update] distance {}", distance);
        self.pos_x = self.pos_x % 800.0 + distance as f32;
        println!("x {}", self.pos_x);
    }*/
}

fn main() -> GameResult {
    let c = ggez::conf::Conf::new();
    let (ctx, event_loop) = ggez::ContextBuilder::new("generative_art", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    println!("{:#?}", ctx.fs.read_config());
    let state = GameState { main_ball: ball::Ball::new() };
    event::run(ctx, event_loop, state)
}


