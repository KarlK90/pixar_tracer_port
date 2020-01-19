use super::vector::Vec3d;
use rand::random;
use std::intrinsics::*;

// Rectangle CSG equation. Returns minimum signed distance from
// space carved by
// lower_left vertex and opposite rectangle vertex upper_right.
pub fn box_test(position: Vec3d, lower_left: Vec3d, upper_right: Vec3d) -> f32 {
    let lower_left = position - lower_left;
    let upper_right = upper_right - position;
    -(lower_left.min(upper_right).min_element())
}

#[derive(PartialEq, Debug)]
pub enum Hit {
    None,
    Letter,
    Wall,
    Sun,
}

lazy_static! {
    static ref LETTERS : Vec<(Vec3d, Vec3d, f32)> =
    {
        [
            // 15 two points lines
            "5O5_", "5W9W", "5_9_", // P (without curve)
            "AOEO", "COC_", "A_E_", // I
            "IOQ_", "I_QO", // X
            "UOY_", "Y_]O", "WW[W", // A
            "aOa_", "aWeW", "a_e_", "cWiO",
        ]
        .concat()
        .as_bytes()
        .chunks(4)
        .map(|s| {
            (
                s[0] as i32 - 79,
                s[1] as i32 - 79,
                s[2] as i32 - 79,
                s[3] as i32 - 79,
            )
        }).map(|(a,b,c,d)|{
            let begin = Vec3d::new(a as f32, b as f32, 0.0) * 0.5;
            let e = Vec3d::new(c as f32, d as f32, 0.0) * 0.5 - begin;
            (begin, e, e % e)
        }).collect()
    };
    static ref CURVES : [Vec3d;2] = [
        Vec3d::new (
            -11.0,
            6.0,
            0.0,
        ),
        Vec3d::new (
             11.0,
             6.0,
             0.0,
  ),
    ];
}

// Sample the world using Signed Distance Fields.
pub fn query_database(position: Vec3d) -> (f32, Hit) {
    let mut hit_type: Hit;
    let mut distance = 1e9_f32;
    let f = Vec3d::new(position.get_x(), position.get_y(), 0.0); // Flattened position (z=0)

    for (begin, e, e_mod_e) in LETTERS.iter() {
        let o_1 = -min((*begin - f) % *e / (e_mod_e), 0.0);
        let o = f - (*begin + *e * min(o_1, 1.0));
        distance = min(distance, o % o); // compare squared distance.
    }
    distance = unsafe { sqrtf32(distance) }; // Get real distance, not square distance.

    // Two curves (for P and R in PixaR) with hard-coded locations.
    for curve in CURVES.iter().rev() {
        unsafe {
            let mut o = f - *curve;
            let cmp = if o.get_x() > 0.0 {
                ((o % o).sqrt() - 2.0).abs()
            } else {
                if o.get_y() > 0.0 {
                    o.set_y(o.get_y() + -2.0)
                } else {
                    o.set_y(o.get_y() + 2.0)
                }
                sqrtf32(o % o)
            };
            distance = min(distance, cmp);
        }
    }
    unsafe {
        distance = fsub_fast(
            (fadd_fast(distance.powi(8), position.get_z().powi(8))).powf(0.125),
            0.5,
        );
    }
    hit_type = Hit::Letter;

    let room_dist = min(
        // min(A,B) = Union with Constructive solid geometry
        //-min carves an empty space
        -min(
            // Lower room
            box_test(
                position,
                Vec3d::new(-30.0, -0.5, -30.0),
                Vec3d::new(30.0, 18.0, 30.0),
            ),
            // Upper room
            box_test(
                position,
                Vec3d::new(-25.0, 17.0, -25.0),
                Vec3d::new(25.0, 20.0, 25.0),
            ),
        ),
        box_test(
            // Ceiling "planks" spaced 8 units apart.
            Vec3d::new(
                position.get_x().abs() % 8.0,
                position.get_y(),
                position.get_z(),
            ),
            Vec3d::new(1.5, 18.5, -25.0),
            Vec3d::new(6.5, 20.0, 25.0),
        ),
    );

    if room_dist < distance {
        distance = room_dist;
        hit_type = Hit::Wall;
    }

    let sun = 19.9 - position.get_y(); // Everything above 19.9 is light source.
    if sun < distance {
        distance = sun;
        hit_type = Hit::Sun;
    }

    (distance, hit_type)
}

// Perform signed sphere marching
// Returns hitType 0, 1, 2, or 3 and update hit position/normal
pub fn ray_marching(origin: Vec3d, direction: Vec3d, mut hit_norm: Vec3d) -> (Hit, Vec3d, Vec3d) {
    let mut hit_pos: Vec3d = Default::default();
    let mut no_hit_count = 0;
    // Signed distance marching

    let mut total_d = 0.0;
    while total_d < 100.0 {
        hit_pos = origin + direction * total_d;
        let (d, hit_type) = query_database(hit_pos);

        no_hit_count += 1;
        if d < 0.01 || no_hit_count > 99 {
            hit_norm = !Vec3d::new(
                query_database(hit_pos + Vec3d::new(0.01, 0.0, 0.0)).0 - d,
                query_database(hit_pos + Vec3d::new(0.0, 0.01, 0.0)).0 - d,
                query_database(hit_pos + Vec3d::new(0.0, 0.0, 0.01)).0 - d,
            );

            return (hit_type, hit_norm, hit_pos);
        }

        total_d += d;
    }
    (Hit::None, hit_norm, hit_pos)
}

pub fn trace(mut origin: Vec3d, mut direction: Vec3d) -> Vec3d {
    let normal: Vec3d = Default::default();
    let mut color: Vec3d = Default::default();
    let mut attenuation = Vec3d::new(1.0, 1.0, 1.0);
    let light_direction = !Vec3d::new(0.6, 0.6, 1.0); // Directional light

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
                let p = 6.283_185 * random::<f32>();
                let c = random::<f32>();
                let s = f32::sqrt(1.0 - c);
                let g = if normal.get_z() < 0.0 { -1.0 } else { 1.0 };
                let u = -1.0 / (g + normal.get_z());
                let v = normal.get_x() * normal.get_y() * u;
                direction = Vec3d::new(v, g + normal.get_y() * normal.get_y() * u, -normal.get_y())
                    * (f32::cos(p) * s)
                    + Vec3d::new(
                        1.0 + g * normal.get_x() * normal.get_x() * u,
                        g * v,
                        -g * normal.get_x(),
                    ) * (f32::sin(p) * s)
                    + normal * c.sqrt();
                origin = sampled_position + direction * 0.1;
                attenuation = attenuation * 0.2;
                if incidence > 0.0
                    && ray_marching(sampled_position + normal * 0.1, light_direction, normal).0
                        == Hit::Sun
                {
                    color = color + attenuation * Vec3d::new(500.0, 400.0, 100.0) * incidence;
                }
            }
            Hit::Sun => {
                color = color + attenuation * Vec3d::new(50.0, 80.0, 100.0);
                break; // Sun Color
            }
        }
    }
    color
}

#[inline]
fn min<T: PartialOrd>(l: T, r: T) -> T {
    if l < r {
        l
    } else {
        r
    }
}
