use crate::approx_eq::EPSILON;
use crate::ray::{Intersection, Ray};
use crate::shapes::{Geometry, Shape};
use crate::tuple::{Point, Vector};

pub fn plane() -> Shape {
    Shape::new(Plane::new())
}

#[derive(Debug)]
pub struct Plane;

impl Plane {
    pub fn new() -> Self {
        Plane
    }
}

impl Geometry for Plane {
    fn is_similar(&self, other: &dyn Geometry) -> bool {
        other.as_any().downcast_ref::<Plane>().is_some()
    }

    fn intersect<'a>(&self, obj: &'a Shape, local_ray: &Ray) -> Vec<Intersection<'a>> {
        if local_ray.direction().y().abs() < EPSILON {
            vec![]
        } else {
            vec![Intersection::new(
                -local_ray.origin().y() / local_ray.direction().y(),
                obj,
            )]
        }
    }

    fn normal_at(&self, _: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq::ApproximateEq;
    use crate::tuple::{point, vector};

    /// The normal of a plane is constant everywhere
    #[test]
    fn normal() {
        let p = Plane::new();
        let n1 = p.normal_at(point(0, 0, 0));
        let n2 = p.normal_at(point(10, 0, -10));
        let n3 = p.normal_at(point(-5, 0, 150));
        assert_almost_eq!(n1, vector(0, 1, 0));
        assert_almost_eq!(n2, vector(0, 1, 0));
        assert_almost_eq!(n3, vector(0, 1, 0));
    }

    /// Intersect with a ray parallel to the plane
    #[test]
    fn intersect_parallel() {
        let dummy_shape = plane();
        let p = Plane::new();
        let r = Ray::new(point(0, 10, 0), vector(0, 0, 1));
        let xs = p.intersect(&dummy_shape, &r);
        assert!(xs.is_empty());
    }

    /// Intersect plane with a coplanar ray
    #[test]
    fn intersect_coplanar() {
        let dummy_shape = plane();
        let p = Plane::new();
        let r = Ray::new(point(0, 0, 0), vector(0, 0, 1));
        let xs = p.intersect(&dummy_shape, &r);
        assert!(xs.is_empty());
    }

    /// A ray intersecting a plane from above
    #[test]
    fn intersect_above() {
        let dummy_shape = plane();
        let p = Plane::new();
        let r = Ray::new(point(0, 1, 0), vector(0, -1, 0));
        let xs = p.intersect(&dummy_shape, &r);
        assert_eq!(xs.len(), 1);
        assert_almost_eq!(xs[0].t, 1.0);
    }

    /// A ray intersecting a plane from below
    #[test]
    fn intersect_below() {
        let dummy_shape = plane();
        let p = Plane::new();
        let r = Ray::new(point(0, -1, 0), vector(0, 1, 0));
        let xs = p.intersect(&dummy_shape, &r);
        assert_eq!(xs.len(), 1);
        assert_almost_eq!(xs[0].t, 1.0);
    }
}