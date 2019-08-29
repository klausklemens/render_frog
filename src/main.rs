use std::fs::File;
use std::io::prelude::*;

mod util;
use crate::util::color::ColorRGB;

mod objects;
use crate::objects::{vector::*, sphere::*};

mod raycaster;

mod consts;

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
    debug_file.write(b"P5\n750 750\n6\n")?;
    
    let mut file = File::create("output/lol.ppm")?;
    file.write(b"P6\n750 750B\n255\n")?;
    
    fn deg_to_rad(angle_in_deg: f32) -> f32 { angle_in_deg * ((2.0 * std::f32::consts::PI) / 360.0) }
    
    let pixeldistance: f32 = 2.0 / consts::PIXELS_PER_SIDE as f32;
    let camera_median_pitch = deg_to_rad(consts::CAMERA_ROTATION.x - 90.0);
    let camera_median_yaw = deg_to_rad(0.0 - consts::CAMERA_ROTATION.z);
    let mut y_ray: f32 = 1.0;
    for _j in 0..consts::PIXELS_PER_SIDE {
        let camera_pitch = camera_median_pitch + (y_ray * 0.43);

        let mut x_ray: f32 = -1.0;
        for _i in 0..consts::PIXELS_PER_SIDE {
            let camaera_yaw = camera_median_yaw + (x_ray * 0.43);
            let ray_direction = Vector {
                x: camaera_yaw.sin() * camera_pitch.cos(),
                y: camaera_yaw.cos() * camera_pitch.cos(),
                z: camera_pitch.sin(),
            };
            
            let pixel_color = raycaster::get_pixel_color(consts::CAMERA_POSITION, ray_direction, 4, &objects, &mut debug_file);
            file.write(&[pixel_color.r, pixel_color.g, pixel_color.b])?;

            // println!("pixel: ({}|{})", x_ray, y_ray);
            x_ray += pixeldistance;
        }
        //println!("row {} complete", y_ray);
        y_ray -= pixeldistance;
    }

    file.write(b"\n")?;
    file.flush()?;

    debug_file.write(b"\n")?;
    debug_file.flush()?;
    println!("Done");
    Ok(())
}
