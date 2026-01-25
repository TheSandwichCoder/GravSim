use crate::vector::Vec2;
use std::io::Write;

pub fn show_progress(done: usize, sub_batch_i: usize, total: usize) {
    let bar_width = 40;
    let filled = done * bar_width / total;
    let empty = bar_width - filled;

    print!(
        "\r[{}{}] {:>3}% ({}/{}) (load-batch i: {})",
        "=".repeat(filled),
        " ".repeat(empty),
        done * 100 / total,
        done,
        total,
        sub_batch_i
    );
    std::io::stdout().flush().unwrap();
}

// not even gonna hide this: ts is all chatgpt cuh
#[inline]
fn clamp_u32(v: i64, lo: i64, hi: i64) -> u32 {
    v.clamp(lo, hi) as u32
}

// Map float in [min,max] to integer in [0, (1<<bits)-1]
#[inline]
fn quantize(v: f32, min: f32, max: f32, bits: u32) -> u32 {
    let levels = (1u64 << bits) - 1;
    if max <= min {
        return 0;
    }
    let t = ((v - min) / (max - min)).clamp(0.0, 1.0);
    (t * (levels as f32) + 0.5) as u32
}

// "Part 1 by 1": spread bits of x so they occupy even positions: 00000000abcdefgh -> a0b0c0...
#[inline]
fn part1by1(mut x: u32) -> u32 {
    x &= 0x0000_FFFF;
    x = (x | (x << 8)) & 0x00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333;
    x = (x | (x << 1)) & 0x5555_5555;
    x
}

#[inline]
fn morton2D(ix: u32, iy: u32) -> u32 {
    part1by1(ix) | (part1by1(iy) << 1)
}

// Example: compute key for particle position
#[inline]
pub fn morton_key(pos: Vec2) -> u32 {
    let bits = 16;
    let ix = quantize(pos.x, -1.0, 1.0, bits);
    let iy = quantize(pos.y, -1.0, 1.0, bits);
    morton2D(ix, iy)
}
