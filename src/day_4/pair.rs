pub struct Pair {
    start: u32,
    end: u32
}

impl Pair {
    pub fn new(pair: &str) -> Pair {
        let pair_boundaries: Vec<u32> = pair.split("-").map(|v| v.parse().unwrap()).collect();

        return Pair {
            start: pair_boundaries[0],
            end: pair_boundaries[1]
        }
    }

    pub fn is_inside_pair(&self, other_pair: &Pair) -> bool {
        return self.start >= other_pair.start && self.end <= other_pair.end;
    }

    pub fn overlaps_with_pair(&self, other_pair: &Pair) -> bool {
        return self.end >= other_pair.start && self.start <= other_pair.end;
    }
}
