use crate::linalg::Three;
use crate::ray::{CanHit, Hit, Material, Ray};

pub struct Pyramid {
    // how high above the center of bottom the pyramid extends
    height: f64,
    up: Three<f64>,

    // aabb of the bottom of the pyramid
    bottom_min: Three<f64>,
    bottom_max: Three<f64>,
}

impl Pyramid {
    // TODO: new()
}

// a pyramid is just a bunch of squares that slowly increase in size

/*
5 faces, the point lies on one of them
- bottom aabb plane is easy to check for
- 4 triangle checks

ray = ray.origin + t * ray.direction
x on pyramid satisfies:
(x - pyramid.origin)
*/

impl CanHit for Pyramid {
    fn hit_by(&self, ray: Ray) -> Option<Hit> {
        let bottom_center = (self.bottom_max + self.bottom_min) * 0.5;
        // cone plus something else? https://lousodrome.net/blog/light/2017/01/03/intersection-of-a-ray-and-a-cone/
        // cube plus something else? https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
        // also need to determine the normal

        // 1. v = point - bottom_center
        // 2. height = v.dot(up)
        // 3. r = max_r * (1 - height / max_height)
        // 4. projected = point - height * up
        // 5. z = projected - bottom_center
        // 6. corner = bottom_max - bottom_center; assert |corner| = r
        // 7. cos_theta = (z dot corner) / |z| |corner| = (z dot corner) / r |z|
        // 8. |z| = r sin 45 / (cos_theta * cos(45) - √(1 - cos_theta * cos_theta) * sin(45))
        // if 8 is satisfied, it is on the pyramid

        /*
        |z| = r*s45 / (cos_theta * c45 - √(1 - cos_theta * cos_theta) * s45)
        |z| * (cos_theta * c45 - √(1 - cos_theta * cos_theta) * s45) = r * s45
        |z| * cos_theta * c45 - |z| * √(1 - cos_theta * cos_theta) * s45 = r * s45
        |z| * cos_theta - |z| * √(1 - cos_theta * cos_theta) = r
        zdc / r - √(|z|² * (1 - cos_theta * cos_theta))) = r
        zdc / r - √(|z|² - (|z| * cos_theta) ²) = r
        zdc / r - √(|z|² - (zdc / r)²) = r
        z·C - √(r²|z|² - (z·C)²) - r² = 0
        z·C - √(r²|z|² - (z·C)²) - r² = 0

        A² - B² = (A + B)(A - B)

        P = O-o
        r = max_r - max_r (P·up+tD·up) / max_height
        z = P + tD - ((P+tD)·up)up
        z = P + tD + (tD·up)up - (P·up)up
        z = P + tD + (tD·up - P·up)up
        z·C = P·C + tD·C - (tD·up - P·up)C·up
        Q1 = P·C
        Q2 = tD·C
        Q3 = (P·up)(C·up)
        Q4 = -t(C·up)(D·up)
        (z·C)²
        = (Q1 + (Q2 + (Q3 + Q4)))²
        = Q1² + 2Q1Q2 + 2Q1Q3 + 2Q1Q4 + (Q2 + (Q3 + Q4)))²
        = Q1² + 2Q1Q2 + 2Q1Q3 + 2Q1Q4 + Q2² + 2Q2Q3 + 2Q2Q4 + (Q3 + Q4))²
        = Q1² + Q2² + Q3² + Q4² + 2Q1Q2 + 2Q1Q3 + 2Q1Q4 + 2Q2Q3 + 2Q2Q4 + 2Q3Q4
        = Q1² + t²(D·C)² + Q3² + t²(C·up)²(D·up)² + t2Q1(D·C) + 2Q1Q3 - t2Q1(C·up)(D·up) + t2(D·C)Q3 - t²2(D·C)(C·up)(D·up) - t2Q3(C·up)(D·up)
        =   t²((D·C)² + (C·up)²(D·up)² - 2(D·C)(C·up)(D·up))
          + t (2Q1(D·C) - 2Q1(C·up)(D·up) + 2(D·C)Q3 - 2Q3(C·up)(D·up))
          +   ()
        */
        None
    }
}
