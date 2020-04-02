use crate::matrix::Matrix;
use crate::shapes::Shape;
use crate::tuple::Tuple;

pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Tuple {
        self.origin
    }

    pub fn direction(&self) -> Tuple {
        self.direction
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix) -> Self {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub obj: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, obj: &'a dyn Shape) -> Self {
        Intersection { t, obj }
    }
}

pub fn hit<'a>(xs: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    xs.iter()
        .fold(None, |h, i| match (h, i.t) {
            (_, t) if t < 0.0 => h,
            (None, _) => Some(i),
            (Some(h), t) if t < h.t => Some(i),
            _ => h,
        })
        .copied()
}

#[macro_export]
macro_rules! intersections {
    ($($x:expr),* $(,)?) => {
        vec![$($x),*]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::{rotation_x, scaling, translation};
    use crate::shapes::Sphere;
    use crate::tuple::{point, vector};
    use std::f32::consts::PI;

    /// Creating and querying a ray
    #[test]
    fn rays() {
        let origin = point(1, 2, 3);
        let direction = vector(4, 5, 6);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin(), origin);
        assert_eq!(r.direction(), direction);
    }

    /// Computing a point from a distance
    #[test]
    fn ray_position() {
        let r = Ray::new(point(2, 3, 4), vector(1, 0, 0));
        assert_eq!(r.position(0.0), point(2, 3, 4));
        assert_eq!(r.position(1.0), point(3, 3, 4));
        assert_eq!(r.position(-1.0), point(1, 3, 4));
        assert_eq!(r.position(2.5), point(4.5, 3, 4));
    }

    /// An intersection encapsulates t and object
    #[test]
    fn intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(i.obj as *const _, &s as *const _);
    }

    /// Aggregating intersections
    #[test]
    fn intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    /// The hit, when all intersections have positive t
    #[test]
    fn hit_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = intersections![i2, i1];
        let i = hit(&xs);
        assert_eq!(i, Some(i1));
    }

    /// The hit, when some intersections have negative t
    #[test]
    fn hit_part_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = intersections![i2, i1];
        let i = hit(&xs);
        assert_eq!(i, Some(i2));
    }

    /// The hit, when all intersections have negative t
    #[test]
    fn hit_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = intersections![i2, i1];
        let i = hit(&xs);
        assert_eq!(i, None);
    }

    /// The hit is always the lowest nonnegative intersection
    #[test]
    fn hit_lowest_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = intersections![i1, i2, i3, i4];
        let i = hit(&xs);
        assert_eq!(i, Some(i4));
    }

    /// Translating a ray
    #[test]
    fn translate() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = translation(3, 4, 5);
        let r2 = r.transform(m);
        assert_eq!(r2.origin(), point(4, 6, 8));
        assert_eq!(r2.direction(), vector(0, 1, 0));
    }

    /// Scaling a ray
    #[test]
    fn scale() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = scaling(2, 3, 4);
        let r2 = r.transform(m);
        assert_eq!(r2.origin(), point(2, 6, 12));
        assert_eq!(r2.direction(), vector(0, 3, 0));
    }

    /// Rotating a ray
    #[test]
    fn rotate() {
        let r = Ray::new(point(1, 2, 3), vector(0, 1, 0));
        let m = rotation_x(PI / 2.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin(), point(1, -3, 2));
        assert_eq!(r2.direction(), vector(0, 0, 1));
    }
}