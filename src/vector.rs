use std::{ops};
#[derive(Debug)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

pub type Point3 = Vector3;
pub type Colour3 = Vector3;

impl Colour3 {
    pub fn to_rgb(&self) -> [u8; 3] {
        let r = (self.x * 255.999) as u8;
        let g = (self.y * 255.999) as u8;
        let b = (self.z * 255.999) as u8;
        let result: [u8; 3] = [r, g, b];
        result
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y, 
            z
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    
    pub fn length(&self) -> f32 {
        let len_squared = self.x*self.x + self.y*self.y + self.z*self.z;
        let length = len_squared.sqrt();
        length
    }

    pub fn dot(&self, v: Vector3) -> f32 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
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

impl ops::Mul<f32> for Vector3 { // Make it so you can multiply Vectors by a float 
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let result = Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs);
        result
    }
}

impl ops::Div for Vector3 { // Make it so you can multiply Vectors by a float 
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let result = Vector3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
        result
    }
}

impl ops::Div<f32> for Vector3 { // Make it so you can multiply Vectors by a float 
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let result = Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs);
        result
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let result = Vector3::new(-self.x, -self.y, -self.z);
        result
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        };
    }
}

impl Copy for Vector3 {}

impl Clone for Vector3 {
    fn clone(&self) -> Self{
        *self
    }
}