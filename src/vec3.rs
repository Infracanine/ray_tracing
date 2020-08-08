use std::ops;
use std::fmt;

pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64

}
// Aliases
pub type Colour = Vec3;
pub type Point3 = Vec3;

// Methods
#[allow(dead_code)]
impl Vec3{
    // Operations on vectors
    pub fn neg(&self) -> Vec3{
        Vec3{x: -self.x,y: -self.y, z: -self.z}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl ops::Add for Vec3{
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}


impl ops::Add for &Vec3{
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::Add<&Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: &self.x + other.x,
            y: &self.y + other.y,
            z: &self.z + other.z
        }
    }
}



impl ops::Sub for Vec3{
    type Output = Vec3;
    
    fn sub(self, other:Self) -> Self{
        self + other.neg()
    }
}

impl ops::Sub for &Vec3{
    type Output = Vec3;
    fn sub(self, other:Self) -> Vec3{
        new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3{
        Vec3{x: self.x * rhs,y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Mul<Vec3> for f64{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3{
        rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3{
        Vec3{x: self.x * rhs.x,y: self.y * rhs.y,z: self.z * rhs.z}
    }
}

// Implements multiplication for references to a vector3 in all three cases

// Case 1: &vec * &vec
impl ops:: Mul<&Vec3> for &Vec3{
    type Output = Vec3;
    fn mul(self,rhs:&Vec3) -> Vec3{
        Vec3{x: self.x * rhs.x,y: self.y * rhs.y,z: self.z * rhs.z}
    }
}

// Case 2: &vec * f64
impl ops::Mul<f64> for &Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3{
        Vec3{x: self.x * rhs,y: self.y * rhs, z: self.z * rhs}
    }
}

// Case 3: f64 * &vec
impl ops::Mul<&Vec3> for f64{
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3{
        rhs * self
    }
}

impl ops::Div<f64> for Vec3{
    type Output = Vec3;
    
    fn div(self, rhs: f64) -> Vec3{
        self * (1.0/rhs)
    }
}

// Display 

impl fmt::Display for Vec3{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{} {} {}",self.x,self.y,self.z)
    }
}

// Utility Functions


pub fn zeroes() -> Vec3{
    Vec3{x: 0.0,y: 0.0, z: 0.0}
}

pub fn new(X: f64, Y: f64, Z: f64) -> Vec3{
    Vec3{
        x: X,
        y: Y, 
        z: Z}
}

pub fn dot(u : &Vec3, v: &Vec3) -> f64{
    (u.x * v.x) + (u.y * v.y) + (u.z * v.z)
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3{
    Vec3{
        x: (u.y * v.z) - (u.z * v.y),
        y: (u.z * v.x) - (u.x - v.z),
        z: (u.x * v.y) - (u.y * v.x)}
}

pub fn unit_vector(u: &Vec3) -> Vec3{
    let v = Vec3{x: u.x,y: u.y,z:u.z};
    v / u.length()
}

pub fn clone(u: &Vec3) -> Vec3{
    new(u.x,u.y,u.z)
}
