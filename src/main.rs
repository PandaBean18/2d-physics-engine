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
    radius: f64,
    gl: GlGraphics
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
        radius: 100.0, 
        gl: GlGraphics::new(opengl)
    };

    let mut cursor_pos: [f64; 2] = [0.0, 0.0];
    let mut prev_cursor_pos: [f64; 2] = [0.0, 0.0];
    let mut button_state: ButtonState = ButtonState::Release;
    let mut prev_mouse_button: Option<MouseButton> = None;
    let mut dx: f64 = 0.0; 
    let mut dy: f64 = 0.0; 
    let mut dt: f64 = 0.0;
    let mut ball_moving: bool = false; 

    while let Some(e) = events.next(&mut window) {
        touch_visualizer.event(window.size(), &e);

        //dy = 0.0; 
        //dx = 0.0;

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
                ball_moving = false;
            }

            _ => {}
        }
        
        if let Some(args) = e.render_args() {
            ball.render(&args);
        }

        if let Some(args) = e.update_args() { 
            println!("speed x: {}", (dx / 3779.5275590551) / args.dt);
            println!("speed y: {}", (dy / 3779.5275590551) / args.dt);
        }

        e.mouse_cursor(|pos| {
            prev_cursor_pos = pos;
        });
    }
}
