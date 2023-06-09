use crate::vector::{Vector3, Point3};

pub struct Ray {
    origin: Point3,
    direction: Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Vector3 {
        let orig = self.origin.clone();
        let dest = self.direction.clone();
        return orig + dest*t;
    }
}