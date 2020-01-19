#![feature(core_intrinsics)]

extern crate pathtracer;
extern crate rayon;
use rand::random;
use std::fs::File;
use std::io::{BufWriter, Error, Write};

use rayon::prelude::*;

use pathtracer::{trace, Vec3d};

fn main() -> Result<(), Error> {
    let output = File::create("pixar.ppm")?;
    let mut file = BufWriter::new(output);
    let w = 960;
    let h = 540;
    let samples_count = 8;

    let position = Vec3d::new(-22.0, 5.0, 25.0);
    let goal = !(Vec3d::new(-3.0, 4.0, 0.0) - position);
    let left = !Vec3d::new(goal.get_z(), 0.0, -goal.get_x()) * ((1.0 / w as f32) as f32);

    // Cross-product to get the up vector
    let up = Vec3d::new(
        goal.get_y() * left.get_z() - goal.get_z() * left.get_y(),
        goal.get_z() * left.get_x() - goal.get_x() * left.get_z(),
        goal.get_x() * left.get_y() - goal.get_y() * left.get_x(),
    );

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
                            + left * (x - w as f32 / 2.0 + random::<f32>())
                            + up * (y - h as f32 / 2.0 + random::<f32>())),
                    );
            }
            // Reinhard tone mapping
            color = color * (1.0 / samples_count as f32) + 14.0 / 241.0;
            let o  = color + 1.0_f32;
            color = (color / o) * 255.0;
            //            color = Vec3d::new(color.get_x() / o.get_x(), color.get_y() / o.y, color.z / o.z) * 255.0;
            rgb[0] = color.get_x() as u8;
            rgb[1] = color.get_y() as u8;
            rgb[2] = color.get_z() as u8;
        });
    file.write_all(&pixels)?;
    file.flush()?;
    Ok(())
} // Andrew Kensler
