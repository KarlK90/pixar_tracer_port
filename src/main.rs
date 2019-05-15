extern crate pathtracer;
extern crate rayon;

use std::fs::File;
use std::io::{BufWriter, Error, Write};
use rand::random;

use rayon::prelude::*;

use pathtracer::{trace, Vec3d};

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

    let mut pixels = vec![0u8; h as usize * w as usize * 3];
    pixels
        .par_chunks_mut(3)
        .into_par_iter()
        .rev()
        .enumerate()
        .for_each(|(index, rgb)| {
            let x = (index % w as usize) as f32;
            let y = (index / w as usize) as f32;
            let mut color = Vec3d::default();
            for _ in (0..samples_count as i32).rev() {
                color = color
                    + trace(
                    position,
                    !(goal
                        + left * (x - w / 2.0 + random::<f32>())
                        + up * (y - h / 2.0 + random::<f32>())),
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
            rgb[0] = color.x as u8;
            rgb[1] = color.y as u8;
            rgb[2] = color.z as u8;
        });
    file.write_all(&pixels)?;
    file.flush()?;
    Ok(())
} // Andrew Kensler
