/// Disjoint union find with rank heuristic and path compression.
/// Uses negative values to count and store the rank.
#[derive(Debug)]
pub struct DisjointUnionFind {
    parent: Vec<i32>,
    num_sets: usize,
}

impl DisjointUnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parent: vec![-1; size],
            num_sets: size,
        }
    }

    pub fn join(&mut self, a: usize, b: usize) -> () {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return;
        }

        let rank_a = -self.parent[a];
        let rank_b = -self.parent[b];

        if rank_a < rank_b {
            self.parent[b] -= rank_a;
            self.parent[a] = b as i32;
        } else {
            self.parent[a] -= rank_b;
            self.parent[b] = a as i32;
        }

        self.num_sets -= 1;
    }

    pub fn find(&mut self, a: usize) -> usize {
        if self.parent[a] >= 0 {
            self.parent[a] = self.find(self.parent[a] as usize) as i32;
            self.parent[a] as usize
        } else {
            a
        }
    }

    pub fn get_size(&self) -> usize {
        self.num_sets
    }

    pub fn is_head(&self, a: &usize) -> bool {
        self.parent[*a] < 0
    }
}