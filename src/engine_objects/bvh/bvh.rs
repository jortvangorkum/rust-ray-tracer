use crate::engine_objects::primitives::Primitive;

use super::{BVHNode, Bin};

pub struct BVH {
    pub bin_count: usize,
    pub bins: Vec<Bin>,
    pub bins_left: Vec<Bin>,
    pub bins_right: Vec<Bin>,
    pub pool: Vec<BVHNode>,
    pub pool_ptr: usize,
    pub triangle_indices: Vec<usize>,
    pub triangle_count: usize,
}

impl BVH {
    pub fn new(triangle_index: usize, triangle_count: usize) -> BVH {
        let bin_count = 4;
        let pool = Vec::with_capacity((triangle_count * 2 - 1) as usize);
        let mut triangle_indices = Vec::with_capacity(triangle_count as usize);

        for (i, t) in triangle_indices.iter_mut().enumerate() {
            *t = triangle_index + i;
        }
        
        return BVH {
            bin_count,
            bins:       Vec::with_capacity(bin_count as usize),
            bins_left:  Vec::with_capacity((bin_count - 1) as usize),
            bins_right: Vec::with_capacity((bin_count - 1) as usize),
            pool,
            pool_ptr: 1,
            triangle_indices,
            triangle_count,
        }
    }

    pub fn build(self: &mut Self, scene: &Vec<Box<dyn Primitive>>) {
        let root = &mut self.pool[0];
        root.first = 0;
        root.count = self.triangle_count;
        root.update_bounds(triangle_indices);
        root.subdivide_node(self, scene, &self.pool, &self.triangle_indices, &self.pool_ptr);
    }

    pub fn reset_bins(self: &mut BVH) {
        for bin in self.bins.iter() {
            bin.clear();
        }
        for bin in self.bins_left.iter() {
            bin.clear();
        }
        for bin in self.bins_right.iter() {
            bin.clear();
        }
    }
}