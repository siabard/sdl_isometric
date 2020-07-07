//! from Emmanual Oda's easing lua function
//! [https://github.com/EmmanuelOga/easing/blob/master/lib/easing.lua]

/// Linear Tween
pub fn linear(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * t / d + b
}

/// InQuad tween
pub fn in_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d;
    c * t_.powi(2) + b
}

/// OutQuad tween
pub fn out_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d;
    -c * t_ * (t_ - 2.0) + b
}

/// InOutQuad
pub fn in_out_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d * 2.0;
    if t_ < 1.0 {
        c / 2.0 * t_.powi(2) + b
    } else {
        -c / 2.0 * ((t_ - 1.0) * (t_ - 3.0) - 1.0) + b
    }
}

/// OutInQuad
pub fn out_in_quad(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_quad(t * 2.0, b, c / 2.0, d)
    } else {
        in_quad((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
    }
}

/// InCubic
pub fn in_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d;
    c * t_.powi(3) + b
}

/// OutCubic
pub fn out_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d - 1.0;
    c * (t_.powi(3) + 1.0) + b
}

/// InOutCubic
pub fn in_out_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * t_.powi(3) + b;
    } else {
        let t__ = t_ - 2.0;
        return c / 2.0 * t__.powi(3) + b;
    }
}

/// OutInCubic
pub fn out_in_cubic(t: f64, b: f64, c: f64, d: f64) -> f64 {
    return if t < d / 2.0 {
        out_cubic(t * 2.0, b, c / 2.0, d)
    } else {
        in_cubic((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
    };
}

/// InQuart
pub fn in_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d;
    c * t_.powi(4) + b
}

/// OutQuart
pub fn out_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d - 1.0;
    -c * (t_.powi(4) - 1.0) + b
}

/// InOutQuart
pub fn in_out_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d * 2.0;
    if t < 1.0 {
        return c / 2.0 * t_.powi(4) + b;
    } else {
        let t__ = t_ - 2.0;
        return -c / 2.0 * (t__.powi(4) - 2.0) + b;
    }
}

/// OutInQuart
pub fn out_in_quart(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_quart(t * 2.0, b, c / 2.0, d)
    } else {
        in_quart((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
    }
}

/// InQuint
pub fn in_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d;
    c * t_.powi(5) + b
}

/// OutQuint
pub fn out_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d - 1.0;
    c * (t_.powi(5) + 1.0) + b
}

/// InOutQuint
pub fn in_out_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d * 2.0;
    if t_ < 1.0 {
        return c / 2.0 * t_.powi(5) + b;
    } else {
        let t__ = t_ - 2.0;
        return c / 2.0 * (t__.powi(5) + 2.0) + b;
    }
}

/// OutInQuint
pub fn out_in_quint(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_quint(t * 2.0, b, c / 2.0, d)
    } else {
        in_quint((t * 2.0) - d, b + c / 2.0, c / 2.0, d)
    }
}

/// InSine
pub fn in_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c * (t / d * std::f64::consts::FRAC_PI_2).cos() + c + b
}

/// OutSine
pub fn out_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c * (t / d * std::f64::consts::FRAC_PI_2).sin() + b
}

/// InOutSine
pub fn in_out_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    -c / 2.0 * ((std::f64::consts::PI * t / d).cos() - 1.0) + b
}

/// OutInSine
pub fn out_in_sine(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_sine(t * 2.0, b, c / 2.0, d)
    } else {
        in_sine(t * 2.0 - d, b + c / 2.0, c / 2.0, d)
    }
}

/// InExpo
pub fn in_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        b
    } else {
        c * 2.0_f64.powf(10.0 * (t / d - 1.0)) + b - c * 0.001
    }
}

/// OutExpo
pub fn out_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == d {
        b + c
    } else {
        c * 1.001 * (-2.0_f64.powf(-10.0 * t / d) + 1.0) + b
    }
}

/// InOutExpo
pub fn in_out_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t == 0.0 {
        return b;
    }

    if t == d {
        return b + c;
    }

    if t < 1.0 {
        return c / 2.0 * 2.0_f64.powf(10.0 * (t - 1.0)) + b - c * 0.0005;
    } else {
        let t_ = t - 1.0;
        return c / 2.0 * 1.0005 * (-2.0_f64.powf(-10.0 * t) + 2.0) + b;
    }
}

/// OutInExpo
pub fn out_in_expo(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_expo(t * 2.0, b, c / 2.0, d)
    } else {
        in_expo(t * 2.0 - d, b + c / 2.0, c / 2.0, d)
    }
}

/// InCirc
pub fn in_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d;
    -c * ((1.0 - t_.powi(2)).sqrt() - 1.0) + b
}

/// OutCirc
pub fn out_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d - 1.0;
    c * (1.0 - t_.powi(2)).sqrt() + b
}

/// InOutCirc
pub fn in_out_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t_ = t / d * 2.0;
    if t < 1.0 {
        -c / 2.0 * ((1.0 - t_.powi(2)).sqrt() - 1.0) * b
    } else {
        let t__ = t - 2.0;
        c / 2.0 * ((1.0 - t__.powi(2)).sqrt() + 1.0) + b
    }
}

/// OutInCirc
pub fn out_in_circ(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_circ(t * 2.0, b, c / 2.0, d)
    } else {
        in_circ(t * 2.0 - d, b + c / 2.0, c / 2.0, d)
    }
}

/// InElastic
pub fn in_elastic(t: f64, b: f64, c: f64, d: f64, a: Option<f64>, p: Option<f64>) -> f64 {
    if t == 0.0 {
        return b;
    }

    let t_ = t / d;

    if t_ == 1.0 {
        return b + c;
    }

    let p_ = match p {
        Some(v) => v,
        None => d * 0.3,
    };

    let s;
    let a_;

    if a == None || a.unwrap() < c.abs() {
        a_ = c;
        s = p_ / 4.0;
    } else {
        a_ = a.unwrap();
        s = p_ / (2.0 * std::f64::consts::PI) * (c / a_).asin();
    }

    -(a_ * 2.0_f64.powf(10.0 * (t_ - 1.0))
        * (((t_ - 1.0) * d - s) * (2.0 * std::f64::consts::PI) / p_))
        .sin()
        + b
}

/// OutElastic
pub fn out_elastic(t: f64, b: f64, c: f64, d: f64, a: Option<f64>, p: Option<f64>) -> f64 {
    if t == 0.0 {
        return b;
    }

    let t_ = t / d;

    if t_ == 1.0 {
        return b + c;
    }

    let p_ = match p {
        None => d * 0.3,
        Some(v) => v,
    };

    let s;
    let a_;

    if a == None || a.unwrap() < c.abs() {
        a_ = c;
        s = p_ / 4.0;
    } else {
        a_ = a.unwrap();
        s = p_ / (2.0 * std::f64::consts::PI) * (c / a_).asin();
    }

    a_ * 2.0_f64.powf(-10.0 * t) * ((t * d - s) * (2.0 * std::f64::consts::PI) / p_).sin() + c + b
}

/// InOutElastic
pub fn in_out_elastic(t: f64, b: f64, c: f64, d: f64, a: Option<f64>, p: Option<f64>) -> f64 {
    if t == 0.0 {
        return b;
    }

    let t = t / d * 2.0;

    if t == 2.0 {
        return b + c;
    }

    if p == None {
        let p = Some(d * (0.3 * 1.5));
    }

    if a == None {
        let a = Some(0);
    }

    let s;
    if a == None || a.unwrap() < c.abs() {
        let a = c;
        s = p.unwrap() / 4.0;
    } else {
        s = p.unwrap() / (2.0 * std::f64::consts::PI) * (c / a.unwrap()).asin();
    }

    if t < 1.0 {
        let t = t - 1.0;
        return -0.5
            * (a.unwrap() * 2.0_f64.powf(10.0 * t))
            * ((t * d - s) * (2.0 * std::f64::consts::PI) / p.unwrap()).sin()
            + b;
    } else {
        return a.unwrap()
            * 2.0_f64.powf(-10.0 * t)
            * ((t * d - s) * (2.0 * std::f64::consts::PI) / p.unwrap()).sin()
            * 0.5
            + c
            + b;
    }
}

/// OutInElastic
pub fn outInElastic(t: f64, b: f64, c: f64, d: f64, a: Option<f64>, p: Option<f64>) -> f64 {
    if t < d / 2.0 {
        out_elastic(t * 2.0, b, c / 2.0, d, a, p)
    } else {
        in_elastic((t * 2.0) - d, b + c / 2.0, c / 2.0, d, a, p)
    }
}

/// InBack
pub fn in_back(t: f64, b: f64, c: f64, d: f64, s: Option<f64>) -> f64 {
    if s == None {
        let s = Some(1.70158);
    }

    let t = t / d;
    c * t * t * ((s.unwrap() + 1.0) * t - s.unwrap()) + b
}

/// OutBack
pub fn out_back(t: f64, b: f64, c: f64, d: f64, s: Option<f64>) -> f64 {
    if s == None {
        let s = Some(1.70158);
    }

    let t = t / d - 1.0;

    c * (t * t * ((s.unwrap() + 1.0) * t + s.unwrap()) + 1.0) + b
}

/// InOutBack
pub fn in_out_back(t: f64, b: f64, c: f64, d: f64, s: Option<f64>) -> f64 {
    if s == None {
        let s = Some(1.70158);
    }

    let s = Some(s.unwrap() * 1.525);
    let t = t / d * 2;

    if t < 1.0 {
        return c / 2.0 * (t * t * ((s.unwrap() + 1.0) * t - s.unwrap)) + b;
    } else {
        let t = t - 2.0;
        return c / 2 * (t * t * ((s.unwrap() + 1.0) * t + s.unwrap) + 2.0) + b;
    }
}

/// OutInBack
pub fn out_in_back(t: f64, b: f64, c: f64, d: f64, s: Option<f64>) -> f64 {
    if t < d / 2.0 {
        out_back(t * 2.0, b, c / 2.0, d, s)
    } else {
        in_back((t * 2.0) - d, b + c / 2.0, c / 2.0, d, s)
    }
}

/// OutBounce
pub fn out_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    let t = t / d;
    if t < 1.0 / 2.75 {
        c * (7.5625 * t * t) + b
    } else if t < 2.0 / 2.75 {
        let t = t - (1.5 / 2.75);
        c * (7.5625 * t * t + 0.75) + b
    } else if t < 2.5 / 2.75 {
        let t = t - (2.25 / 2.75);
        c * (7.5625 * t * t + 0.9375) + b
    } else {
        let t = t - (2.625 / 2.75);
        c * (7.5625 * t * t + 0.984375) + b
    }
}

/// InBounce
pub fn in_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    c - out_bounce(d - t, 0.0, c, d) + b
}

/// InOutBounce
pub fn in_out_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        in_bounce(t * 2.0, 0.0, c, d) * 0.5 + b
    } else {
        out_bounce(t * 2.0 - d, 0, c, d) * 0.5 + c * 0.5 + b
    }
}

/// OutInBounce
pub fn out_in_bounce(t: f64, b: f64, c: f64, d: f64) -> f64 {
    if t < d / 2.0 {
        out_bounce(t * 2.0, b, c / 2.0, d)
    } else {
        in_bounce(t * 2.0 - d, b + c / 2.0, c / 2.0, d)
    }
}
