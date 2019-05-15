extern crate pathtracer;
extern crate rayon;

use std::fs::File;
use std::io::{BufWriter, Error, Write};

use rayon::prelude::*;

use pathtracer::{random_val, trace, Vec3d};

fn main() -> Result<(), Error> {
    let output = File::create("pixar.ppm")?;
    let mut file = BufWriter::new(output);
    let w = 960.0;
    let h = 540.0;
    let samples_count = 16.0;

    let position = Vec3d {
        x: -22.0,
        y: 5.0,
        z: 25.0,
    };
    let goal = !(Vec3d {
        x: -3.0,
        y: 4.0,
        z: 0.0,
    } - position);
    let left = !Vec3d {
        x: goal.z,
        y: 0.0,
        z: -goal.x,
    } * ((1.0 / w) as f32);

    // Cross-product to get the up vector
    let up = Vec3d {
        x: goal.y * left.z - goal.z * left.y,
        y: goal.z * left.x - goal.x * left.z,
        z: goal.x * left.y - goal.y * left.x,
    };

    file.write_all(format!("P6 {} {} 255 ", w as u32, h as u32).as_bytes())?;

    let mut pixels = Vec::<(f32, f32)>::with_capacity(h as usize * w as usize);
    for y in (0..h as u32).rev() {
        for x in (0..w as u32).rev() {
            pixels.push((x as f32, y as f32));
        }
    }

    let bytes: Vec<u8> = pixels.iter().flat_map(|(x, y)| {
        let mut color = Vec3d::default();
        for _ in (0..samples_count as i32).rev() {
            color = color
                + trace(
                position,
                !(goal
                    + left * (*x as f32 - w / 2.0 + random_val())
                    + up * (*y as f32 - h / 2.0 + random_val())),
            );
        }
        // Reinhard tone mapping
        color = color * (1.0 / samples_count) + 14.0 / 241.0;
        let o = color + 1.0;
        color = Vec3d {
            x: color.x / o.x,
            y: color.y / o.y,
            z: color.z / o.z,
        } * 255.0;
        vec![color.x as u8, color.y as u8, color.z as u8]
    }).collect();

    file.write_all(&bytes)?;
    file.flush()?;
    Ok(())
} // Andrew Kensler
