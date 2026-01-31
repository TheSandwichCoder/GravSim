use std::thread::spawn;

use rand::random;

use crate::constants::*;
use crate::functions::*;
use crate::particle::*;
use crate::qtree::QuadTree;
use crate::simulation::NORMAL_DISTRIBUTION;
use crate::simulation::SimulationSpecs;
use crate::vector::*;

pub struct Container {
    pub particles: Vec<Particle>,
    pub cached_potential_collisions: Vec<Vec<usize>>,
    pub quadtree: QuadTree,
}

impl Container {
    pub fn new() -> Container {
        return Container {
            particles: Vec::new(),
            cached_potential_collisions: Vec::new(),
            quadtree: QuadTree::new(),
        };
    }

    pub fn construct_quadtree(&mut self) {
        self.quadtree.reset();

        self.particles.sort_unstable_by_key(|p| morton_key(p.pos));

        for particle_i in 0..self.particles.len() {
            self.quadtree.add_particle(&self.particles, particle_i);
        }
    }

    pub fn init_particles(&mut self, info: &SimulationSpecs) {
        let spawn_radius = info.get_spawn_radius();
        let spawn_radius_squared = info.get_spawn_radius_squared();

        for _particle_i in 0..info.get_n_particles() {
            let mut new_particle = Particle::new();

            let mut random_pos = Vec2::zero();
            let distribution = info.get_distribution();
            if distribution == NORMAL_DISTRIBUTION {
                random_pos = Vec2::rand_normal() * spawn_radius;
            } else {
                random_pos = Vec2::rand_uniform() * spawn_radius;
            }

            while random_pos.length_squared() > spawn_radius_squared {
                if distribution == NORMAL_DISTRIBUTION {
                    random_pos = Vec2::rand_normal() * spawn_radius;
                } else {
                    random_pos = Vec2::rand_uniform() * spawn_radius;
                }
            }

            new_particle.set_pos(random_pos);
            self.particles.push(new_particle);
            self.cached_potential_collisions.push(Vec::new());
        }

        self.construct_quadtree();
        self.particle_collision(5, 1, 1.0);

        for particle in &mut self.particles {
            particle.set_vel(particle.pos.perp() * 0.0001);
        }
    }

    pub fn add_particle(&mut self) {
        let mut new_particle = Particle::new();
        let mut random_pos = Vec2::rand_uniform();

        let spawn_radius = 0.5;

        while random_pos.length_squared() > spawn_radius * spawn_radius {
            random_pos = Vec2::rand_uniform();
        }

        let length_ratio = random_pos.length();

        new_particle.set_pos(random_pos * length_ratio * length_ratio);

        new_particle.set_vel(random_pos.perp() * 0.00003);
        new_particle.set_density(1.0);
        self.particles.push(new_particle);
        self.cached_potential_collisions.push(Vec::new());
    }

    pub fn interparticle_gravity_quadratic(&mut self) {
        let n_particles = self.particles.len();
        for pt1_i in 0..n_particles {
            for pt2_i in (pt1_i + 1)..n_particles {
                let delta = self.particles[pt2_i].pos - self.particles[pt1_i].pos;
                let delta_length_squared = delta.length_squared();

                if delta_length_squared != 0.0 {
                    let attract_vec = delta.normalize() / (delta_length_squared)
                        * GRAVITY_CONST
                        * self.particles[pt1_i].mass
                        * self.particles[pt2_i].mass;

                    self.particles[pt1_i].apply_force(attract_vec);
                    self.particles[pt2_i].apply_force(-attract_vec);
                }
            }
        }
    }

    pub fn interparticle_gravity(&mut self) {
        for particle in &mut self.particles {
            let grav_force = self.quadtree.get_grav_force(particle.pos) * GRAVITY_CONST;
            // println!("{}", grav_force);

            particle.apply_force(grav_force);
        }
    }

    pub fn resolve_collision(&mut self, pt1_i: usize, pt2_i: usize) {
        let mut delta = self.particles[pt2_i].pos - self.particles[pt1_i].pos;

        let mut dist2 = delta.length_squared();
        let min_dis = self.particles[pt1_i].radius + self.particles[pt2_i].radius;

        if dist2 == 0.0 {
            delta = Vec2::rand_uniform();
            dist2 = delta.length_squared();
        }

        if dist2 < min_dis * min_dis {
            let dist = dist2.sqrt();
            let n = delta / dist;

            let pen = min_dis - dist;

            let corr = n * (pen * 0.5);
            self.particles[pt1_i].pos -= corr;
            self.particles[pt2_i].pos += corr;

            self.particles[pt1_i].n_collisions += 1;
            self.particles[pt2_i].n_collisions += 1;
            self.particles[pt1_i].n_total_collisions += 1;
            self.particles[pt2_i].n_total_collisions += 1;
        }
    }

    pub fn particle_collision(
        &mut self,
        n_collision_steps: u32,
        n_update_cache_steps: u32,
        dt: f32,
    ) {
        let n_particles = self.particles.len();

        for particle in &mut self.particles {
            particle.n_collisions = 0;
        }

        let mut collision_particles_i: Vec<usize> = (0..n_particles).collect();

        let mut delta_n_collisions = 0;

        for coll_step_i in 0..n_collision_steps {
            if coll_step_i % n_update_cache_steps == 0 {
                for pt1_i in &collision_particles_i {
                    self.cached_potential_collisions[*pt1_i].clear();
                    self.quadtree.idx_bound(
                        &self.particles[*pt1_i].get_bound(),
                        &mut self.cached_potential_collisions[*pt1_i],
                    );
                    // self.cached_potential_collisions
                    //     .push();
                }
            }

            for pt1_i in &collision_particles_i {
                self.particles[*pt1_i].n_collisions = 0;
                for pt2_ii in 0..self.cached_potential_collisions[*pt1_i].len() {
                    if *pt1_i == self.cached_potential_collisions[*pt1_i][pt2_ii] {
                        continue;
                    }

                    self.resolve_collision(
                        *pt1_i,
                        self.cached_potential_collisions[*pt1_i][pt2_ii],
                    );
                }
            }
            // println!("{}", collision_particles_i.len());

            let mut new_collision_particles = Vec::new();

            for pt1_i in &collision_particles_i {
                if self.particles[*pt1_i].n_collisions > 0 {
                    new_collision_particles.push(*pt1_i);
                }
            }

            delta_n_collisions = collision_particles_i.len() - new_collision_particles.len();
            if delta_n_collisions < 10 {
                break;
            }

            collision_particles_i = new_collision_particles;
        }
    }

    pub fn particle_collisions_quadratic(&mut self, dt: f32) {
        let n_particles = self.particles.len();

        for pt1_i in 0..n_particles {
            for pt2_i in (pt1_i + 1)..n_particles {
                self.resolve_collision(pt1_i, pt2_i);
            }
        }
    }

    pub fn container_collisions(&mut self, dt: f32) {
        for particle in &mut self.particles {
            let particle_vel = particle.get_vel();

            if particle.pos.x - particle.radius < -1.0 {
                particle.pos = Vec2::new(-1.0 + particle.radius, particle.pos.y);
                particle.set_vel(Vec2::new((particle_vel.x).abs(), particle_vel.y));
            } else if particle.pos.x + particle.radius > 1.0 {
                particle.pos = Vec2::new(1.0 - particle.radius, particle.pos.y);
                particle.set_vel(Vec2::new(-(particle_vel.x).abs(), particle_vel.y));
            }

            if particle.pos.y - particle.radius < -1.0 {
                particle.pos = Vec2::new(particle.pos.x, -1.0 + particle.radius);
                particle.set_vel(Vec2::new(particle_vel.x, (particle_vel.y).abs()));
            } else if particle.pos.y + particle.radius > 1.0 {
                particle.pos = Vec2::new(particle.pos.x, 1.0 - particle.radius);
                particle.set_vel(Vec2::new(particle_vel.x, -(particle_vel.y).abs()));
            }
        }
    }

    pub fn apply_gravity(&mut self) {
        for particle in &mut self.particles {
            particle.apply_force(Vec2::new(0.0, GLOBAL_GRAVITY_CONST) * particle.mass);
        }
    }

    pub fn integrate_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.integrate(dt);
        }
    }

    pub fn index(&mut self, i: usize) -> &mut Particle {
        return &mut self.particles[i];
    }

    pub fn show_particles(&self) {
        for particle in &self.particles {
            println!("{}\n", particle);
        }
    }
}
