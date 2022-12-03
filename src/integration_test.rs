#[macro_export]
macro_rules! define_integration_test {
    ($day:ident, $number:literal, $answer_1:literal$(, $answer_2:literal)?) => {
        use std::fs::File;

        use advent_of_code::$day::Solver;

        fn get_file() -> File {
            File::open(format!("input/day/{}/input", $number)).unwrap()
        }

        #[test]
        fn part_1() {
            let answer = advent_of_code::solve_part_1::<Solver, _>(get_file()).unwrap();
            assert_eq!(answer, $answer_1);
        }
        $(

        #[test]
        fn part_2() {
            let answer = advent_of_code::solve_part_2::<Solver, _>(get_file()).unwrap();
            assert_eq!(answer, $answer_2);
        })?
    };
}
