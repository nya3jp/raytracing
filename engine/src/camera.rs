use crate::geom::{cross, Vec3, Vec3Unit};
use crate::ray::Ray;
use crate::rng::Rng;
use crate::time::TimeRange;
use rand::Rng as _;

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3Unit,
    v: Vec3Unit,
    lens_radius: f64,
    time: TimeRange,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time: TimeRange,
    ) -> Camera {
        let viewport_height = 2.0 * (fov / 2.0).atan();
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_at - origin).unit();
        let up = Vec3Unit::Y;
        let u = cross(w, up).unit();
        let v = cross(u, w).unit();

        let horizontal = u * (focus_dist * viewport_width);
        let vertical = v * (focus_dist * viewport_height);
        let lower_left_corner = origin + w * focus_dist - horizontal / 2.0 - vertical / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture / 2.0,
            time,
        }
    }

    pub fn ray(&self, u: f64, v: f64, rng: &mut Rng) -> Ray {
        let lens = Vec3::random_in_unit_disc(rng) * self.lens_radius;
        let blur = self.u * lens.x + self.v * lens.y;
        let origin = self.origin + blur;
        let target = self.lower_left_corner + self.horizontal * u + self.vertical * v;
        let time = rng.gen_range(self.time.lo..=self.time.hi);
        Ray::new(origin, (target - origin).unit(), time)
    }
}
