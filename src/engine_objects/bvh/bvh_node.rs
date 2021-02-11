use crate::{EPSILON, engine_objects::primitives::Primitive};

use super::{AABB, BVH};

#[derive(Clone, Copy)]
pub struct BVHNode {
    pub bounds: AABB,
    pub is_leaf: bool,
    pub left: usize,
    pub first: usize,
    pub count: usize,
    pub split_axis: u32,
}

impl BVHNode {
    pub fn subdivide_node(self: &mut Self, bvh: &mut BVH, scene: &Vec<Box<dyn Primitive>>, pool: &Vec<BVHNode>, triangle_indices: &Vec<usize>, pool_ptr: &usize) {
        if self.count <= 2 { return; }

        self.is_leaf = true;
        self.left = *pool_ptr;
        *pool_ptr += 2;

        if !self.partition_triangles(bvh, scene, pool, triangle_indices) { return; }

        let left: &BVHNode = &pool[self.left];
        let right: &BVHNode = &pool[self.left + 1];

        left.subdivide_node(bvh, scene, pool, triangle_indices, pool_ptr);
        right.subdivide_node(bvh, scene, pool, triangle_indices, pool_ptr);

        self.is_leaf = false;
    }

    fn find_best_split(self: &mut Self, bvh: &mut BVH) -> Option<usize> {
        let best_bin_index = None;
        let best_cost = std::f64::MAX;
        let cur_cost = self.bounds.surface_area() * self.count as f64;

        for i in 0..(bvh.bin_count - 1) {
            let bin_left = &bvh.bins_left[i];
            let bin_right = &bvh.bins_right[i];

            let cost = bin_left.bounds.surface_area() * bin_left.count as f64 + bin_right.bounds.surface_area() * bin_right.count as f64;

            if (cost < cur_cost) &&
               (bin_left.count > 0 && bin_right.count > 0) &&
               (best_bin_index == None || cost < best_cost) 
            {
                best_bin_index = Some(i);
                best_cost = cost;
            } 
        }

        return best_bin_index;
    }
    

    fn partition_triangles(self: &mut Self, bvh: &mut BVH, scene: &Vec<Box<dyn Primitive>>, pool: &Vec<BVHNode>, triangle_indices: &Vec<usize>) -> bool {
        let centroid_bounding_box = AABB::new();
        for i in self.first..(self.first + self.count) {
            let primitive = scene[triangle_indices[i]];
            centroid_bounding_box.grow_by_point(&primitive.get_centroid());
        }

        let l_axis = centroid_bounding_box.longest_axis();
        self.split_axis = l_axis as u32;

        bvh.reset_bins();

        let cbmin: f64 = centroid_bounding_box.bmin[l_axis];
        let cbmax: f64 = centroid_bounding_box.bmax[l_axis];

        if (cbmax - cbmin).abs() <= EPSILON { return false; }

        let k1 = (bvh.bin_count as f64 * (1.0 - EPSILON)) / (cbmax - cbmin);

        // Fill the bins with triangles
        for i in self.first..(self.first + self.count) {
            let primitive = scene[triangle_indices[i]];
            let ci = primitive.get_centroid()[l_axis];

            let bin_id = (k1 * (ci - cbmin)) as usize;

            let bin = &mut bvh.bins[bin_id];

            bin.count += 1;
            bin.bounds.grow_by_bounds(&primitive.get_bounds());
        }

        // Do a pass through the bins from the left
        bvh.bins_left[0] = bvh.bins[0];
        for i in 1..(bvh.bin_count - 1) {
            let bin = &bvh.bins[i];
            let bin_left = &mut bvh.bins_left[i];
            let prev_bin_left = &bvh.bins_left[i - 1];

            bin_left.count = prev_bin_left.count + bin.count;
            bin_left.bounds.grow_by_bounds(&prev_bin_left.bounds);
            bin_left.bounds.grow_by_bounds(&bin.bounds);
        }

        // Do a pass through the bins from the right
        bvh.bins_left[bvh.bin_count - 2] = bvh.bins[bvh.bin_count - 1];
        for i in (0..=(bvh.bin_count - 3)).rev() {
            let bin = &bvh.bins[i + 1];
            let bin_right = &mut bvh.bins_left[i];
            let prev_bin_right = &bvh.bins_left[i + 1];

            bin_right.count = prev_bin_right.count + bin.count;
            bin_right.bounds.grow_by_bounds(&prev_bin_right.bounds);
            bin_right.bounds.grow_by_bounds(&bin.bounds);
        }

        let best_bin_index = self.find_best_split(bvh);

        if best_bin_index == None { return false; }

        let Some(bin_index) = best_bin_index;

        // Quicksort triangle indices
        let j = self.first;
        for i in self.first..(self.first + self.count) {
            let primitive = scene[triangle_indices[i]];
            let ci = primitive.get_centroid()[l_axis];
            let bin_id = (k1 * (ci - cbmin)) as usize;

            if bin_id <= bin_index {
                let tmp = triangle_indices[i];
                triangle_indices[i] = triangle_indices[j];
                triangle_indices[j] = tmp; 
            }
        }

        let bin_left = &bvh.bins_left[bin_index];
        let bin_right = &bvh.bins_right[bin_index];

        let left_node = &mut bvh.pool[self.left];
        let right_node = &mut bvh.pool[self.left + 1];

        left_node.first = self.first;
        left_node.count = j - self.first;
        right_node.first = j;
        right_node.count = self.count - left_node.count;

        left_node.bounds = bin_left.bounds;
        right_node.bounds = bin_right.bounds;

        return true;
    }
}