pub trait Problem {
    fn solve_part_one(&self, input: &str) -> String;
    fn solve_part_two(&self, input: &str) -> String;
    fn index(&self) -> usize;
    fn name(&self) -> String;
}
