mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::io;
use vec3::{Point3, Vec3};
/*
check if the ray hits the sphere
given a sphere located at the origin:  x^2 + y^2 + z^2 = R^2
We can check if ray P(t) = A + t*b intersects the sphere by substituting the ray equation into the sphere equation
We can express a sphere in vector form as dot((P(t) - C), (P(t) - C)) = R^2 where C is the center of the sphere
and C = (0, 0, 0) for a sphere at the origin. If the ray hits the sphere, there will be a real solution for t.
We know A, b, and R, so we can substitute them into the equation and solve for t.
If the discriminant is negative, the ray does not hit the sphere. If the discriminant is zero, the ray grazes the sphere.
If the discriminant is positive, the ray hits the sphere at two points.
*/
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center; // vector from origin of ray to center of sphere
    let a = r.direction().length_squared(); // squared length of ray direction
    let half_b = vec3::dot(oc, r.direction()); // dot product of ray direction and vector from origin of ray to center of sphere
    let c = oc.length_squared() - radius * radius; // squared length of vector from origin of ray to center of sphere
    let discriminant = half_b * half_b - 4.0 * a * c; // high school math: b^2 - 4ac
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        let direction = rec.normal + vec3::random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);
    }

    let t = hit_sphere(Point3::new(0.0, 0.0, -2.0), 0.5, r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -2.0)); // normal relative to center of sphere
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0); // map normal vector to color
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0); // scale y to [0, 1]
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0) // linear blend
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //camera

    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}
