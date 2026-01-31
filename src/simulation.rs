use crate::functions::show_progress;
use crate::particle::*;
use crate::particle_container::*;
use crate::progress_bar::ProgressBar;
use crate::vector::*;
use std::time::Instant;

pub const UNIFORM_DISTRIBUTION: u32 = 0;
pub const NORMAL_DISTRIBUTION: u32 = 1;

#[derive(Clone)]
pub struct SimulationSpecs {
    dt: f32,
    sim_time: f32, // ms
    n_sub_steps: u32,
    n_collision_steps: u32,
    n_update_cache_steps: u32,
    n_particles: u32,
    particle_distribution: u32,
    spawn_radius: f32,
    is_recording: bool,

    // dependent variables
    n_steps: u32,
    sub_step_dt: f32,
}

impl SimulationSpecs {
    pub fn default() -> SimulationSpecs {
        return SimulationSpecs {
            dt: 0.1,
            sim_time: 10.0,
            n_sub_steps: 5,
            n_collision_steps: 3,
            n_update_cache_steps: 1,
            n_particles: 100,
            particle_distribution: UNIFORM_DISTRIBUTION,
            spawn_radius: 0.5,
            is_recording: false,

            n_steps: (10.0 / 0.1) as u32,
            sub_step_dt: 0.1 / 5.0,
        };
    }

    pub fn get_n_particles(&self) -> u32 {
        return self.n_particles;
    }

    pub fn set_distribution(&mut self, new_dist: u32) {
        self.particle_distribution = new_dist;
    }

    pub fn set_spawn_radius(&mut self, radius: f32) {
        self.spawn_radius = radius;
    }

    pub fn get_spawn_radius(&self) -> f32 {
        return self.spawn_radius;
    }

    pub fn get_spawn_radius_squared(&self) -> f32 {
        return self.spawn_radius * self.spawn_radius;
    }

    pub fn get_distribution(&self) -> u32 {
        return self.particle_distribution;
    }

    pub fn update_dependents(&mut self) {
        self.n_steps = (self.sim_time / self.dt) as u32;
        self.sub_step_dt = self.dt / self.n_sub_steps as f32;
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt;
        self.update_dependents();
    }

    pub fn set_recording(&mut self, rec: bool) {
        self.is_recording = rec;
    }

    pub fn set_n_particles(&mut self, n_particles: u32) {
        self.n_particles = n_particles;
    }

    pub fn set_n_collision_steps(&mut self, coll_steps: u32) {
        self.n_collision_steps = coll_steps;
    }

    pub fn set_update_cache_steps(&mut self, cache_steps: u32) {
        self.n_update_cache_steps = cache_steps;
    }

    pub fn set_update_cache_ratio(&mut self, ratio: f32) {
        self.n_update_cache_steps = (self.n_collision_steps as f32 * (1.0 - ratio)) as u32;
    }

    pub fn set_framerate(&mut self, fr: u32) {
        self.dt = 1.0 / fr as f32;
        self.update_dependents();
    }

    pub fn set_sim_time(&mut self, sim_time: f32) {
        self.sim_time = sim_time;
        self.update_dependents();
    }
    pub fn set_n_sub_steps(&mut self, sub_steps: u32) {
        self.n_sub_steps = sub_steps;
        self.update_dependents();
    }
}

pub struct SimulationRecorder {
    data: Vec<RecorderStep>,
}

impl SimulationRecorder {
    pub fn new() -> SimulationRecorder {
        return SimulationRecorder { data: Vec::new() };
    }

    pub fn record_step(&mut self, container: &mut Container, n_sub_steps: u32) {
        for particle in &mut container.particles {
            particle.n_total_collisions /= n_sub_steps;
        }
        self.data.push(RecorderStep::new(&container));
        for particle in &mut container.particles {
            particle.reset_collisions();
        }
    }

    pub fn export_recording(&self, path: &str) {
        let mut recording_string = String::new();

        println!("EXPORTING RECORDING");

        let mut progress_bar = ProgressBar::new(self.data.len() as u32);
        progress_bar.refresh();
        for particle_step in &self.data {
            for data in &particle_step.particle_data {
                recording_string.push_str(&format!(
                    "{} {} {} {},",
                    data.position.x, data.position.y, data.speed, data.n_collisions
                ));
            }
            recording_string.push_str("\n");
            progress_bar.increment();
            progress_bar.refresh();
        }
        println!("\nSIM END");

        std::fs::write(path, recording_string).expect("Unable to write file");
    }
}

pub struct RecorderStep {
    particle_data: Vec<ParticleData>,
}

impl RecorderStep {
    pub fn new(container: &Container) -> RecorderStep {
        let mut particle_data: Vec<ParticleData> = Vec::new();
        for particle in &container.particles {
            particle_data.push(ParticleData::new(&particle));
        }

        return RecorderStep { particle_data };
    }
}

pub struct Simulation {
    pub container: Container,

    pub sim_info: SimulationSpecs,
    pub sim_recorder: SimulationRecorder,
}

impl Simulation {
    pub fn construct(sim_specs: &SimulationSpecs) -> Simulation {
        return Simulation {
            container: Container::new(),
            sim_info: sim_specs.clone(),
            sim_recorder: SimulationRecorder::new(),
        };
    }

    pub fn run(&mut self) {
        println!("SIM START");

        self.container.init_particles(&self.sim_info);

        let mut progress_bar = ProgressBar::new(self.sim_info.n_steps);
        progress_bar.refresh();

        for _sim_step_i in 0..self.sim_info.n_steps {
            for _sub_step_i in 0..self.sim_info.n_sub_steps {
                self.container
                    .integrate_particles(self.sim_info.sub_step_dt);

                self.container
                    .container_collisions(self.sim_info.sub_step_dt);

                self.container.construct_quadtree();
                self.container.quadtree.propogate_mass();

                self.container.interparticle_gravity();

                self.container.particle_collision(
                    self.sim_info.n_collision_steps,
                    self.sim_info.n_update_cache_steps,
                    self.sim_info.sub_step_dt,
                );

                self.container
                    .container_collisions(self.sim_info.sub_step_dt);
            }

            if self.sim_info.is_recording {
                self.sim_recorder
                    .record_step(&mut self.container, self.sim_info.n_sub_steps);
            }

            progress_bar.increment();
            progress_bar.refresh();
        }
        println!("\nSIM END");
    }
}
