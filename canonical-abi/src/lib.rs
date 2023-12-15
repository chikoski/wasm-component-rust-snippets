//#[repr(C)]
//#[repr(packed)]
pub struct Point {
    x: i32,
    y: u8,
    z: u16,
}

#[no_mangle]
pub fn new(x: i32, y: u8, z: u16) -> Point {
    Point { x, y, z }
}

#[no_mangle]
pub fn hash(point: &Point) -> u64 {
    (point.x as u64) << 32 | (point.y as u64) << 16 | point.z as u64
}
