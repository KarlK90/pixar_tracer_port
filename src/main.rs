use std::os::raw::c_int;

mod vector;

fn min<T: PartialOrd>(l: T, r: T) -> T {
    if l < r {
        l
    } else {
        r
    }
}

fn random_val() -> f64 {
    unsafe { rand() as f64 / 2_147_483_647_f64 }
}

extern "C" {
    fn rand() -> c_int;
}
// // Rectangle CSG equation. Returns minimum signed distance from
// // space carved by
// // lowerLeft vertex and opposite rectangle vertex upperRight.
// BoxTest(Vec position, Vec lowerLeft, Vec upperRight) -> f64 {
//   lowerLeft = position + lowerLeft * -1;
//   upperRight = upperRight + position * -1;
//   return -min(
//           min(
//                   min(lowerLeft.x, upperRight.x),
//                   min(lowerLeft.y, upperRight.y)),
//           min(lowerLeft.z, upperRight.z));
// }

enum Hit {
    HIT_NONE = 0,
    HIT_LETTER = 1,
    HIT_WALL = 2,
    HIT_SUN = 3,
}

// Sample the world using Signed Distance Fields.
fn query_database(position : Vec3d, hitType : Hit) -> f64 {
  let mut distance : f64 = 1e9;
  let mut f : Vec3d = position.clone(); // Flattened position (z=0)
  f.z = 0;
  let letters : [&str; 15*4+1] =               // 15 two points lines
          "5O5_" "5W9W" "5_9_"         // P (without curve)
          "AOEO" "COC_" "A_E_"         // I
          "IOQ_" "I_QO"                // X
          "UOY_" "Y_]O" "WW[W"         // A
          "aOa_" "aWeW" "a_e_" "cWiO"; // R (without curve)

for i in (0..letters.len()).step_by(4)
  for (int i = 0; i < sizeof(letters); i += 4) {
    let begin = Vec::new(letters[i] - 79, letters[i + 1] - 79) * .5;
    let e = Vec::new(letters[i + 2] - 79, letters[i + 3] - 79) * .5 + begin * -1;
    let o = f + (begin + e * min(-min((begin + f * -1) % e / (e % e),
                                      0),
                                 1)
                ) * -1;
    distance = min(distance, o % o); // compare squared distance.
  }
  distance = sqrtf(distance); // Get real distance, not square distance.

  // Two curves (for P and R in PixaR) with hard-coded locations.
  Vec curves[] = {Vec(-11, 6), Vec(11, 6)};
  for (int i = 2; i--;) {
    Vec o = f + curves[i] * -1;
    distance = min(distance,
                   o.x > 0 ? fabsf(sqrtf(o % o) - 2)
                           : (o.y += o.y > 0 ? -2 : 2, sqrtf(o % o))
               );
  }
  distance = powf(powf(distance, 8) + powf(position.z, 8), .125) - .5;
  hitType = HIT_LETTER;

  float roomDist ;
  roomDist = min(// min(A,B) = Union with Constructive solid geometry
               //-min carves an empty space
                -min(// Lower room
                     BoxTest(position, Vec(-30, -.5, -30), Vec(30, 18, 30)),
                     // Upper room
                     BoxTest(position, Vec(-25, 17, -25), Vec(25, 20, 25))
                ),
                BoxTest( // Ceiling "planks" spaced 8 units apart.
                  Vec(fmodf(fabsf(position.x), 8),
                      position.y,
                      position.z),
                  Vec(1.5, 18.5, -25),
                  Vec(6.5, 20, 25)
                )
  );
  if (roomDist < distance) distance = roomDist, hitType = HIT_WALL;

  sun : f64 = 19.9 - position.y ; // Everything above 19.9 is light source.
  if (sun < distance){
      distance = sun, hitType = HIT_SUN;
      }

  distance;
}

// // Perform signed sphere marching
// // Returns hitType 0, 1, 2, or 3 and update hit position/normal
// RayMarching(Vec origin, Vec direction, Vec &hitPos, Vec &hitNorm) -> i32 {
//   int hitType = HIT_NONE;
//   int noHitCount = 0;
//   float d; // distance from closest object in world.

//   // Signed distance marching
//   for (float total_d=0; total_d < 100; total_d += d)
//     if ((d = QueryDatabase(hitPos = origin + direction * total_d, hitType)) < .01
//             || ++noHitCount > 99)
//       return hitNorm =
//          !Vec(QueryDatabase(hitPos + Vec(.01, 0), noHitCount) - d,
//               QueryDatabase(hitPos + Vec(0, .01), noHitCount) - d,
//               QueryDatabase(hitPos + Vec(0, 0, .01), noHitCount) - d)
//          , hitType; // Weird return statement where a variable is also updated.
//   return 0;
// }

// fn Trace(Vec origin, Vec direction) -> Vec {
//   Vec sampledPosition, normal, color, attenuation = 1;
//   Vec lightDirection(!Vec(.6, .6, 1)); // Directional light

//   for (int bounceCount = 3; bounceCount--;) {
//     int hitType = RayMarching(origin, direction, sampledPosition, normal);
//     if (hitType == HIT_NONE) break; // No hit. This is over, return color.
//     if (hitType == HIT_LETTER) { // Specular bounce on a letter. No color acc.
//       direction = direction + normal * ( normal % direction * -2);
//       origin = sampledPosition + direction * 0.1;
//       attenuation = attenuation * 0.2; // Attenuation via distance traveled.
//     }
//     if (hitType == HIT_WALL) { // Wall hit uses color yellow?
//       float incidence = normal % lightDirection;
//       float p = 6.283185 * random_val();
//       float c = random_val();
//       float s = sqrtf(1 - c);
//       float g = normal.z < 0 ? -1 : 1;
//       float u = -1 / (g + normal.z);
//       float v = normal.x * normal.y * u;
//       direction = Vec(v,
//                       g + normal.y * normal.y * u,
//                       -normal.y) * (cosf(p) * s)
//                   +
//                   Vec(1 + g * normal.x * normal.x * u,
//                       g * v,
//                       -g * normal.x) * (sinf(p) * s) + normal * sqrtf(c);
//       origin = sampledPosition + direction * .1;
//       attenuation = attenuation * 0.2;
//       if (incidence > 0 &&
//           RayMarching(sampledPosition + normal * .1,
//                       lightDirection,
//                       sampledPosition,
//                       normal) == HIT_SUN)
//         color = color + attenuation * Vec(500, 400, 100) * incidence;
//     }
//     if (hitType == HIT_SUN) { //
//       color = color + attenuation * Vec(50, 80, 100); break; // Sun Color
//     }
//   }
//   return color;
// }

fn main() {
    // //  int w = 960, h = 540, samplesCount = 16;
    //   int w = 960, h = 540, samplesCount = 32;
    //   Vec position(-22, 5, 25);
    //   Vec goal = !(Vec(-3, 4, 0) + position * -1);
    //   Vec left = !Vec(goal.z, 0, -goal.x) * (1. / w);

    //   // Cross-product to get the up vector
    //   Vec up(goal.y * left.z - goal.z * left.y,
    //       goal.z * left.x - goal.x * left.z,
    //       goal.x * left.y - goal.y * left.x);

    //   printf("P6 %d %d 255 ", w, h);
    //   for (int y = h; y--;)
    //     for (int x = w; x--;) {
    //       Vec color;
    //       for (int p = samplesCount; p--;)
    //         color = color + Trace(position, !(goal + left * (x - w / 2 + random_val()) + up * (y - h / 2 + random_val())));

    //       // Reinhard tone mapping
    //       color = color * (1. / samplesCount) + 14. / 241;
    //       Vec o = color + 1;
    //       color = Vec(color.x / o.x, color.y / o.y, color.z / o.z) * 255;
    //       printf("%c%c%c", (int) color.x, (int) color.y, (int) color.z);
    //     }
} // Andrew Kensler
