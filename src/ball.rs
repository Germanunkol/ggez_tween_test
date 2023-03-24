use ggez::glam::*;
use ggez::*;
use ggez::graphics::{self, DrawParam, Canvas, Mesh};

pub struct Ball {
    radius: f32,
    pos: Vec2,
    vel: Vec2,
}

impl Ball {

    pub fn new() -> Self {

        let radius = 25.0;

        Ball {
            radius: radius,
            //pos: Vec2::ZERO,
            pos: Vec2::new( 200.0, 300.0 ),
            //vel: Vec2::new( 685.0, 390.0 ),
            vel: Vec2::new( 50.0, 100.0 ),
        }
    }

    pub fn update( &mut self, dt: f32, screen_w: f32, screen_h: f32 ) {
        self.pos += self.vel * dt;
        if self.pos.x + self.radius > screen_w {
            let overshoot = self.pos.x + self.radius - screen_w;
            self.pos.x = self.pos.x - overshoot;
            self.vel.x = -self.vel.x;
        }
        if self.pos.y + self.radius > screen_h {
            let overshoot = self.pos.y + self.radius - screen_h;
            self.pos.y = self.pos.y - overshoot;
            self.vel.y = -self.vel.y;
        }
        if self.pos.x - self.radius < 0.0 {
            let overshoot = self.pos.x - self.radius;
            self.pos.x = self.pos.x - overshoot;
            self.vel.x = -self.vel.x;
        }
        if self.pos.y - self.radius < 0.0 {
            let overshoot = self.pos.y - self.radius;
            self.pos.y = self.pos.y - overshoot;
            self.vel.y = -self.vel.y;
        }
        //println!( "Pos {}, {}", self.pos.x, self.pos.y );
        //println!( "Vel {}, {}", self.vel.x, self.vel.y );
    }

    pub fn draw( &self, ctx: &mut Context, canvas: &mut graphics::Canvas ) {

        let c = graphics::Mesh::new_circle(
            ctx, graphics::DrawMode::fill(),
            //Vec2::ZERO,
            self.pos,
            self.radius, 0.1, graphics::Color::BLUE
            ).unwrap();

        let drawparams = graphics::DrawParam::default();
            //.color( graphics::Color::BLUE );
        println!("pos {} {}", self.pos.x, self.pos.y); 
        canvas.draw( &c, drawparams );
    }
}
