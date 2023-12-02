/// Interface for a Puzzle; includes a two-part solution..
/// All days should implement this trait.
pub trait Puzzle {
    fn name(&self) -> String;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
    fn input_path(&self) -> String;
    // return input.lines().collect::<Vec<&str>>().join("\n");
}
