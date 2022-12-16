#[macro_export]
macro_rules! define_main {
    ($day:ident) => {
        fn main() {
            advent_of_code::run::<advent_of_code::$day::Solver>();
        }
    };
}

#[macro_export]
macro_rules! define_integration_test {
    ($day:ident, $number:literal, $answer_1:expr $( => $attr_1:meta)* $(, $answer_2:expr $( => $attr_2:meta)*)?) => {
        use advent_of_code::$day::Solver;

        fn get_input() -> String {
            std::fs::read_to_string(format!("input/day/{}/input", $number)).unwrap()
        }

        #[test]
        $(#[$attr_1])*
        fn part_1() {
            let answer = advent_of_code::solve_part_1::<Solver>(get_input()).unwrap();
            assert_eq!(answer, $answer_1);
        }
        $(

        #[test]
        $(#[$attr_2])*
        fn part_2() {
            let answer = advent_of_code::solve_part_2::<Solver>(get_input()).unwrap();
            assert_eq!(answer, $answer_2);
        })?
    };
}
