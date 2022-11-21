pub trait Problem {
    fn solve_part_one(&self, input: &str) -> u64;
    fn solve_part_two(&self, input: &str) -> u64;
    fn index(&self) -> usize;
}
