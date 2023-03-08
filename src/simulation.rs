use rand::rngs::ThreadRng;
use rand::Rng;

use std::rc::Rc;
use std::time::Instant;
use std::vec;

use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::Font;
use speedy2d::font::FormattedTextBlock;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;
use speedy2d::window::WindowHelper;
use speedy2d::Graphics2D;

use crate::body::{Body, BodyType};
use crate::particle::Particle;
use crate::physics::Physics;
use crate::shape::Shape;
use crate::vertices::ParticleCircle;
use crate::vertices::{CubeShape, SphereShape};
use crate::debug;
use crate::camera::Camera;

pub struct Simulation {
    pub camera: Camera,
    background_color: Color,
    center_point: (f64, f64),
    objects: Vec<BodyType>,
    timestep: f64,
    font: Font,
    fps_txp: (f32, f32),
    fps_txc: Color,
}

impl Simulation {
    pub fn new(camera: Camera, center_point: (f64, f64)) -> Simulation {
        let objects: Vec<BodyType> = vec![];
        let timestep: f64 = 1.0 / 10000.0;
        let bytes: &[u8; 367112] = include_bytes!("../fonts/arial.ttf");
        let font: Font = Font::new(bytes).unwrap();
        let background_color = Color::from_rgb(0.15, 0.15, 0.15);

        let fx: f32 = center_point.0 as f32 - 300.0;
        let fy: f32 = center_point.1 as f32 - 300.0;

        let fps_txp: (f32, f32) = (fx, fy);
        let fps_txc: Color = Color::from_rgb(1.0, 1.0, 1.0);

        Simulation {
            camera,
            background_color,
            center_point,
            objects,
            timestep,
            font,
            fps_txp,
            fps_txc,
        }
    }

    pub fn add_center_cube(&mut self) {
        let x: f64 = self.center_point.0;
        let y: f64 = self.center_point.1;
        let z: f64 = 0.0;
        let mass: f64 = 5_000_000.0;
        let shape: Vec<[f64; 3]> = CubeShape::new().get_shape();
        let color = (1.0, 0.4, 0.4);
        let scale: f64 = mass / 50_000.0;

        let mut shape: Shape = Shape::new(shape);
        shape.set_color(color.0, color.1, color.2);

        let physics: &mut Physics = shape.physics();
        physics.set_position(x, y, z);
        physics.set_mass(mass);
        physics.set_scale(scale);
        physics.set_spin_velocity(0.0, 0.0, 0.0);

        self.objects.push(BodyType::Shape(shape));
    }

    pub fn add_center_sphere(&mut self) {
        let x: f64 = self.center_point.0;
        let y: f64 = self.center_point.1;
        let z: f64 = 0.0;
        let mass: f64 = 10_000_000.0;
        let shape: Vec<[f64; 3]> = SphereShape::new(20, 20, 20).get_shape();
        let color = (0.8, 0.3, 0.3);
        let scale: f64 = mass / 50_000.0;

        let mut shape: Shape = Shape::new(shape);
        shape.set_color(color.0, color.1, color.2);

        let physics: &mut Physics = shape.physics();
        physics.set_position(x, y, z);
        physics.set_mass(mass);
        physics.set_scale(scale);
        physics.set_spin_velocity(0.0, 0.0, 0.0);

        self.objects.push(BodyType::Shape(shape));
    }


    pub fn add_center_particle(&mut self) {
        let x: f64 = self.center_point.0;
        let y: f64 = self.center_point.1;
        let z: f64 = 0.0;
        let mass: f64 = 10_000_000.0;
        let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
        let color = (0.8, 0.3, 0.3);
        let scale: f64 = mass / 50_000.0;

        let mut particle: Particle = Particle::new(shape);
        particle.set_color(color.0, color.1, color.2);

        let physics: &mut Physics = particle.physics();
        physics.set_position(x, y, z);
        physics.set_mass(mass);
        physics.set_scale(scale);
        physics.set_spin_velocity(0.0, 0.0, 0.0);

        self.objects.push(BodyType::Particle(particle));
    }






    pub fn add_particle_t1(&mut self, z: f64) {
        let px = -1000.0 + self.center_point.0;
        let py = 40.0 + self.center_point.1;
        let pz = z;

        let mass = 500_000_000.0;
        let shape = vec![[0.0, 0.0, 0.0]];
        let scale = 80.0;

        let vx = 5_000.0;
        let vy = 0.0;

        let mut p = Particle::new(shape);
        p.set_color(0.8, 0.2, 0.2);

        let physics: &mut Physics = p.physics();
        physics.set_position(px, py, pz);
        physics.set_velocity(vx, vy, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t2(&mut self, z: f64) {
        let px = 500.0 + self.center_point.0;
        let py = 0.0 + self.center_point.1;
        let pz = z;

        let mass = 20.0;
        let shape = vec![[0.0, 0.0, 0.0]];
        let scale = 20.0;

        let vx = -5_000.0;
        let vy = 0.0;

        let mut p = Particle::new(shape);
        let physics: &mut Physics = p.physics();
        physics.set_position(px, py, pz);
        physics.set_velocity(vx, vy, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t3(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let x: f64 = self.center_point.0 - 0.0;
        let y: f64 = self.center_point.1 - 0.0;
        let z: f64 = 0.0;

        let mass: f64 = rng.gen_range(1.0..30.0);
        let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
        let scale: f64 = mass / 10.0;

        let mut p: Particle = Particle::new(shape);
        let physics: &mut Physics = p.physics();
        physics.set_position(x, y, z);
        physics.set_velocity(-10.0, -30.0, 0.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(p));
    }

    pub fn add_particle_t4(&mut self, z: f64) {
        let particles = ParticleCircle::new(100).generate(0.0, 0.0);

        for particle in particles {
            let px: f64 = particle[0] + self.center_point.0;
            let py: f64 = particle[1] + self.center_point.1;
            let pz: f64 = z;

            let mass: f64 = particle[2];
            let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
            let scale = particle[2];

            let mut p = Particle::new(shape);
            let physics: &mut Physics = p.physics();
            physics.set_position(px, py, pz);
            physics.set_mass(mass);
            physics.set_scale(scale);
            self.objects.push(BodyType::Particle(p));
        }
    }

    pub fn add_orbiting_object(&mut self) {
        let mut rng: ThreadRng = rand::thread_rng();
        let x_rnd: f64 = rng.gen_range(200.0..400.0);
        let y_rnd: f64 = rng.gen_range(200.0..400.0);

        let x: f64 = self.center_point.0 - x_rnd;
        let y: f64 = self.center_point.1 - y_rnd;
        let z: f64 = -500.0;

        let mass: f64 = rng.gen_range(10.0..50.0);
        let shape: Vec<[f64; 3]> = vec![[0.0, 0.0, 0.0]];
        let scale: f64 = 100.0;

        let mut particle: Particle = Particle::new(shape);
        let physics: &mut Physics = particle.physics();

        physics.set_position(x, y, z);
        physics.set_velocity(10.0, 30.0, 1.0);
        physics.set_mass(mass);
        physics.set_scale(scale);

        self.objects.push(BodyType::Particle(particle));
    }

    pub fn setup_gravity_configuration(&mut self) {
        self.add_center_particle();

        for _ in 0..100 {
            self.add_orbiting_object();
        }
    }

    pub fn setup_collision_configuration(&mut self) {
        let z = -10.0;

        self.add_particle_t1(z);
        self.add_particle_t2(z);
        self.add_particle_t4(z);


        // for _ in 0..1500 {
        //     self.add_particle_t3();
        // }
    }

    pub fn compute_objects(&mut self, graphics: &mut Graphics2D) {
        let mut objects_cl: Vec<BodyType> = self.objects.clone();
        for (i, pl1) in self.objects.iter_mut().enumerate() {
            let pl1_physics = pl1.physics();
            for (j, pl2) in objects_cl.iter_mut().enumerate() {
                if i == j {
                    continue;
                }

                let pl2_physics = pl2.physics();
                pl1_physics.apply_forces(pl2_physics, self.timestep);
            }
            pl1_physics.update(self.timestep);
            pl1.draw(graphics, &self.camera);
        }

        // self.debug_z_height();
    }

    // fn debug_z_height(&mut self) {
    //     for pl1 in self.objects.iter_mut() {
    //         let physics = pl1.physics();
    //         if physics.mass > 1_000_000.0 {
    //             println!("{:?}{:?}", "z:", physics.position.z)
    //         }
    //     }
    // }

    fn get_text_block(&self, string: String) -> Rc<FormattedTextBlock> {
        let text_options: TextOptions = TextOptions::new();
        let text_scale: f32 = 32.0;
        let text_block: Rc<FormattedTextBlock> =
            self.font.layout_text(&string, text_scale, text_options);
        text_block
    }

    pub fn write_fps(&self, frame_time: f32, graphics: &mut Graphics2D) {
        let fps_str: String = format!("{:.2} FPS", 1.0 / frame_time);
        let fps_block: Rc<FormattedTextBlock> = self.get_text_block(fps_str);
        let x: f32 = self.fps_txp.0;
        let y: f32 = self.fps_txp.1;
        let position: Vector2<f32> = Vector2::new(x, y);
        graphics.draw_text(position, self.fps_txc, &fps_block);
    }

    pub fn simulate(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(self.background_color);
        let frame_st: Instant = Instant::now();
        self.compute_objects(graphics);
        let frame_time: f32 = Instant::now().duration_since(frame_st).as_secs_f32();
        self.write_fps(frame_time, graphics);
        // debug::sleep(0.1);
        helper.request_redraw();
    }
}
