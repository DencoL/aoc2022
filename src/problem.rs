pub trait Problem {
    fn solve_part_one(&self, input: &str) -> usize;
    fn solve_part_two(&self, input: &str) -> usize;
    fn index(&self) -> usize;
}
