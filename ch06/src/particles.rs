use graphics::math::{add, mul_scalar, Vec2d};
use piston_window::*;
use rand::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::time::{Duration, Instant};

struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
  unsafe fn alloc(
    &self,
    layout: Layout,
  ) -> *mut u8 {
    let time_start: Instant = Instant::now();
    let pointer: *mut u8 = System.alloc(layout);
    let time_end: Instant = Instant::now();
    let time_taken: Duration = time_end - time_start;
    let bytes_requested: usize = layout.size();

    eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
    pointer
  }

  unsafe fn dealloc(
    &self,
    ptr: *mut u8,
    layout: Layout,
  ) {
    System.dealloc(ptr, layout)
  }
}

struct Particle {
  height: f64,
  width: f64,
  position: Vec2d<f64>,
  velocity: Vec2d<f64>,
  acceleration: Vec2d<f64>,
  color: [f32; 4],
}

impl Particle {
  fn new(world: &World) -> Self {
    let mut rng: ThreadRng = thread_rng();

    let x = rng.gen_range(0.0..=world.width);
    let y = world.height;
    let x_velocity = 0.0;
    let y_velocity = rng.gen_range(-2.0..0.0);
    let x_acceleration = 0.0;
    let y_acceleration = rng.gen_range(0.0..0.15);

    Self {
      height: 4.0,
      width: 4.0,
      position: Vec2d::from([x, y]),
      velocity: Vec2d::from([x_velocity, y_velocity]),
      acceleration: [x_acceleration, y_acceleration],
      color: [1.0, 1.0, 1.0, 0.99],
    }
  }

  fn update(&mut self) {
    self.velocity = add(self.velocity, self.acceleration);

    self.position = add(self.position, self.velocity);

    self.acceleration = mul_scalar(self.acceleration, 0.7);

    self.color[3] *= 0.995;
  }
}

struct World {
  current_turn: u64,
  particles: Vec<Box<Particle>>,
  height: f64,
  width: f64,
  rng: ThreadRng,
}

impl World {
  fn new(
    width: f64,
    height: f64,
  ) -> Self {
    Self {
      current_turn: 0,
      particles: Vec::<Box<Particle>>::new(),
      height,
      width,
      rng: thread_rng(),
    }
  }

  fn add_shapes(
    &mut self,
    n: i32,
  ) {
    for _ in 0..n.abs() {
      let particle = Particle::new(self);
      let boxed_particle = Box::new(particle);
      self.particles.push(boxed_particle);
    }
  }

  fn remove_shapes(
    &mut self,
    n: i32,
  ) {
    for _ in 0..n.abs() {
      let mut to_delete = None;

      let particle_iter = self.particles.iter().enumerate();

      for (i, particle) in particle_iter {
        if particle.color[3] < 0.02 {
          to_delete = Some(i);
          break;
        }
      }

      if let Some(p) = to_delete {
        self.particles.remove(p);
      } else if !self.particles.is_empty() {
        self.particles.remove(0);
      }
    }
  }

  fn update(&mut self) {
    let n = self.rng.gen_range(-3..=3);

    if n > 0 {
      self.add_shapes(n);
    } else {
      self.remove_shapes(n);
    }

    self.particles.shrink_to_fit();

    for shape in &mut self.particles {
      shape.update();
    }
    self.current_turn += 1;
  }
}

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

fn main() {
  let (width, height) = (1280.0, 960.0);
  let mut window: PistonWindow = WindowSettings::new("particles", [width, height])
    .exit_on_esc(true)
    .build()
    .expect("The window could not be created.");

  let mut world = World::new(width, height);
  world.add_shapes(10_000);

  while let Some(event) = window.next() {
    world.update();

    window.draw_2d(&event, |ctx, renderer, _device| {
      clear([0.15, 0.17, 0.17, 0.9], renderer);

      for p in &mut world.particles {
        let size = [p.position[0], p.position[1], p.width, p.height];
        rectangle(p.color, size, ctx.transform, renderer);
      }
    });
  }
}
