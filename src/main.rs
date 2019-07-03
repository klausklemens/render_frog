use std::fs::File;
use std::io::prelude::*;

mod util;
use crate::util::color::ColorRGB;

mod objects;
use crate::objects::{vector::*, sphere::*, drawable::Drawable};

const SKY_COLOR:ColorRGB = ColorRGB {r: 190, g: 255, b: 240};
const RENDER_DISTANCE:f32 = 15.0;
fn cast_ray(origin: Vector, direction: Vector, bounce_limit: u8, objects: &[Sphere], debug_file: &mut File) -> ColorRGB { // return ()
    let direction = direction.unit();
    
    let pixel_unit_size = (0.43 / PIXELS_PER_SIDE as f32).tan(); // half pixel width/height at a distance of one

    let mut hit_object: &Sphere = &Sphere::new();
    let mut hit_location: Vector = origin;
    let mut hit_distance: f32 = 0.0;
    let mut circle_count: u8 = 1;
    loop {
        let mut distance_to_closest: f32 = RENDER_DISTANCE;
        for object in objects {
            let distance = object.distance(&hit_location);
            if distance < distance_to_closest {
                distance_to_closest = distance;
                hit_object = object;
            }
        }
        
        if hit_distance >= RENDER_DISTANCE {
            // println!("out of range");
            break;
        }

        if distance_to_closest <= pixel_unit_size * hit_distance {
            // println!("hit");
            break;
        } else {
            hit_location = hit_location.add_vector(&direction.mul(distance_to_closest));
            hit_distance += distance_to_closest;
        }
        circle_count += 1;
    }

    debug_file.write(&[circle_count]);

    if hit_distance <= 0.0 || hit_distance >= RENDER_DISTANCE {
        SKY_COLOR
    } else {
        hit_object.color.clone()
    }
}

const CAMERA_POSITION:Vector = Vector {x: 7.48, y: -6.50, z: 5.34};
const CAMERA_ROTATION:Vector = Vector {x: 63.6, y: 0.0, z: 46.7};
const PIXELS_PER_SIDE:u16 = 500;
const PIXELDISTANCE:f32 = 2.0 / PIXELS_PER_SIDE as f32;
fn main() -> std::io::Result<()> {
    let objects = [
        Sphere {
            center: Vector::new(-1.84731, -2.30269, 0.270243),
            radius: 1.65843,
            color: ColorRGB::new(255, 255, 255),
        },
        Sphere {
            center: Vector::new(1.30585, -1.81446, 0.827292),
            radius: 1.0,
            color: ColorRGB::new(104, 255, 162),
        },
        Sphere {
            center: Vector::new(1.11094, 1.44646, 1.74862),
            radius: 0.479237,
            color: ColorRGB::new(255, 255, 255),
        },
        Sphere {
            center: Vector::new(3.62087, -0.594781, 2.26308),
            radius: 0.608286,
            color: ColorRGB::new(255, 104, 162),
        },
        Sphere {
            center: Vector::new(3.09755, 0.710382, -0.612779),
            radius: 1.48277,
            color: ColorRGB::new(234, 196, 255),
        },
        Sphere {
            center: Vector::new(0.263811, 4.14569, 1.35786),
            radius: 0.79056,
            color: ColorRGB::new(255, 255, 168),
        },
    ];

    // output
    let mut debug_file = File::create("output/metadata.pgm")?;
    debug_file.write(b"P5\n500 501\n100\n")?;
    
    let mut file = File::create("output/lol.ppm")?;
    file.write(b"P6\n500 501\n255\n")?;
    
    fn deg_to_rad(angle_in_deg: f32) -> f32 { angle_in_deg * ((2.0 * std::f32::consts::PI) / 360.0) }
    
    let camera_median_pitch = deg_to_rad(CAMERA_ROTATION.x - 90.0);
    let camera_median_yaw = deg_to_rad(0.0 - CAMERA_ROTATION.z);
    let mut y_ray: f32 = 1.0;
    for j in 0..PIXELS_PER_SIDE {
        let camera_pitch = camera_median_pitch + (y_ray * 0.43);

        let mut x_ray: f32 = -1.0;
        for i in 0..PIXELS_PER_SIDE {
            let camaera_yaw = camera_median_yaw + (x_ray * 0.43);
            let ray_direction = Vector {
                x: camaera_yaw.sin() * camera_pitch.cos(),
                y: camaera_yaw.cos() * camera_pitch.cos(),
                z: camera_pitch.sin(),
            };
            
            let pixel_color = cast_ray(CAMERA_POSITION, ray_direction, 4, &objects, &mut debug_file);
            file.write(&[pixel_color.r, pixel_color.g, pixel_color.b])?;

            // println!("pixel: ({}|{})", x_ray, y_ray);
            x_ray += PIXELDISTANCE;
        }
        println!("row {} complete", y_ray);
        y_ray -= PIXELDISTANCE;
    }

    file.write(b"\n")?;
    file.flush()?;

    debug_file.write(b"\n")?;
    debug_file.flush()?;
    println!("Done");
    Ok(())
}
