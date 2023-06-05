use std::{ops, process::Output, clone};
#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    pub fn new(X: f64, Y: f64, Z: f64) -> Self {
        Self {
            x: X,
            y: Y, 
            z: Z
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    

}

impl ops::Add for Vector3 { // Overwrite Addition operations, custom for 2 Vectors
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let result = Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
        result
    }
}

impl ops::Sub for Vector3 { // Same with subtract
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let result = Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
        result
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Self{
        self.clone()
    }
}


