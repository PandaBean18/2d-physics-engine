extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate touch_visualizer;

#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;
#[cfg(feature = "include_glfw")]
extern crate glfw_window;

use touch_visualizer::TouchVisualizer;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;
use crate::piston::Window as OtherWindow;
use piston::input::*;
use piston::Size;

pub struct Ball {
    x: f64, 
    y: f64,
    initial_velocity_x: f64, 
    final_velocity_x: f64,
    initial_velocity_y: f64, 
    final_velocity_y: f64,
    initial_pos_x: f64, // to calc velocity
    initial_pos_y: f64,
    time: f64, 
    accn_x: f64,
    accn_y: f64,
    dir_x: i8, 
    dir_y: i8,
    radius: f64,
    gl: GlGraphics
}

pub struct Wall {
    points: Vec<[f64; 2]>
}

impl Ball {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);

            let ellipse = Rectangle::new_round(color::BLACK, self.radius);
            let dims = rectangle::square(0.0, 0.0, self.radius * 2.0); 
            ellipse.draw(dims, &draw_state::DrawState::default(), c.transform.trans(self.x, self.y), gl);
        });
            
    }

    fn cursor_on_ball(&self, pos: &[f64; 2]) -> bool {
        // principle of working 
        // equation of a circle on xy plane is given as: (x-h)^2 + (y-k)^2 = a^2
        // where (h, k) is the centre of the circle and a is the radius 
        // by replacing x and y with given coords, if we get a value less than (a^2), we can
        // say that the  point is inside the circle.

        // finding coords of the centre as the circle is not actually a circle but a square with rounded corners.
        let centre: [f64; 2] = [self.x + self.radius, self.y + self.radius]; 
        let val_x = (pos[0] - centre[0]) * (pos[0] - centre[0]); 
        let val_y = (pos[1] - centre[1]) * (pos[1] - centre[1]); 
        
        if (val_x + val_y) <= self.radius * self.radius {
            return true;
        } else { 
            return false;
        }
    }

    fn update_pos_x(&mut self, dt: f64) {
        // v = u + a*t
        // s = u*t + 0.5*a*t^2
        let u = self.final_velocity_x; 
        let a = self.accn_x; 
        let time = dt;
        let v = u + (a * time); 
        self.final_velocity_x = v; 
        let dist_m = (u * time) + (0.5 * a * time * time); 
        let dist_pix = dist_m * 37.795275590551; 
        self.x += dist_pix;
        if self.final_velocity_x > 0.0 {
            self.dir_x = 1;
        } else {
            self.dir_x = -1;
        }
    }

    fn update_pos_y(&mut self, dt: f64) {
        let u = self.final_velocity_y; 
        let a = self.accn_y; 
        let time = dt;
        let v = u + (a * time); 
        self.final_velocity_y = v; 
        let dist_m = (u * time) + (0.5 * a * time * time); 
        let dist_pix = dist_m * 37.795275590551; 
        self.y += dist_pix;
        if self.final_velocity_y > 0.0 {
            self.dir_y = 1;
        } else {
            self.dir_y = -1;
        }
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("test", [800, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    let mut touch_visualizer = TouchVisualizer::new();
    
    let mut ball = Ball {
        x: 0.0, 
        y: 0.0,
        radius: 25.0, 
        initial_velocity_x: 0.0, 
        final_velocity_x: 0.0,
        initial_velocity_y: 0.0, 
        final_velocity_y: 0.0,
        initial_pos_x: 0.0, 
        initial_pos_y: 0.0,
        time: 0.0,
        accn_x: 0.0,
        accn_y: 9.8,
        dir_x: 1, 
        dir_y: 1,
        gl: GlGraphics::new(opengl)
    };

    let mut cursor_pos: [f64; 2] = [0.0, 0.0];
    let mut prev_cursor_pos: [f64; 2] = [0.0, 0.0];
    let mut button_state: ButtonState = ButtonState::Release;
    let mut prev_mouse_button: Option<MouseButton> = None;
    let mut dx: f64 = 0.0; 
    let mut dy: f64 = 0.0; 
    let mut dt: f64 = 0.0;
    // stroes if ball is moving due to user input
    let mut ball_moving: bool = false; 

    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);

        e.mouse_cursor(|pos| {
            cursor_pos = pos;
        });

        if let Some(Button::Mouse(button)) = e.press_args() {
            prev_mouse_button = Some(button);
        }

        e.button(|args| {
            button_state = args.state;
        });

        match button_state {
            ButtonState::Press => {
                match prev_mouse_button {
                    Some(MouseButton::Left) => {
                        if ball_moving || ball.cursor_on_ball(&cursor_pos) {
                            if !ball_moving {
                                ball_moving = true; 
                                ball.time = 0.0;
                                ball.initial_pos_x = ball.x; 
                                ball.initial_pos_y = ball.y;
                                ball.accn_x = 0.0; 
                                ball.accn_y = 9.8;
                            }
                            e.mouse_relative(|d| {
                                dx = d[0]; 
                                dy = d[1];
                                ball.x += d[0]; 
                                ball.y += d[1];
                            });
                        }
                    }

                    _ => {}
                }
            }

            ButtonState::Release => {
                if ball_moving {
                    ball_moving = false; 
                    ball.final_velocity_x = (ball.x - ball.initial_pos_x) / (ball.time * 37.795275590551); 
                    ball.final_velocity_y = (ball.y - ball.initial_pos_y) / (ball.time * 37.795275590551); 
                    ball.initial_pos_x = 0.0; 
                    ball.initial_pos_y = 0.0;
                    println!("x: {}", ball.x); 
                    println!("y: {}", ball.y); 
                    println!("velocity x: {}", ball.final_velocity_x);
                    println!("velocity y: {}", ball.final_velocity_y); 
                    println!("acceleration x: {}", ball.accn_x); 
                    println!("acceleration y: {}", ball.accn_y);
                    println!("time: {}", ball.time);
                }
            }
        }

        if let Some(args) = e.update_args() { 
            ball.time += args.dt;
            if !ball_moving {
                if ball.x <= 0.0 && ball.dir_x == -1 {
                    ball.dir_x = 1;
                    ball.final_velocity_x = -1.0 * (0.6 * ball.final_velocity_x);
                } else if ball.x >= window.size().width - 50.0 && ball.dir_x == 1 {
                    ball.dir_x = -1; 
                    ball.final_velocity_x = -1.0 * (0.6 * ball.final_velocity_x);
                }

                if ball.y <= 0.0 && ball.dir_y == -1 {
                    ball.dir_y= 1;
                    ball.final_velocity_y = -1.0 * (0.6 * ball.final_velocity_y);
                } else if ball.y >= window.size().height - 50.0 && ball.dir_y == 1 {
                    ball.dir_y = -1; 
                    ball.final_velocity_y = -1.0 * (0.6 * ball.final_velocity_y);
                }

                ball.update_pos_x(args.dt);
                ball.update_pos_y(args.dt);
            }
        }

        if let Some(args) = e.render_args() {
            ball.render(&args);
        }

        e.mouse_cursor(|pos| {
            prev_cursor_pos = pos;
        });
    }
}
