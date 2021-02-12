use nalgebra::Vector3;

#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub bmin: Vector3<f64>,
    pub bmax: Vector3<f64>,
}

impl AABB {
    pub fn grow_by_point(self: &mut Self, point: &Vector3<f64>) {
        self.bmin = Vector3::new(
            self.bmin.x.min(point.x),
            self.bmin.y.min(point.y),
            self.bmin.z.min(point.z),
        );
        self.bmax = Vector3::new(
            self.bmax.x.max(point.x),
            self.bmax.y.max(point.y),
            self.bmax.z.max(point.z),
        );
    }

    pub fn grow_by_bounds(self: &mut Self, bounds: &AABB) {
        self.grow_by_point(&bounds.bmin);
        self.grow_by_point(&bounds.bmax);
    }

    pub fn surface_area(self: &Self) -> f64 {
        let d = self.bmax - self.bmin;
        return 2.0 * (d.x * d.y + d.y * d.z + d.z * d.x);
    }

    pub fn new() -> AABB {
        return AABB {
            bmin: Vector3::new( std::f64::MAX,  std::f64::MAX,  std::f64::MAX),
            bmax: Vector3::new(-std::f64::MAX, -std::f64::MAX, -std::f64::MAX),
        }
    }

    pub fn reset(self: &mut Self) {
        self.bmin = Vector3::new( std::f64::MAX,  std::f64::MAX,  std::f64::MAX);
        self.bmax = Vector3::new(-std::f64::MAX, -std::f64::MAX, -std::f64::MAX);
    }

    pub fn longest_axis(self: &Self) -> usize {
        let dx = self.bmax.x - self.bmin.x;
        let dy = self.bmax.y - self.bmin.y;
        let dz = self.bmax.z - self.bmin.z;

        let mut l_axis = 0;
        if dy > dx { l_axis = 1; }
        if dz > dy && dz > dx { l_axis = 2; }
        
        return l_axis;
    }
}