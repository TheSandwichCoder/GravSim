use core::time;
use std::{io::Write, time::Instant};

pub struct ProgressBar {
    total: u32,
    done: u32,
    prev_time: Instant,
    iteration_elapsed: u128,
}

impl ProgressBar {
    pub fn new(total: u32) -> Self {
        return ProgressBar {
            total,
            done: 0,
            prev_time: Instant::now(),
            iteration_elapsed: 0,
        };
    }

    pub fn increment(&mut self) {
        self.done += 1;
        self.iteration_elapsed = self.prev_time.elapsed().as_millis();
        self.prev_time = Instant::now();
    }

    pub fn refresh(&self) {
        let bar_width = 40;
        let filled = self.done * bar_width / self.total;
        let empty = bar_width - filled;

        let time_left = self.iteration_elapsed * (self.total - self.done) as u128 / 1000;

        let minutes_left = time_left / 60;
        let seconds_left = time_left % 60;

        print!(
            "\r[{}{}] {:>3}% ({}/{}) Time Left: ({}m{}s)",
            "=".repeat(filled as usize),
            " ".repeat(empty as usize),
            self.done * 100 / self.total,
            self.done,
            self.total,
            minutes_left,
            seconds_left
        );
        std::io::stdout().flush().unwrap();
    }
}

// pub fn show_progress(done: usize, sub_batch_i: usize, total: usize) {}
