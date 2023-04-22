/// A 3D point
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

// fn dot_product(one: Point3D, other: Point3D) -> i32 {
//     return one.x * other.x + one.y * other.y + one.z * other.z;
// }

pub fn distance_squared(point1: &Point3D, point2: &Point3D) -> i32 {
    return (point1.x - point2.x).pow(2)
        + (point1.y - point2.y).pow(2)
        + (point1.z - point2.z).pow(2);
}

// pub struct Camera {
//     pub position: Point3D,

//     /// Any point along the vector
//     /// running from `position`
//     /// to the camera's "target"
//     pub target: Point3D,
// }

// impl Camera {
//     fn new(position: Point3D, target: Point3D) -> Result<Camera, String> {
//         let camera = Camera { position, target };
//         Ok(camera)
//     }
// }

pub struct Sphere {
    pub center: Point3D,
    pub radius: i32,
}

// pub struct Scene {
//     pub camera: Camera,
// }
