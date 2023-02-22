#![allow(clippy::single_match)]
#![feature(iter_array_chunks)]

mod math;

use std::{time::Instant, error::Error, f64::consts::PI};

use iter_tools::Itertools;
use math::Vec2;
use pixels::{Pixels, SurfaceTexture, PixelsBuilder, wgpu::Color};
use winit::{
    dpi::{PhysicalSize, Position},
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const SCALE: f64 = 64.0; // Defines how many pixels a meter contains

type Pos = Vec2<f64>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let mut input = InputHandler::new();

    let mut ball = Ball::new(1.0);

    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_title("A fantastic window!")
        .with_inner_size(ball.size())
        .with_min_inner_size(ball.size())
        .with_max_inner_size(ball.size())
        .build(&event_loop)
        .unwrap();

    window.set_transparent(true);
    window.set_decorations(false);
    window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(ball.size().width, ball.size().height, surface_texture)
            .texture_format(pixels::wgpu::TextureFormat::Rgba8UnormSrgb)
            .clear_color(Color::TRANSPARENT)
            .build()?
    };

    pixels
        .get_frame_mut()
        .iter_mut()
        .array_chunks::<4>()
        .enumerate()
        .map(|(pos, pixel)| {
            // Calculate coordinates
            (Vec2::<i64>::new(pos as i64 / ball.size().width as i64, pos as i64 % ball.size().height as i64), pixel)
        })
        .for_each(|(pos, pixel)| {
            let [r,g,b,a] = pixel;
            let center = Vec2::<i64>::from(ball.size()) / 2;
            let dist = (pos - center).len();

            *r = 255;
            *g = 255;
            *b = 255;
            if dist < ball.radius as f64 {
                *a = 255;
            } else {
                *a = 0;
            }
        });
    
    ball.set(window.outer_position().unwrap());

    event_loop.run(move |event, _, control_flow| {
        // control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::Moved(new_pos),
                window_id,
            } if window_id == window.id() => ball.set(new_pos),
            Event::WindowEvent {
                event: WindowEvent::MouseInput{ state, button, .. },
                window_id,
            } if window_id == window.id() => {
                if button == MouseButton::Right && state == ElementState::Pressed {
                    ball.velocity = (ball.pos - input.mouse).map_length(|m| (m/(ball.radius as f64)) * 30.0 * SCALE);
                }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let Err(err) = pixels.render() {
                    println!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }

        input.handle(&window, &event);
        if input.mouse_down() {
            ball.move_ball(input.mouse_diff());
        } else {
            ball.tick(&window);
        }
        ball.update_window(&window);
    });
}

pub struct InputHandler {
    mouse_prev: Pos,
    mouse: Pos,
    mouse_down: bool,
}

impl InputHandler {
    fn new() -> Self {
        InputHandler {
            mouse_prev: Pos::zero(),
            mouse: Pos::zero(),
            mouse_down: false,
        }
    }

    fn handle(&mut self, window: &Window, event: &Event<()>) {
        self.mouse_prev = self.mouse;
        match event {
            Event::WindowEvent { event, window_id } if *window_id == window.id() => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    let window_pos = window.outer_position().unwrap();
                    self.mouse = Pos::from(window_pos) + Pos::from(*position);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if *button == MouseButton::Left {
                        self.mouse_down = *state == ElementState::Pressed;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn mouse_diff(&self) -> Pos {
        self.mouse - self.mouse_prev
    }

    fn mouse_down(&self) -> bool {
        self.mouse_down
    }
}

pub struct Ball {
    pos: Pos,
    velocity: Pos,
    last_tick: Instant,
    last_update: Pos,
    /// Measured in pixels
    radius: u32,
    start_drag: Option<Pos>,
}

impl Ball {
    /// Constructs a ball with a radius in meters
    fn new(radius: f64) -> Self {
        Ball {
            radius: (radius * SCALE) as u32,
            pos: Pos::zero(),
            last_update: Pos::new(f64::MIN, f64::MIN),
            velocity: Pos::zero(),
            last_tick: Instant::now(),
            start_drag: None,
        }
    }

    fn set(&mut self, pos: impl Into<Pos>) {
        let pos = self.corner_to_center(pos.into());
        if (pos - self.pos).len() > 5.0 {
            // println!("Set pos: {pos:?}, {:?}", self.pos);
            // self.pos = pos;
        }
    }

    fn move_ball(&mut self, diff: impl Into<Pos>) {
        let diff = diff.into();
        self.pos += diff;
        let now = Instant::now();
        match &mut self.start_drag {
            Some(x) => {
                self.velocity = self.pos - *x;
                *x += self.velocity * 0.99 * (now - self.last_tick).as_secs_f64(); // Slowly catch up the start pos
                self.velocity *= 4.0;
            }
            None => {
                self.start_drag = Some(self.pos);
            }
        }
        self.last_tick = now;
    }

    fn tick(&mut self, window: &Window) -> Option<()> {
        self.start_drag = None;
        let bounds: Vec2<f64> = window.current_monitor()?.size().into();
        let current_time = Instant::now();
        let delta = (current_time - self.last_tick).as_secs_f64();
        self.velocity += Pos::new(0.0, 9.8) * SCALE * delta; // Positive is down

        let radius = self.radius as f64;
        let new_y = self.pos.clone().y.clamp(radius, bounds.y-radius);
        let new_x = self.pos.clone().x.clamp(radius, bounds.x-radius);

        if new_y != self.pos.y {
            self.pos.y = new_y;
            self.velocity.y *= -0.6; // Boeing
        }

        if new_x != self.pos.x {
            self.pos.x = new_x;
            self.velocity.x *= -0.6;
        }

        self.velocity -= self.velocity * 0.05 * delta; // drag

        self.pos += self.velocity * delta;
        self.last_tick = current_time;

        Some(())
    }

    fn size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.radius * 2, self.radius * 2)
    }

    fn update_window(&mut self, window: &Window) {
        if (self.pos - self.last_update).len() > 1.0 {
            window.set_outer_position(Position::Physical(self.center_to_corner(self.pos).into()));
            self.last_update = self.pos;
        }
    }

    fn center_to_corner(&self, pos: Pos) -> Pos {
        pos - Pos::from(self.size())/2.0
    }

    fn corner_to_center(&self, pos: Pos) -> Pos {
        pos + Pos::from(self.size())/2.0
    }
}
