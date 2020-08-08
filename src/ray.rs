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
        vec3::clone(self.origin())+ (vec3::clone(self.direction()) * t)
    }
}

//
pub fn ray_colour(r: Ray) -> vec3::Colour{
    let unit_direction : vec3::Vec3 =  vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    ((1.0 - t) * vec3::Colour{x:1.0,y:1.0,z:1.0}) + (t * vec3::Colour{x:0.5,y:0.7,z:1.0})
}