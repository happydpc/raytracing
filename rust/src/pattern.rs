use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Point;
use std::rc::Rc;

pub fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern::new(move |p| if p.x().floor() % 2.0 == 0.0 { a } else { b })
}

#[derive(Clone)]
pub struct Pattern {
    func: Rc<dyn Fn(Point) -> Color>,
    inv_transform: Matrix,
}

impl Pattern {
    pub fn new(f: impl 'static + Fn(Point) -> Color) -> Self {
        Pattern {
            func: Rc::new(f),
            inv_transform: Matrix::identity(),
        }
    }

    pub fn at(&self, obj_point: Point) -> Color {
        (self.func)(self.inv_transform * obj_point)
    }

    pub fn set_transform(&mut self, t: Matrix) {
        self.inv_transform = t.inverse();
    }

    pub fn with_transform(mut self, t: Matrix) -> Self {
        self.set_transform(t);
        self
    }

    pub fn inv_transform(&self) -> &Matrix {
        &self.inv_transform
    }
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pattern")
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::approx_eq::ApproximateEq;
    use crate::color::{color, BLACK, WHITE};
    use crate::lights::PointLight;
    use crate::materials::Phong;
    use crate::matrix::{scaling, translation};
    use crate::shapes::sphere;
    use crate::tuple::{point, vector};

    fn test_pattern() -> Pattern {
        Pattern::new(|p| color(p.x(), p.y(), p.z()))
    }

    /// The default pattern transformation
    #[test]
    fn default_transform() {
        let pattern = test_pattern();
        assert_almost_eq!(pattern.inv_transform(), Matrix::identity());
    }

    /// Assigning a transformation
    #[test]
    fn assign_transform() {
        let mut pattern = test_pattern();
        pattern.set_transform(translation(1, 2, 3));
        assert_almost_eq!(pattern.inv_transform(), translation(1, 2, 3).inverse());
    }

    /// A pattern with an object transformation
    #[test]
    fn obj_transform() {
        let shape = sphere().with_transform(scaling(2, 2, 2));
        let pattern = test_pattern();
        let c = shape.pattern_at(&pattern, point(2, 3, 4));
        assert_almost_eq!(c, color(1, 1.5, 2));
    }

    /// A pattern with a pattern transformation
    #[test]
    fn pat_transform() {
        let shape = sphere();
        let pattern = test_pattern().with_transform(scaling(2, 2, 2));
        let c = shape.pattern_at(&pattern, point(2, 3, 4));
        assert_almost_eq!(c, color(1, 1.5, 2));
    }

    /// A pattern with both an object and a pattern transformation
    #[test]
    fn objpat_transform() {
        let shape = sphere().with_transform(scaling(2, 2, 2));
        let pattern = test_pattern().with_transform(translation(0.5, 1, 1.5));
        let c = shape.pattern_at(&pattern, point(2.5, 3, 3.5));
        assert_almost_eq!(c, color(0.75, 0.5, 0.25));
    }

    /// A stripe pattern is constant in y
    #[test]
    fn stripe_y() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_almost_eq!(pattern.at(point(0, 0, 0)), WHITE);
        assert_almost_eq!(pattern.at(point(0, 1, 0)), WHITE);
        assert_almost_eq!(pattern.at(point(0, 2, 0)), WHITE);
    }

    /// A stripe pattern is constant in z
    #[test]
    fn stripe_z() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_almost_eq!(pattern.at(point(0, 0, 0)), WHITE);
        assert_almost_eq!(pattern.at(point(0, 0, 1)), WHITE);
        assert_almost_eq!(pattern.at(point(0, 0, 2)), WHITE);
    }

    /// A stripe pattern alternates in x
    #[test]
    fn stripe_x() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_almost_eq!(pattern.at(point(0, 0, 0)), WHITE);
        assert_almost_eq!(pattern.at(point(0.9, 0, 0)), WHITE);
        assert_almost_eq!(pattern.at(point(1.0, 0, 0)), BLACK);
        assert_almost_eq!(pattern.at(point(-0.1, 0, 0)), BLACK);
        assert_almost_eq!(pattern.at(point(-1.0, 0, 0)), BLACK);
        assert_almost_eq!(pattern.at(point(-1.1, 0, 0)), WHITE);
    }

    /// Lighting with a pattern applied
    #[test]
    fn stripe_light() {
        let m = Phong::new_pattern(stripe_pattern(WHITE, BLACK), 1.0, 0.0, 0.0, 1.0);
        let eyev = vector(0, 0, -1);
        let normalv = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10), WHITE);
        let c1 = m.lighting(&sphere(), &light, point(0.9, 0, 0), eyev, normalv, false);
        let c2 = m.lighting(&sphere(), &light, point(1.1, 0, 0), eyev, normalv, false);
        assert_almost_eq!(c1, WHITE);
        assert_almost_eq!(c2, BLACK);
    }
}
