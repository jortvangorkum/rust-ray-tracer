use crate::engine_objects::primitives::Primitive;

use super::{BVHNode, Bin};

const BIN_COUNT: usize = 4;

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
        let bin_count = BIN_COUNT;
        let pool = vec![BVHNode::default(); triangle_count * 2 - 1];
        let triangle_indices: Vec<usize> =  (triangle_index..(triangle_index + triangle_count)).collect();
        
        return BVH {
            bin_count,
            bins:       vec![Bin::default(); bin_count],
            bins_left:  vec![Bin::default(); bin_count - 1],
            bins_right: vec![Bin::default(); bin_count - 1],
            pool,
            pool_ptr: 1,
            triangle_indices,
            triangle_count,
        }
    }

    pub fn build(self: &mut Self, primitives: &Vec<Box<dyn Primitive>>) {
        let mut root = self.pool[0];
        root.first = 0;
        root.count = self.triangle_count;
        root.update_bounds(primitives, &self.triangle_indices);
        root.subdivide_node(self, primitives);
        self.pool[0] = root;
    }

    pub fn reset_bins(self: &mut Self) {
        for bin in self.bins.iter_mut() {
            bin.clear();
        }
        for bin in self.bins_left.iter_mut() {
            bin.clear();
        }
        for bin in self.bins_right.iter_mut() {
            bin.clear();
        }
    }
}