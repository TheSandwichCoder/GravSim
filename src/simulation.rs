use crate::functions::show_progress;
use crate::particle::*;
use crate::particle_container::*;
use crate::vector::*;

#[derive(Clone)]
pub struct SimulationSpecs {
    dt: f32,
    sim_time: f32, // ms
    n_sub_steps: u32,
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
            is_recording: false,

            n_steps: (10.0 / 0.1) as u32,
            sub_step_dt: 0.1 / 5.0,
        };
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

    pub fn record_step(&mut self, container: &Container) {
        let mut particle_step: Vec<ParticleData> = Vec::new();
        for particle in &container.particles {
            particle_step.push(ParticleData::new(&particle));
        }
        self.data.push(RecorderStep::new(&container));
    }

    pub fn export_recording(&self, path: &str) {
        let mut recording_string = String::new();
        for particle_step in &self.data {
            for particle in &particle_step.particle_data {
                recording_string
                    .push_str(&format!("{} {},", particle.position.x, particle.position.y));
            }
            recording_string.push_str("\n");
        }

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

        for i in 0..10000 {
            self.container.add_particle();
        }

        for sim_step in 0..self.sim_info.n_steps {
            for sub_step in 0..self.sim_info.n_sub_steps {
                // self.container.apply_gravity();

                self.container
                    .integrate_particles(self.sim_info.sub_step_dt);

                // self.container
                //     .container_collisions(self.sim_info.sub_step_dt);

                self.container.interparticle_gravity();

                for i in 0..5 {
                    self.container
                        .particle_collisions_slow(self.sim_info.sub_step_dt);
                }

                // self.container
                //     .container_collisions(self.sim_info.sub_step_dt);
            }
            if (self.sim_info.is_recording) {
                self.sim_recorder.record_step(&self.container);
            }

            show_progress(sim_step as usize, 0, self.sim_info.n_steps as usize);
        }
        println!("SIM END");
    }
}
