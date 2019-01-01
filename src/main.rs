mod vector;

use crate::vector::Vec3d;
use std::ops::Sub;
use std::os::raw::c_int;

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

fn random_val() -> f64 {
    unsafe { rand() as f64 / 2_147_483_647_f64 }
}

extern "C" {
    fn rand() -> c_int;
}

// Rectangle CSG equation. Returns minimum signed distance from
// space carved by
// lowerLeft vertex and opposite rectangle vertex upperRight.
fn box_test(position: Vec3d, lowerLeft: Vec3d, upperRight: Vec3d) -> f64 {
    let lowerLeft = position + lowerLeft * -1.0;
    let upperRight = upperRight + position * -1.0;
    -min(
        min(
            min(lowerLeft.x, upperRight.x),
            min(lowerLeft.y, upperRight.y),
        ),
        min(lowerLeft.z, upperRight.z),
    )
}

#[derive(PartialEq)]
enum Hit {
    HIT_NONE = 0,
    HIT_LETTER = 1,
    HIT_WALL = 2,
    HIT_SUN = 3,
}

// Sample the world using Signed Distance Fields.
fn query_database(position: Vec3d) -> (f64, Hit) {
    let mut hitType: Hit;
    let mut distance = 1e9_f64;
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
            x: (letter[0] - 79) as f64,
            y: (letter[1] - 79) as f64,
            z: 0.0,
        } * 0.5;
        let e = Vec3d {
            x: (letter[2] - 79) as f64,
            y: (letter[3] - 79) as f64,
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

    for curve in curves.iter() {
        let mut o: Vec3d = f + *curve * -1.0;
        let cmp = if o.x > 0.0 {
            f64::abs(f64::sqrt(o % o) - 2.0)
        } else {
            o.y += o.y;
            if o.y > 0.0 {
                -2.0
            } else {
                2.0 + f64::sqrt(o % o) // This is wrong!
            }
        };

        distance = min(distance, cmp);
    }

    distance = f64::powf(f64::powf(distance, 8.0) + f64::powf(position.z, 8.0), 0.125) - 0.5;
    hitType = Hit::HIT_LETTER;

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
        hitType = Hit::HIT_WALL;
    }

    let sun: f64 = 19.9 - position.y; // Everything above 19.9 is light source.
    if sun < distance {
        distance = sun;
        hitType = Hit::HIT_SUN;
    }

    (distance, hitType)
}

// Perform signed sphere marching
// Returns hitType 0, 1, 2, or 3 and update hit position/normal
fn ray_marching(
    origin: Vec3d,
    direction: Vec3d,
    mut hitPos: Vec3d,
    mut hitNorm: Vec3d,
) -> (Hit, Vec3d) {
    let mut noHitCount: i32 = 0;
    // Signed distance marching

    let mut total_d = 0.0;
    while total_d < 100.0 {
        hitPos = origin + direction * total_d;
        let (d, hitType) = query_database(hitPos);

        // TODO: NOT SURE ABOUT NOHITCOUNT BEEING PASSED TO QUERY DATABASE
        noHitCount += 1;
        if d < 0.01 || noHitCount > 99 {
            hitNorm = !Vec3d {
                x: query_database(
                    hitPos
                        + Vec3d {
                            x: 0.01,
                            y: 0.0,
                            z: 0.0,
                        },
                )
                .0 - d,
                y: query_database(
                    hitPos
                        + Vec3d {
                            x: 0.0,
                            y: 0.01,
                            z: 0.0,
                        },
                )
                .0 - d,
                z: query_database(
                    hitPos
                        + Vec3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.01,
                        },
                )
                .0 - d,
            };

            return (hitType, hitNorm);
        }

        total_d += d;
    }
    (Hit::HIT_NONE, hitNorm)
}

fn trace(mut origin: Vec3d, mut direction: Vec3d) -> Vec3d {
    let sampledPosition: Vec3d = Default::default();
    let normal: Vec3d = Default::default();
    let mut color: Vec3d = Default::default();
    let mut attenuation = Vec3d::new(1.0);
    let lightDirection = Vec3d {
        ..!Vec3d {
            x: 0.6,
            y: 0.6,
            z: 1.0,
        }
    }; // Directional light

    for _ in (0..4).rev() {
        // Number of bounces
        let (hitType, normal) = ray_marching(origin, direction, sampledPosition, normal);
        match hitType {
            Hit::HIT_NONE => break,
            Hit::HIT_LETTER => {
                // Specular bounce on a letter. No color acc.
                direction = direction + normal * (normal % direction * -2.0);
                origin = sampledPosition + direction * 0.1;
                attenuation = attenuation * 0.2; // Attenuation via distance traveled.
            }
            Hit::HIT_WALL => {
                // Wall hit uses color yellow?
                let incidence = normal % lightDirection;
                let p = 6.283_185 * random_val();
                let c = random_val();
                let s = f64::sqrt(1.0 - c);
                let g = if normal.z < 0.0 { -1.0 } else { 1.0 };
                let u = -1.0 / (g + normal.z);
                let v = normal.x * normal.y * u;
                direction = Vec3d {
                    x: v,
                    y: g + normal.y * normal.y * u,
                    z: -normal.y,
                } * (f64::cos(p) * s)
                    + Vec3d {
                        x: 1.0 + g * normal.x * normal.x * u,
                        y: g * v,
                        z: -g * normal.x,
                    } * (f64::sin(p) * s)
                    + normal * c.sqrt();
                origin = sampledPosition + direction * 0.1;
                attenuation = attenuation * 0.2;
                if incidence > 0.0
                    && ray_marching(
                        sampledPosition + normal * 0.1,
                        lightDirection,
                        sampledPosition,
                        normal,
                    )
                    .0 == Hit::HIT_SUN
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
            Hit::HIT_SUN => {
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

fn main() {
    //  int w = 960, h = 540, samplesCount = 16;
    let w = 960.0;
    let h = 540.0;
    let samplesCount = 32.0;
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
    } * ((1.0 / w) as f64);

    // Cross-product to get the up vector
    let up = Vec3d {
        x: goal.y * left.z - goal.z * left.y,
        y: goal.z * left.x - goal.x * left.z,
        z: goal.x * left.y - goal.y * left.x,
    };

    println!("P6 {} {} 255 ", w, h);
    for y in (0..h as i32).rev() {
        for x in (0..w as i32).rev() {
            let mut color = Vec3d::default();
            for _p in 0..samplesCount as i32 {
                color = color
                    + trace(
                        position,
                        !(goal
                            + left * (x as f64 - w / 2.0 + random_val())
                            + up * (y as f64 - h / 2.0 + random_val())),
                    );

                // Reinhard tone mapping
                color = color * (1.0 / samplesCount) + 14.0 / 241.0;
                let o = color + 1.0;
                color = Vec3d {
                    x: color.x / o.x,
                    y: color.y / o.y,
                    z: color.z / o.z,
                } * 255.0;
                print!("{}{}{}", color.x as u8 as char, color.y as u8 as char, color.z as u8 as char);
            }
        }
    }
} // Andrew Kensler
