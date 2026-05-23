use nalgebra::Vector2;

use crate::world::World;

// extracted part of raycaster to make shooting possible
pub fn raycast2d(world: &World, pos: Vector2<f32>, dir: Vector2<f32>) -> Vector2<f32>{

    let ray_dir = dir;

    let mut map_pos = Vector2::new(pos.x as i32, pos.y as i32);
    let delta_dist = Vector2::new((1.0 / ray_dir.x).abs(), (1.0 / ray_dir.y).abs());
    let step = ray_dir.map(|c| c.signum() as i32);

    let mut side_dist = Vector2::new(
        if ray_dir.x < 0.0 {
            (pos.x - map_pos.x as f32) * delta_dist.x
        } else {
            (map_pos.x as f32 + 1.0 - pos.x) * delta_dist.x
        },
        if ray_dir.y < 0.0 {
            (pos.y - map_pos.y as f32) * delta_dist.y
        } else {
            (map_pos.y as f32 + 1.0 - pos.y) * delta_dist.y
        },
    );

    let mut hit = false;
    while !hit {
        if side_dist.x < side_dist.y {
            side_dist.x += delta_dist.x;
            map_pos.x += step.x;
        } else {
            side_dist.y += delta_dist.y;
            map_pos.y += step.y;
        }
        if world.get_wall_tile(map_pos.x as usize, map_pos.y as usize) != 0 {
            hit = true;
        }
    }

    return Vector2::new(map_pos.x as f32, map_pos.y as f32);

}
