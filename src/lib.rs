use integrate::prelude::legendre_rule;
use nannou::glam::Vec2;

pub trait ParametricCurve: Send + Sync {
    fn position(&self, t: f32) -> Vec2;
    fn velocity(&self, t: f32) -> Vec2;
    fn a(t: f32) -> f32;

    fn s(&self, t: f32) -> f32 {
        let func = |theta: f32| {
            let vel = self.velocity(theta);
            (vel.x.powi(2) + vel.y.powi(2)).powf(0.5)
        };
        let upper_limit: f32 = t;
        static LOWER_LIMIT: f32 = 0.;
        static N: usize = 50;

        legendre_rule(func, LOWER_LIMIT, upper_limit, N) as f32
    }

    fn n(&self, t: f32) -> Vec2 {
        let vel = self.velocity(t);
        let ds = (vel.x.powi(2) + vel.y.powi(2)).powf(0.5);
        if ds < 1e-6 {
            Vec2::new(1., 0.)
        } else {
            Vec2::new(-vel.y, vel.x) / ds
        }
    }
}

pub trait WaveTransform: ParametricCurve {
    fn normal_offset(&self, t: f32, ta: f32) -> f32 {
        let current_s = self.s(t);
        Self::a(t) * (current_s / ta).sin()
    }

    fn offset(&self, t: f32, ta: f32) -> Vec2 {
        let normal = self.n(t);
        self.position(t) + normal * self.normal_offset(t, ta)
    }
}

impl<T: ParametricCurve> WaveTransform for T {}
