use crate::constants::{EPS_SQUARED, THETA_SQUARED};
use crate::particle::{self, Particle};
use crate::vector::Vec2;

pub struct Bound {
    top_right: Vec2,
    bot_left: Vec2,
    size_squared: f32,
}

impl Bound {
    pub fn new(bl: Vec2, tr: Vec2) -> Self {
        let sz = tr.x - bl.x;
        return Bound {
            top_right: tr,
            bot_left: bl,
            size_squared: sz * sz,
        };
    }

    pub fn get_dim(&self) -> Vec2 {
        return self.top_right - self.bot_left;
    }

    pub fn is_overlap(&self, other: &Bound) -> bool {
        if self.top_right.x <= other.bot_left.x || self.bot_left.x >= other.top_right.x {
            return false;
        }

        if self.top_right.y <= other.bot_left.y || self.bot_left.y >= other.top_right.y {
            return false;
        }

        return true;
    }

    pub fn in_bounds(&self, pos: Vec2) -> bool {
        return pos.x >= self.bot_left.x
            && pos.x <= self.top_right.x
            && pos.y >= self.bot_left.y
            && pos.y <= self.top_right.y;
    }
}

pub struct QNode {
    bound: Bound,
    depth: u32,
    children: usize,
    next: usize,
    is_leaf: bool,
    particle_contained: i32,

    total_mass: f32,
    center_mass: Vec2,
}

impl QNode {
    pub fn new(bot_left: Vec2, bound_size: Vec2, d: u32, next: usize) -> Self {
        return QNode {
            bound: Bound::new(bot_left, bot_left + bound_size),
            depth: d,
            children: 0,
            next: next,
            is_leaf: true,
            particle_contained: -1,
            total_mass: 0.0,
            center_mass: Vec2::zero(),
        };
    }

    pub fn set_particle(&mut self, particles: &Vec<Particle>, pt_idx: i32) {
        self.particle_contained = pt_idx;

        self.total_mass += particles[pt_idx as usize].mass;
        self.center_mass = particles[pt_idx as usize].pos;
    }
}

pub struct QuadTree {
    stack: Vec<QNode>,
    dfs_stack: Vec<usize>,
}

impl QuadTree {
    pub fn new() -> Self {
        return QuadTree {
            stack: vec![QNode::new(Vec2::new(-1.0, -1.0), Vec2::new(2.0, 2.0), 0, 0)],
            dfs_stack: vec![0],
        };
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.stack
            .push(QNode::new(Vec2::new(-1.0, -1.0), Vec2::new(2.0, 2.0), 0, 0));
    }

    pub fn subdivide_node(&mut self, node_i: usize) {
        self.stack[node_i].is_leaf = false;
        self.stack[node_i].particle_contained = -1;
        self.stack[node_i].total_mass = 0.0;
        self.stack[node_i].center_mass = Vec2::zero();

        let new_bound_dim = self.stack[node_i].bound.get_dim() * 0.5;

        let last_idx = self.stack.len();

        let bound_bot_left = self.stack[node_i].bound.bot_left;
        let next_depth = self.stack[node_i].depth + 1;

        self.stack.push(QNode::new(
            bound_bot_left + Vec2::zero(),
            new_bound_dim,
            next_depth,
            last_idx + 1,
        ));
        self.stack.push(QNode::new(
            bound_bot_left + Vec2::new(new_bound_dim.x, 0.0),
            new_bound_dim,
            next_depth,
            last_idx + 2,
        ));
        self.stack.push(QNode::new(
            bound_bot_left + Vec2::new(0.0, new_bound_dim.y),
            new_bound_dim,
            next_depth,
            last_idx + 3,
        ));
        self.stack.push(QNode::new(
            bound_bot_left + new_bound_dim,
            new_bound_dim,
            next_depth,
            self.stack[node_i].next,
        ));

        self.stack[node_i].children = last_idx;
    }

    pub fn idx_pos(&self, pos: Vec2) -> usize {
        let mut curr_node_i: usize = 0;

        loop {
            if self.stack[curr_node_i].is_leaf {
                break;
            }

            let child_start_i = self.stack[curr_node_i].children;

            for child_i in child_start_i..child_start_i + 4 {
                if self.stack[child_i].bound.in_bounds(pos) {
                    curr_node_i = child_i;
                    break;
                }
            }
        }

        return curr_node_i;
    }

    pub fn idx_pos_single(&self, node_i: usize, pos: Vec2) -> usize {
        let mut curr_node_i = 0;

        let child_start_i = self.stack[node_i].children;

        for child_i in child_start_i..child_start_i + 4 {
            if self.stack[child_i].bound.in_bounds(pos) {
                curr_node_i = child_i;
                break;
            }
        }

        if (curr_node_i == 0 && !self.stack[curr_node_i].is_leaf) {
            println!("");

            for child_i in child_start_i..child_start_i + 4 {
                println!(
                    "{} {}",
                    self.stack[node_i].bound.bot_left, self.stack[node_i].bound.top_right
                );
            }

            println!("{}", pos);
        }

        return curr_node_i;
    }

    pub fn idx_bound(&mut self, other_bound: &Bound) -> Vec<usize> {
        let mut close_children: Vec<usize> = Vec::new();

        let mut node_i = 1;

        loop {
            if node_i == 0 {
                break;
            }
            let curr_node = &self.stack[node_i];

            if !curr_node.bound.is_overlap(other_bound) {
                node_i = curr_node.next;
                continue;
            }

            if curr_node.is_leaf {
                if curr_node.particle_contained != -1 {
                    close_children.push(curr_node.particle_contained as usize);
                }
                node_i = curr_node.next;
                continue;
            }

            node_i = curr_node.children
        }

        return close_children;
    }

    pub fn propogate(&mut self) {
        for nd_i in (0..self.stack.len()).rev() {
            if self.stack[nd_i].is_leaf {
                continue;
            }

            let child_start_i = self.stack[nd_i].children;

            let mut center_mass = Vec2::zero();
            let mut total_mass = 0.0;

            for child_i in child_start_i..child_start_i + 4 {
                center_mass += self.stack[child_i].center_mass * self.stack[child_i].total_mass;
                total_mass += self.stack[child_i].total_mass;
            }

            // println!("{}", center_mass / total_mass);

            self.stack[nd_i].center_mass = center_mass / total_mass;
            self.stack[nd_i].total_mass = total_mass;
        }
    }

    pub fn get_grav_force(&mut self, pos: Vec2) -> Vec2 {
        let mut force = Vec2::zero();
        // let mut dfs_stack = vec![0];

        let mut node_i = 1;

        loop {
            if node_i == 0 {
                break;
            }
            let curr_node = &self.stack[node_i];

            let delta = curr_node.center_mass - pos;

            let distance_squared = delta.length_squared();

            if distance_squared < EPS_SQUARED {
                node_i = curr_node.next;
                continue;
            }

            if curr_node.is_leaf || curr_node.bound.size_squared < distance_squared * THETA_SQUARED
            {
                let denom = (distance_squared + EPS_SQUARED) * distance_squared.sqrt();

                force += delta * (curr_node.total_mass / denom);
                node_i = curr_node.next
            } else {
                node_i = curr_node.children;
            }
        }

        return force;
    }

    pub fn add_particle(&mut self, particle_vec: &Vec<Particle>, particle_idx: usize) {
        let mut curr_node_i = self.idx_pos(particle_vec[particle_idx].pos);

        let mut counter = 0;
        // base check
        if self.stack[curr_node_i].particle_contained == -1 {
            self.stack[curr_node_i].set_particle(particle_vec, particle_idx as i32);
        }
        // there is a collision so we go through until collision is resolved
        else {
            let particle1_pos = particle_vec[particle_idx].pos;
            let particle2_pos =
                particle_vec[self.stack[curr_node_i].particle_contained as usize].pos;

            loop {
                let other_particle_contained = self.stack[curr_node_i].particle_contained;

                // depth 18 is just a safety measure to cut off recursion (also prevent floating point errors)
                // when this happens, we essentially delete a particle but I think it is fine
                if other_particle_contained == -1 || self.stack[curr_node_i].depth >= 18 {
                    self.stack[curr_node_i].set_particle(particle_vec, particle_idx as i32);
                    break;
                } else {
                    self.subdivide_node(curr_node_i);

                    let roommate_node_i = self.idx_pos_single(curr_node_i, particle2_pos);
                    self.stack[roommate_node_i]
                        .set_particle(particle_vec, other_particle_contained);

                    curr_node_i = self.idx_pos_single(curr_node_i, particle1_pos);
                }
                counter += 1;
                if counter > 50 {
                    println!("asdjfkhasdkjf");
                }
            }
        }
    }
}

/*

idx the smallest
loop:
    if it is not taken:
        take it

    else:
        subdivide node
        assign roomate new node
*/
