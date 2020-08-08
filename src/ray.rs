use crate::vec3;

pub struct Ray{
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3
}


impl Ray{
    pub fn origin(&self) -> &vec3::Point3{
        &self.orig
    }

    pub fn direction(&self) -> &vec3::Point3{
        &self.dir
    }

    // don't necessarily like how this is done and suggests maybe need to rethink how i've implemented my multi,addition, etc.
    pub fn at(&self,t: f64) -> vec3::Point3{
        self.origin() + &(self.direction() * t)
    }
}

