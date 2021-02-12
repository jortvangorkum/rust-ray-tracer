use crate::{EPSILON, engine_objects::{Ray, primitives::Primitive}};

use super::{AABB, BVH};

#[derive(Clone, Copy, Default)]
pub struct BVHNode {
    pub bounds: AABB,
    pub is_leaf: bool,
    pub left: usize,
    pub first: usize,
    pub count: usize,
    pub split_axis: usize,
}

impl BVHNode {
    pub fn subdivide_node(self: &mut Self, bvh: &mut BVH, primitives: &Vec<Box<dyn Primitive>>) {
        if self.count <= 2 { return; }

        self.is_leaf = true;
        self.left = bvh.pool_ptr;
        bvh.pool_ptr += 2;

        if !self.partition_triangles(bvh, primitives) { return; }

        let mut left = bvh.pool[self.left];
        left.subdivide_node(bvh, primitives);
        bvh.pool[self.left] = left;
        
        let mut right = bvh.pool[self.left + 1];
        right.subdivide_node(bvh, primitives);
        bvh.pool[self.left + 1] = right;

        self.is_leaf = false;
    }

    fn find_best_split(self: &mut Self, bvh: &BVH) -> Option<usize> {
        let mut best_bin_index = None;
        let mut best_cost = std::f64::MAX;
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
    
    fn update_triangles_nodes(self: &mut Self, bvh: &mut BVH, primitives: &Vec<Box<dyn Primitive>>, bin_index: usize, l_axis: usize, k1: f64, cbmin: f64) {
        // Quicksort triangle indices
        let j = self.first;
        for i in self.first..(self.first + self.count) {
            let primitive = &primitives[bvh.triangle_indices[i]];
            let ci = primitive.get_centroid()[l_axis];
            let bin_id = (k1 * (ci - cbmin)) as usize;

            if bin_id <= bin_index {
                let tmp = bvh.triangle_indices[i];
                bvh.triangle_indices[i] = bvh.triangle_indices[j];
                bvh.triangle_indices[j] = tmp; 
            }
        }

        let bin_left = &bvh.bins_left[bin_index];
        let bin_right = &bvh.bins_right[bin_index];
        let left_node_count = j - self.first;

        let left_node = &mut bvh.pool[self.left];
        left_node.first = self.first;
        left_node.count = left_node_count;
        left_node.bounds = bin_left.bounds;
        
        let right_node = &mut bvh.pool[self.left + 1];
        right_node.first = j;
        right_node.count = self.count - left_node_count;
        right_node.bounds = bin_right.bounds;
    }

    fn partition_triangles(self: &mut Self, bvh: &mut BVH, primitives: &Vec<Box<dyn Primitive>>) -> bool {
        let mut centroid_bounding_box = AABB::new();
        for i in self.first..(self.first + self.count) {
            let primitive = &primitives[bvh.triangle_indices[i]];
            centroid_bounding_box.grow_by_point(&primitive.get_centroid());
        }

        let l_axis = centroid_bounding_box.longest_axis();
        self.split_axis = l_axis;

        bvh.reset_bins();

        let cbmin: f64 = centroid_bounding_box.bmin[l_axis];
        let cbmax: f64 = centroid_bounding_box.bmax[l_axis];

        if (cbmax - cbmin).abs() <= EPSILON { return false; }

        let k1 = (bvh.bin_count as f64 * (1.0 - EPSILON)) / (cbmax - cbmin);

        // Fill the bins with triangles
        for i in self.first..(self.first + self.count) {
            let primitive = &primitives[bvh.triangle_indices[i]];
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
            let prev_bin_left = &bvh.bins_left[i - 1];
            
            let mut bin_left = bvh.bins_left[i];
            bin_left.count = prev_bin_left.count + bin.count;
            bin_left.bounds.grow_by_bounds(&prev_bin_left.bounds);
            bin_left.bounds.grow_by_bounds(&bin.bounds);
            bvh.bins_left[i] = bin_left;
        }

        // Do a pass through the bins from the right
        bvh.bins_left[bvh.bin_count - 2] = bvh.bins[bvh.bin_count - 1];
        for i in (0..=(bvh.bin_count - 3)).rev() {
            let bin = &bvh.bins[i + 1];
            let prev_bin_right = &bvh.bins_left[i + 1];
            
            let mut bin_right = bvh.bins_left[i];
            bin_right.count = prev_bin_right.count + bin.count;
            bin_right.bounds.grow_by_bounds(&prev_bin_right.bounds);
            bin_right.bounds.grow_by_bounds(&bin.bounds);
            bvh.bins_right[i] = bin_right;
        }

        match self.find_best_split(bvh) {
            None => { return false; }
            Some(best_bin_index) => {
                self.update_triangles_nodes(bvh, primitives, best_bin_index, l_axis, k1, cbmin);
                return true;
            }
        };
    }

    pub fn update_bounds(self: &mut Self, primitives: &Vec<Box<dyn Primitive>>, triangle_indices: &Vec<usize>) {
        self.bounds = AABB::new();
        for triangle_index in triangle_indices {
            let triangle = &primitives[self.first + *triangle_index];
            self.bounds.grow_by_bounds(&triangle.get_bounds());
        }
    }

    fn intersect_triangles(self: &Self, ray: &Ray, primitives: &Vec<Box<dyn Primitive>>, triangle_indices: &Vec<usize>) -> Option<(usize, f64)> {
        let mut nearest_intersection: Option<(usize, f64)> = None;

        for i in 0..self.count {
            let primitive_index = triangle_indices[i];
            let primitive = &primitives[primitive_index];
            match primitive.intersect(ray) {
                None => { continue; }
                Some(distance) => {
                    if let Some((_, min_distance)) = nearest_intersection {
                        if distance < min_distance && distance > EPSILON {
                            nearest_intersection = Some((primitive_index, distance));
                        }
                    } else {
                        nearest_intersection = Some((primitive_index, distance));
                    }
                }
            }
        }

        return nearest_intersection;
    }

    pub fn traverse(self: &Self, ray: &Ray, bvh: &BVH, primitives: &Vec<Box<dyn Primitive>>) -> Option<(usize, f64)> {
        match ray.intersect_bounds(&self.bounds) {
            None => { return None; }
            Some(_) => {
                if self.is_leaf {
                    return self.intersect_triangles(ray, primitives, &bvh.triangle_indices);
                }

                let left = &bvh.pool[self.left];
                let right = &bvh.pool[self.left + 1];

                let ray_dir_axis = ray.direction[self.split_axis];

                if ray_dir_axis > 0.0 {
                    return left.traverse(ray, bvh, primitives).or_else(|| right.traverse(ray, bvh, primitives));
                } else {
                    return right.traverse(ray, bvh, primitives).or_else(|| left.traverse(ray, bvh, primitives));
                }
            }
        }
    }
}