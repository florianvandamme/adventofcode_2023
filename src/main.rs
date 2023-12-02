mod day1;
mod day2;

fn main() {
    use day1::solve_part_1 as solve_day_1_part_1;
    use day1::solve_part_2 as solve_day_1_part_2;
    use day2::solve_part_1 as solve_day_2_part_1;
    use day2::solve_part_2 as solve_day_2_part_2;

    solve_day_1_part_1();
    solve_day_1_part_2();
    solve_day_2_part_1();
    solve_day_2_part_2();
}
