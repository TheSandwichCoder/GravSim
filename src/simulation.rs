use crate::particle::*;
use crate::particle_container::*;
use crate::vector::*;

#[derive(Clone)]
pub struct SimulationSpecs {
    dt: f32,
    sim_time: f32, // ms
    n_sub_steps: u32,

    // dependent variables
    n_steps: u32,
}

impl SimulationSpecs {
    pub fn default() -> SimulationSpecs {
        return SimulationSpecs {
            dt: 0.1,
            sim_time: 10.0,
            n_sub_steps: 5,

            n_steps: (10.0 / 0.1) as u32,
        };
    }

    pub fn update_dependents(&mut self) {
        self.n_steps = (self.sim_time / self.dt) as u32;
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt;
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

pub struct Simulation {
    container: Container,
    sim_info: SimulationSpecs,
}

impl Simulation {
    pub fn construct(sim_specs: &SimulationSpecs) -> Simulation {
        return Simulation {
            container: Container::new(),
            sim_info: sim_specs.clone(),
        };
    }

    pub fn run(&mut self) {
        println!("SIM START");
        self.container.add_particle();
        for sim_step in 0..self.sim_info.n_steps {
            for sub_step in 0..self.sim_info.n_sub_steps {
                println!("{} {}", sim_step, sub_step);
                let p1 = self.container.index(0);
                p1.apply_force(Vec2::new(1.0, 0.0));
                self.container.integrate_particles(0.01);
                self.container.show_particles();
            }
        }
        println!("SIM END");
    }
}
