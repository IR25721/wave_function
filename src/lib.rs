use integrate::prelude::legendre_rule;

#[warn(dead_code)]
pub struct PositionAndVelocity {
    x: fn(f32) -> f32,
    y: fn(f32) -> f32,
    dx: fn(f32) -> f32,
    dy: fn(f32) -> f32,
}

impl PositionAndVelocity {
    pub fn new(
        x: fn(f32) -> f32,
        y: fn(f32) -> f32,
        dx: fn(f32) -> f32,
        dy: fn(f32) -> f32,
    ) -> Self {
        Self { x, y, dx, dy }
    }
    fn s(&self, t: f32) -> f32 {
        let func = |theta: f32| ((self.dx)(theta).powi(2) + (self.dy)(theta).powi(2)).powf(0.5);
        let lower_limit = 0.;
        let upper_limit = t;
        let n = 50_usize;
        legendre_rule(func, lower_limit, upper_limit, n) as f32
    }
    fn n(&self, t: f32) -> Vec<f32> {
        let ds = ((self.dx)(t).powi(2) + (self.dy)(t).powi(2)).powf(0.5);
        if ds < 1e-6 {
            vec![1., 0.]
        } else {
            vec![-(self.dy)(t) / ds, (self.dx)(t) / ds]
        }
    }

    fn normal_offset(&self, t: f32, theta: f32, a: fn(f32) -> f32, ta: f32) -> f32 {
        let current_s = self.s(theta);
        a(t) * (current_s / ta).sin()
    }

    pub fn x_offset(&self, t: f32, theta: f32, a: fn(f32) -> f32, ta: f32) -> f32 {
        let normal = self.n(t);
        (self.x)(t) + self.normal_offset(t, theta, a, ta) * normal[0]
    }
    pub fn y_offset(&self, t: f32, theta: f32, a: fn(f32) -> f32, ta: f32) -> f32 {
        let normal = self.n(t);
        (self.y)(t) + self.normal_offset(t, theta, a, ta) * normal[1]
    }
}
