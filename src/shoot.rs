use nalgebra::Vector2;

use crate::world::{World, entity_id};

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

pub fn raycast2d_entities_except_player(world: &World, pos: Vector2<f32>, dir: Vector2<f32>) -> Option<(entity_id, Vector2<f32>)>{

	//let a = dir.y / dir.x;

	let mut target: Option<entity_id> = None;
	let mut target_dist: f32 = f32::INFINITY;
	let mut target_pos: Vector2<f32> = Vector2::new(0., 0.);

	for e in world.get_entities(){
		if e.id() != world.player_id.unwrap(){
			let intersection = ray_intersection(pos, dir, e.position(), e.bounding_box());

			if let Some(extracted) = intersection{
				let dist = (extracted - pos).norm();
				if dist < target_dist{
					target_dist = dist;
					target_pos = extracted;
					target = Some(e.id());
				}
			}
		}
	}

	match target {
		None => None,
		Some(eid) => {
			Some((eid, target_pos))
		}
	}

}

fn find_intersection(pos: Vector2<f32>, dir: Vector2<f32>, a: Vector2<f32>, b: Vector2<f32>) -> Option<Vector2<f32>> {
	let line_dir = b - a;

	let denom = -line_dir.x * dir.y + line_dir.y * dir.x;

	if denom == 0.{
		return None;
	}

	let m_solution = (dir.y * (a.x - pos.x) - dir.x * (a.y - pos.y)) / denom;
	let t_solution = (line_dir.x * (pos.y - a.y) + line_dir.y * (a.x - pos.x)) / denom;

	if m_solution >= 0. && m_solution <= 1. && t_solution >= 0.{
		return Some(t_solution * dir + pos);
	}

	None
}

fn min_sol(pos: Vector2<f32>, point_1: Option<Vector2<f32>>, point_2: Option<Vector2<f32>>) -> Option<Vector2<f32>>{
	if point_1.is_none() && point_2.is_none(){
		return None;
	}

	if point_1.is_none(){
		return Some(point_2.unwrap());
	}
	else if point_2.is_none(){
		return Some(point_1.unwrap());
	}

	if (point_2? - pos).norm() < (point_1? - pos).norm(){
		Some(point_2?)
	}
	else{
		Some(point_1?)
	}
}

fn ray_intersection(pos: Vector2<f32>, dir: Vector2<f32>, target: Vector2<f32>, bounding_box: Vector2<f32>)
-> Option<Vector2<f32>>{

	let a = Vector2::new(target.x-bounding_box.x, target.y-bounding_box.y);
	let b = Vector2::new(target.x-bounding_box.x, target.y+bounding_box.y);

	let mut intersection = find_intersection(pos, dir, a, b);

	//
	let a = Vector2::new(target.x+bounding_box.x, target.y-bounding_box.y);
	let b = Vector2::new(target.x+bounding_box.x, target.y+bounding_box.y);

	intersection = min_sol(pos, intersection, find_intersection(pos, dir, a, b));

	//
	let a = Vector2::new(target.x-bounding_box.x, target.y-bounding_box.y);
	let b = Vector2::new(target.x+bounding_box.x, target.y-bounding_box.y);

	intersection = min_sol(pos, intersection, find_intersection(pos, dir, a, b));

	//
	let a = Vector2::new(target.x-bounding_box.x, target.y+bounding_box.y);
	let b = Vector2::new(target.x+bounding_box.x, target.y+bounding_box.y);

	intersection = min_sol(pos, intersection, find_intersection(pos, dir, a, b));

	intersection
}
