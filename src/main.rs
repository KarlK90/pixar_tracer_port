extern crate rand;
use crate::vector::Vec3d;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Sub;

mod vector;

fn min<T: PartialOrd>(l: T, r: T) -> T {
    if l < r {
        l
    } else {
        r
    }
}

fn mod_<T: PartialOrd + Sub<Output = T> + Copy>(a: T, b: T) -> T {
    if a >= b {
        mod_(a - b, b)
    } else {
        a
    }
}

fn random_val() -> f32 {
    rand::random()
}

// Rectangle CSG equation. Returns minimum signed distance from
// space carved by
// lower_left vertex and opposite rectangle vertex upper_right.
fn box_test(position: Vec3d, lower_left: Vec3d, upper_right: Vec3d) -> f32 {
    let lower_left = position + lower_left * -1.0;
    let upper_right = upper_right + position * -1.0;
    -min(
        min(
            min(lower_left.x, upper_right.x),
            min(lower_left.y, upper_right.y),
        ),
        min(lower_left.z, upper_right.z),
    )
}

#[derive(PartialEq, Debug)]
enum Hit {
    None = 0,
    Letter = 1,
    Wall = 2,
    Sun = 3,
}

// Sample the world using Signed Distance Fields.
fn query_database(position: Vec3d) -> (f32, Hit) {
    let mut hit_type: Hit;
    let mut distance = 1e9_f32;
    let f = Vec3d { z: 0.0, ..position }; // Flattened position (z=0)
    let letters = [
        // 15 two points lines
        "5O5_", "5W9W", "5_9_", // P (without curve)
        "AOEO", "COC_", "A_E_", // I
        "IOQ_", "I_QO", // X
        "UOY_", "Y_]O", "WW[W", // A
        "aOa_", "aWeW", "a_e_", "cWiO",
    ]; // R (without curve)

    for letter in letters.iter() {
        let letter = &letter[0..4].as_bytes();
        let begin = Vec3d {
            x: (letter[0] as i32 - 79) as f32,
            y: (letter[1] as i32 - 79) as f32,
            z: 0.0,
        } * 0.5;
        let e = Vec3d {
            x: (letter[2] as i32 - 79) as f32,
            y: (letter[3] as i32 - 79) as f32,
            z: 0.0,
        } * 0.5
            + begin * -1.0;
        let o = f + (begin + e * min(-min((begin + f * -1.0) % e / (e % e), 0.0), 1.0)) * -1.0;
        distance = min(distance, o % o); // compare squared distance.
    }
    distance = distance.sqrt(); // Get real distance, not square distance.

    // Two curves (for P and R in PixaR) with hard-coded locations.
    let curves = [
        Vec3d {
            x: -11.0,
            y: 6.0,
            z: 0.0,
        },
        Vec3d {
            x: 11.0,
            y: 6.0,
            z: 0.0,
        },
    ];

    for curve in curves.iter().rev() {
        let mut o: Vec3d = f + *curve * -1.0;
        let cmp = if o.x > 0.0 {
            ((o % o).sqrt() - 2.0).abs()
        } else {
            if o.y > 0.0 {
                o.y += -2.0
            } else {
                o.y += 2.0
            }
            (o % o).sqrt()
        };

        distance = min(distance, cmp);
    }

    distance = (distance.powf(8.0) + position.z.powf(8.0)).powf(0.125) - 0.5;
    hit_type = Hit::Letter;

    let room_dist = min(
        // min(A,B) = Union with Constructive solid geometry
        //-min carves an empty space
        -min(
            // Lower room
            box_test(
                position,
                Vec3d {
                    x: -30.0,
                    y: -0.5,
                    z: -30.0,
                },
                Vec3d {
                    x: 30.0,
                    y: 18.0,
                    z: 30.0,
                },
            ),
            // Upper room
            box_test(
                position,
                Vec3d {
                    x: -25.0,
                    y: 17.0,
                    z: -25.0,
                },
                Vec3d {
                    x: 25.0,
                    y: 20.0,
                    z: 25.0,
                },
            ),
        ),
        box_test(
            // Ceiling "planks" spaced 8 units apart.
            Vec3d {
                x: mod_(position.x.abs(), 8.0),
                ..position
            },
            Vec3d {
                x: 1.5,
                y: 18.5,
                z: -25.0,
            },
            Vec3d {
                x: 6.5,
                y: 20.0,
                z: 25.0,
            },
        ),
    );

    if room_dist < distance {
        distance = room_dist;
        hit_type = Hit::Wall;
    }

    let sun = 19.9 - position.y; // Everything above 19.9 is light source.
    if sun < distance {
        distance = sun;
        hit_type = Hit::Sun;
    }

    (distance, hit_type)
}

// Perform signed sphere marching
// Returns hitType 0, 1, 2, or 3 and update hit position/normal
fn ray_marching(origin: Vec3d, direction: Vec3d, mut hit_norm: Vec3d) -> (Hit, Vec3d, Vec3d) {
    let mut hit_pos: Vec3d = Default::default();
    let mut no_hit_count = 0;
    // Signed distance marching

    let mut total_d = 0.0;
    while total_d < 100.0 {
        hit_pos = origin + direction * total_d;
        let (d, hit_type) = query_database(hit_pos);

        no_hit_count += 1;
        if d < 0.01 || no_hit_count > 99 {
            hit_norm = !Vec3d {
                x: query_database(
                    hit_pos
                        + Vec3d {
                            x: 0.01,
                            y: 0.0,
                            z: 0.0,
                        },
                )
                .0 - d,
                y: query_database(
                    hit_pos
                        + Vec3d {
                            x: 0.0,
                            y: 0.01,
                            z: 0.0,
                        },
                )
                .0 - d,
                z: query_database(
                    hit_pos
                        + Vec3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.01,
                        },
                )
                .0 - d,
            };

            return (hit_type, hit_norm, hit_pos);
        }

        total_d += d;
    }
    (Hit::None, hit_norm, hit_pos)
}

fn trace(mut origin: Vec3d, mut direction: Vec3d) -> Vec3d {
    let normal: Vec3d = Default::default();
    let mut color: Vec3d = Default::default();
    let mut attenuation = Vec3d::new(1.0);
    let light_direction = !Vec3d {
        x: 0.6,
        y: 0.6,
        z: 1.0,
    }; // Directional light

    for _ in 0..3 {
        // Number of bounces
        let (hit_type, normal, sampled_position) = ray_marching(origin, direction, normal);
        match hit_type {
            Hit::None => break,
            Hit::Letter => {
                // Specular bounce on a letter. No color acc.
                direction = direction + normal * (normal % direction * -2.0);
                origin = sampled_position + direction * 0.1;
                attenuation = attenuation * 0.2; // Attenuation via distance traveled.
            }
            Hit::Wall => {
                // Wall hit uses color yellow?
                let incidence = normal % light_direction;
                let p = 6.283_185 * random_val();
                let c = random_val();
                let s = f32::sqrt(1.0 - c);
                let g = if normal.z < 0.0 { -1.0 } else { 1.0 };
                let u = -1.0 / (g + normal.z);
                let v = normal.x * normal.y * u;
                direction = Vec3d {
                    x: v,
                    y: g + normal.y * normal.y * u,
                    z: -normal.y,
                } * (f32::cos(p) * s)
                    + Vec3d {
                        x: 1.0 + g * normal.x * normal.x * u,
                        y: g * v,
                        z: -g * normal.x,
                    } * (f32::sin(p) * s)
                    + normal * c.sqrt();
                origin = sampled_position + direction * 0.1;
                attenuation = attenuation * 0.2;
                if incidence > 0.0
                    && ray_marching(sampled_position + normal * 0.1, light_direction, normal).0
                        == Hit::Sun
                {
                    color = color
                        + attenuation
                            * Vec3d {
                                x: 500.0,
                                y: 400.0,
                                z: 100.0,
                            }
                            * incidence;
                }
            }
            Hit::Sun => {
                color = color
                    + attenuation
                        * Vec3d {
                            x: 50.0,
                            y: 80.0,
                            z: 100.0,
                        };
                break; // Sun Color
            }
        }
    }
    color
}

fn main() -> Result<(), std::io::Error> {
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
    } + position * -1.0);
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
    for y in (0..h as i32).rev() {
        for x in (0..w as i32).rev() {
            let mut color = Vec3d::default();
            for _ in (0..samples_count as i32).rev() {
                color = color
                    + trace(
                        position,
                        !(goal
                            + left * (x as f32 - w / 2.0 + random_val())
                            + up * (y as f32 - h / 2.0 + random_val())),
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

            file.write_all(&[color.x as u8, color.y as u8, color.z as u8])?;
        }
        file.flush()?;
    }
    Ok(())
} // Andrew Kensler
