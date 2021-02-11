use super::AABB;


pub struct Bin {
    pub count: u32,
    pub bounds: AABB,   
}

impl Bin {
    pub fn clear(self: &mut Self) {
        self.count = 0;
        self.bounds.reset();
    }
}