use crate::util::color::ColorRGB;

use crate::objects::{vector::*, drawable::Drawable};

struct HitInfo {
    object_index: usize,
    location: Vector,
    distance: f32,
    circle_count: u16,
}

#[allow(dead_code)] // TODO: REMOVE
enum RayResult {
    Hit(HitInfo),
    Miss {
        circle_count: u16,
    },
}

fn cast_ray<T>(origin: &Vector, direction: &Vector, half_pixel_unit_size: f32, ray_max_length: f32, objects: &[T]) -> RayResult
    where T: Drawable
{
    let mut hit_data = HitInfo {
        object_index: 0,
        location: origin.clone(),
        distance: 0.0,
        circle_count: 1,
    };
    loop {
        let mut distance_to_closest: f32 = ray_max_length;
        for (i, object) in objects.iter().enumerate() {
            let distance = object.distance(&hit_data.location);
            if distance < distance_to_closest {
                distance_to_closest = distance;
                hit_data.object_index = i;
            }
        }
        
        if hit_data.distance >= ray_max_length {
            return RayResult::Miss {
                circle_count: hit_data.circle_count,
            };
        }

        if distance_to_closest <= half_pixel_unit_size * hit_data.distance {
            return RayResult::Hit(hit_data);
        } else {
            hit_data.location = hit_data.location.add_vector(&direction.mul(distance_to_closest));
            hit_data.distance += distance_to_closest;
        }
        hit_data.circle_count += 1;
    }
}

fn cast_bouncable_ray<T>(origin: &Vector, direction: &Vector, half_pixel_unit_size: f32, ray_max_length: f32, bounce_limit: u8, objects: &[T]) -> Vec<RayResult>
    where T: Drawable
{
    let mut hits = Vec::new();
    hits.push(cast_ray(origin, direction, half_pixel_unit_size, ray_max_length, objects));
    
    let mut last_direction = direction.clone();
    for _i in 0..bounce_limit {
        let last_hit_location;
        let last_hit_object;

        if let RayResult::Hit(hit_info) = &hits[hits.len() - 1] {
            last_hit_object = &objects[hit_info.object_index];
            last_hit_location = &hit_info.location;

            // https://www.stackoverflow.com/a/573206 Bounce
            let hit_surface_normal = last_hit_object.get_surface_normal(last_hit_location);
            let dot_product = last_direction.dot(&hit_surface_normal);

		    let wall_perpendicular = hit_surface_normal.mul(dot_product);  // u
		    let wall_parallel = last_direction.sub_vector(&wall_perpendicular);     // w

		    last_direction = wall_parallel.sub_vector(&wall_perpendicular).unit();
        } else {
            break;
        }

        let ray_result = cast_ray(last_hit_location, &last_direction, half_pixel_unit_size, ray_max_length, objects);
        hits.push(ray_result);
    }
    return hits;
}

const PIXELS_PER_SIDE:u16 = 750;

const SKY_COLOR:ColorRGB = ColorRGB {r: 190, g: 255, b: 240};
const RENDER_DISTANCE:f32 = 15.0;

pub fn get_pixel_color<T, U>(origin: Vector, direction: Vector, bounce_limit: u8, objects: &[T], debug_file: &mut U) -> ColorRGB
    where T: Drawable, U: std::io::Write
{
    let half_pixel_unit_size = (0.43 / PIXELS_PER_SIDE as f32).tan(); // half pixel width/height at a distance of one

    let rays = cast_bouncable_ray(&origin, &direction.unit(), half_pixel_unit_size, RENDER_DISTANCE, bounce_limit, objects);

    debug_file.write(&[rays.len() as u8]).unwrap();
    /*match &rays[0] {
        RayResult::Hit(hit) => {
            //debug_file.write(&[(hit.circle_count/2) as u8]).unwrap();
            debug_file.write(&[rays.len() as u8]).unwrap();
            return objects[hit.object_index].get_color().clone();
        },
        RayResult::Miss { circle_count } => {
            //debug_file.write(&[(*circle_count/2) as u8]).unwrap();
            debug_file.write(&[rays.len() as u8]).unwrap();
            return SKY_COLOR;
        }
    }*/
    for ray in (&rays).iter().rev() {
        if let RayResult::Hit(hit) = ray {
            return objects[hit.object_index].get_color().clone();
        }
    }
    SKY_COLOR
}
