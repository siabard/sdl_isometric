pub fn rotate_deg((cx, cy): (f64, f64), (ox, oy): (f64, f64), deg: f64) -> (f64, f64) {
    let cx1 = cx - ox;
    let cy1 = cy - oy;

    // deg를 rad으로 변환
    let radian = deg * std::f64::consts::PI / 180.0;

    let cx2 = radian.cos() * cx1 - radian.sin() * cy1;
    let cy2 = radian.sin() * cx1 + radian.cos() * cy1;

    (cx2 + ox, cy2 + oy)
}

pub fn sat_detection(r1: &Vec<(f64, f64)>, r2: &Vec<(f64, f64)>) -> bool {
    for shape in 0..2 {
        let poly1 = if shape == 1 { r2 } else { r1 };
        let poly2 = if shape == 1 { r1 } else { r2 };

        for a in 0..poly1.len() {
            let b = (a + 1) % poly1.len();
            let axis: (f64, f64) = (-(poly1[b].1 - poly1[a].1), poly1[b].0 - poly1[a].0);
            let mut max_r1: f64 = f64::NEG_INFINITY;
            let mut min_r1: f64 = f64::INFINITY;

            for (_, p) in poly1.iter().enumerate() {
                let q = p.0 * axis.0 + p.1 * axis.1;

                min_r1 = min_r1.min(q);
                max_r1 = max_r1.max(q);
            }

            let mut max_r2: f64 = f64::NEG_INFINITY;
            let mut min_r2: f64 = f64::INFINITY;

            for (_, p) in poly2.iter().enumerate() {
                let q = p.0 * axis.0 + p.1 * axis.1;
                min_r2 = min_r2.min(q);
                max_r2 = max_r2.max(q);
            }

            if !(max_r2 >= min_r1 && max_r1 >= min_r2) {
                // 축 겹칩이 없으므로, 겹치지않았다.
                return false;
            }
        }
    }
    // 모든 축에 겹침이 있으므로 Intersect가 발생한다.
    true
}

pub fn distance(h: (f64, f64), t: (f64, f64)) -> f64 {
    ((h.0 - t.0).powi(2) + (h.1 - t.1).powi(2)).sqrt()
}

pub fn next_trailing(h: (f64, f64), t: (f64, f64), d: f64, h2: (f64, f64)) -> (f64, f64) {
    let m1 = (h.1 - t.1) / (h.0 - t.0);
    let m2 = (h2.1 - t.1) / (h2.0 - t.0);

    let ratio = (m2 - m1) / (1.0 + m1 * m2);
    let theta = ratio.atan();

    let sin_d = theta.sin() * d;
    let cos_d = theta.cos() * d;

    (h2.0 - cos_d, h2.1 - sin_d)
}
