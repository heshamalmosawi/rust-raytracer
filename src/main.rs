use std::io;

use rust_raytracer::{
    camera::Camera,
    color::{self, Color},
    geometry::vec3::Point3,
    hittable::hittable_list::HittableList,
    shapes::sphere::Sphere,
    util::{random_double, IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES_PER_PIXEL},
};

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))); // ???????????????
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.get_ray_collor(&world);
            }
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
