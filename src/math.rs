use std::f64::consts::PI;
pub struct Rotor {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
    theta: f64,
    v: f64,
    l: f64,
}

impl Rotor {
    pub fn get_point(&self) -> (f64, f64) {
        // sin(theta) = dy / r
        let dy = self.theta.sin() * self.r;
        // y = cy + dy
        let y = self.cy + dy;

        // cos(theta) = dx / r
        let dx = self.theta.cos() * self.r;
        // x = cx + dx
        let x = self.cx + dx;

        (x, y)
    }

    pub fn advance(&mut self) {
        self.theta += self.v;
        // self.cx += self.v * 2.0;
        // self.cy += self.v * 2.0;
    }
}

pub fn init_rotors() -> (Rotor, Rotor) {
    let a = Rotor {
        cx: 400.0,
        cy: 200.0,
        r: 80.0,
        theta: 0.0,
        v: 0.02,
        l: 200.0,
    };

    let b = Rotor {
        cx: 450.0,
        cy: 200.0,
        r: 80.0,
        theta: PI / 4.0,
        v: 0.02008,
        l: 240.0,
    };

    (a, b)
}

pub fn get_intersection(a: &Rotor, b: &Rotor) -> ((f64, f64), (f64, f64)) {
    let r1 = a.l;
    let r2 = b.l;

    let (x1, y1) = a.get_point();

    let (x2, y2) = b.get_point();

    let cdx = x1 - x2;
    let cdy = y1 - y2;

    let dist = (cdx * cdx + cdy * cdy).sqrt();

    let dist2 = dist * dist;
    let dist4 = dist2 * dist2;

    let a = (r1 * r1 - r2 * r2) / (2.0 * dist2);
    let r1r2 = r1 * r1 - r2 * r2;
    let c = (2.0 * (r1 * r1 + r2 * r2) / dist2 - (r1r2 * r1r2) / dist4 - 1.0).sqrt();

    let fx = (x1 + x2) / 2.0 + a * (x2 - x1);
    let gx = c * (y2 - y1) / 2.0;

    let fy = (y1 + y2) / 2.0 + a * (y2 - y1);
    let gy = c * (x1 - x2) / 2.0;

    ((fx + gx, fy + gy), (fx - gx, fy - gy))
}
