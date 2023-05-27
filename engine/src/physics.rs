use crate::geom::{dot, Vec3Unit};

pub fn reflectance(in_dir: Vec3Unit, normal: Vec3Unit, ratio: f64) -> f64 {
    let in_normal = if dot(in_dir, normal) < 0.0 {
        normal
    } else {
        -normal
    };
    let cos = -dot(in_dir, in_normal).min(1.0);
    let r0 = ((1.0 - ratio) / (1.0 + ratio)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub fn reflect(in_dir: Vec3Unit, normal: Vec3Unit) -> Vec3Unit {
    (in_dir - (dot(normal, in_dir) * 2.0) * normal).unit()
}

pub fn refract(in_dir: Vec3Unit, normal: Vec3Unit, ratio: f64) -> Option<Vec3Unit> {
    let in_normal = if dot(in_dir, normal) < 0.0 {
        normal
    } else {
        -normal
    };
    let cos = -dot(in_dir, in_normal).min(1.0);
    let sin = (1.0 - cos * cos).sqrt();
    if ratio * sin > 1.0 {
        return None;
    }
    let out_dir_perp = (in_dir + in_normal * cos) * ratio;
    let out_dir_para = -(1.0 - out_dir_perp.norm()).abs().sqrt() * in_normal;
    Some((out_dir_perp + out_dir_para).unit())
}
