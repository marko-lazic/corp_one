#[cfg(test)]
mod tests {
    use std::ops::Neg;

    use bevy::prelude::*;

    #[test]
    fn test_add_two_vectors() {
        let a = Vec2::new(2., 1.);
        let b = Vec2::new(-1., 1.);
        let c = a + b;
        assert_eq!(c, Vec2::new(1., 2.))
    }

    #[test]
    fn test_subtract_two_vectors() {
        let a = Vec2::new(2., 1.);
        let b = Vec2::new(-1., 1.);
        let c = a - b;
        assert_eq!(c, Vec2::new(3., 0.))
    }

    #[test]
    fn test_distance_two_vectors() {
        let a = Vec2::new(2., 1.);
        let b = Vec2::new(-1., 1.);
        let c = a.distance(b);
        assert_eq!(c, 3.0);
    }

    #[test]
    fn test_distance_two_vectors_another_way() {
        let a = Vec2::new(2., 1.);
        let b = Vec2::new(-1., 1.);
        let c = (a - b).length();
        // another abstraction of calculating distance
        // sqrt((a.x - b.x)^2 + (a.y - b.y)^2)
        assert_eq!(c, 3.0);
    }

    #[test]
    fn test_negate_vector() {
        let a = Vec2::new(2., 1.);
        let a = a.neg();
        assert_eq!(a, Vec2::new(-2., -1.));
    }

    #[test]
    fn test_vector_length_or_magnitude() {
        let a = Vec2::new(3., 3.);
        assert_eq!(a.length(), 4.2426405);
    }

    #[test]
    fn test_sign() {
        let a = Vec2::new(-2., 1.);
        assert_eq!(a.signum(), Vec2::new(-1., 1.));
    }

    #[test]
    fn test_normalization() {
        let a = Vec2::new(2., 1.);
        let a = a.normalize();
        assert_eq!(a, Vec2::new(0.8944272, 0.4472136));
    }

    #[test]
    fn test_normalize_by_length() {
        let a = Vec2::new(2., 1.);
        let a = a / (a - Vec2::new(0., 0.)).length();
        assert_eq!(a, Vec2::new(0.8944272, 0.4472136));
    }

    #[test]
    fn the_dot_product() {
        let a = Vec2::new(0., 1.).normalize();
        let b = Vec2::new(1., 1.).normalize();
        println!("a {:?}, b {:?}", a, b);
        let dot_prod_a_b = a.dot(b);
        let dot_prod_b_a = b.dot(a);
        println!("a dot b {:?}, b dot a {:?}", dot_prod_a_b, dot_prod_b_a);
        println!("angle {} degrees", a.angle_between(b).to_degrees());
    }

    #[test]
    fn impact_sound_based_on_dot_product() {
        // volume = dot(velocity, plane_normal)
        let velocity = Vec2::new(5.0, -3.1);
        let plane_normal = Vec2::new(1.0, 0.9).normalize();
        let volume = velocity.dot(plane_normal);
        println!("impact volume: {}", volume);
    }

    // Look-at Trigger. Threshold from 0-1
    // 1 = very strict, needs to look exactly toward the thing
    // 0 = perpendicular or closer means your are looking at it
    #[test]
    fn look_at_trigger() {
        // volume = dot(velocity, plane_normal)
        let player_facing = Vec2::new(2.0, 4.0).normalize();
        let object = Vec2::new(4.0, 3.0).normalize();
        let threshold = player_facing.dot(object);
        println!("look at threshold: {}", threshold);
    }

    #[test]
    fn function_world_to_local() {
        let object = Vec2::new(2.0, 4.0);
        let point_global = Vec2::new(4.0, 3.0);
        let local = to_local(object, point_global);
        println!("point to_local {:?}", local);
        assert_eq!(local, Vec2::new(2.0, -1.0));
    }

    fn to_local(object: Vec2, global_point: Vec2) -> Vec2 {
        global_point - object
    }

    #[test]
    fn function_local_to_world() {
        let object = Vec2::new(2.0, 4.0);
        let point_local = Vec2::new(2.0, -1.0);
        let world = to_world(object, point_local);
        println!("to_world {:?}", world);
        assert_eq!(world, Vec2::new(4.0, 3.0));
    }

    fn to_world(object: Vec2, local_point: Vec2) -> Vec2 {
        object + local_point
    }
}
