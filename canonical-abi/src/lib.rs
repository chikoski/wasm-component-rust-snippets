//#[repr(C)]
//#[repr(packed)]
pub struct Point {
    x: i32,
    y: u8,
    z: u16,
}

impl Point {
    #[no_mangle]
    pub fn new(x: i32, y: u8, z: u16) -> Self {
        Point { x, y, z }
    }
    
    #[no_mangle]
    pub fn hash(&self) -> u64 {
        (self.x as u64) << 32 | (self.y as u64) << 16 | self.z as u64
    }

    #[no_mangle]
    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}