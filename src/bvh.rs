use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Axis;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: Vec<Arc<dyn Hittable>>, time: (f64, f64)) -> Self {
        let mut objects = src_objects;

        let rand_axis: Axis = rand::thread_rng().gen();
        let comparison_fn: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = match rand_axis
        {
            Axis::X => |a, b| box_compare(a, b, Axis::X),
            Axis::Y => |a, b| box_compare(a, b, Axis::Y),
            Axis::Z => |a, b| box_compare(a, b, Axis::Z),
        };

        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                let order_of_objects = comparison_fn(&objects[0], &objects[1]);
                let first_is_left = matches!(order_of_objects, Ordering::Less);

                if first_is_left {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                objects.sort_by(comparison_fn);

                let mid = objects.len() / 2;

                let (list_a, list_b) = objects.split_at_mut(mid);

                (
                    Arc::new(Self::new(list_a.to_vec(), time)) as Arc<dyn Hittable>,
                    Arc::new(Self::new(list_b.to_vec(), time)) as Arc<dyn Hittable>,
                )
            }
        };

        let possible_box_left = left.bounding_box(time);
        let possible_box_right = right.bounding_box(time);

        match (possible_box_left, possible_box_right) {
            (Some(box_left), Some(box_right)) => Self {
                left,
                right,
                aabb: Aabb::surrounding_box(&box_left, &box_right),
            },
            _ => panic!("No bounding box in bvh_node constructor."),
        }
    }
}

fn box_compare(
    surface_a: &Arc<dyn Hittable>,
    surface_b: &Arc<dyn Hittable>,
    axis: Axis,
) -> Ordering {
    let possible_box_a = surface_a.bounding_box((0.0, 1.0));
    let possible_box_b = surface_b.bounding_box((0.0, 1.0));

    match (possible_box_a, possible_box_b) {
        (Some(box_a), Some(box_b)) => {
            if box_a.minimum[axis] < box_b.minimum[axis] {
                return Ordering::Less;
            }
            Ordering::Greater
        }
        _ => panic!("No bounding box in bvh_node constructor."),
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // If it doesn't hit the bounding box, it doesn't hit anything
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        };

        let hit_left: Option<HitRecord> = self.left.hit(ray, t_min, t_max);

        hit_left.map_or_else(
            // If left didn't hit return possible hit from right
            || self.right.hit(ray, t_min, t_max),
            // If left did hit,
            |left_hit_record| {
                // check if right hit something closer
                let right_hit_record = self.right.hit(ray, t_min, left_hit_record.t);
                // and if it does, return that
                // Otherwise, return the original left hit
                Some(right_hit_record.map_or(left_hit_record, |rec| rec))
            },
        )
    }

    fn bounding_box(&self, _time: (f64, f64)) -> Option<Aabb> {
        Some(self.aabb.clone())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::material::dielectric::Dielectric;
    use crate::sphere::Sphere;
    use crate::vec3::Vec3;

    use super::*;

    #[test]
    fn test_new_1() {
        let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();

        let material1 = Dielectric::new(1.5);
        let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, material1);
        scene.push(Arc::new(sphere1));

        let bvh = BvhNode::new(scene, (0.0, 1.0));

        assert_eq!(
            bvh.aabb,
            Aabb {
                minimum: Vec3::new(-1.0, -1.0, -1.0),
                maximum: Vec3::new(1.0, 1.0, 1.0)
            }
        );
    }

    #[test]
    fn test_new_2() {
        let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();

        let material1 = Dielectric::new(1.5);
        let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, material1);
        scene.push(Arc::new(sphere1));

        let material2 = Dielectric::new(1.5);
        let sphere2 = Sphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0, material2);
        scene.push(Arc::new(sphere2));

        let bvh = BvhNode::new(scene, (0.0, 1.0));

        assert_eq!(
            bvh.aabb,
            Aabb {
                minimum: Vec3::new(-1.0, -1.0, -1.0),
                maximum: Vec3::new(6.0, 1.0, 1.0)
            }
        );
    }
}
