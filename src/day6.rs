/*
Time:        40     82     84     92
Distance:   233   1011   1110   1487 */

pub fn execute_day_6_a() -> i32 {
    let race1 = calculate_ways_to_win(40, 233);
    let race2 = calculate_ways_to_win(82, 1011);
    let race3 = calculate_ways_to_win(84, 1110);
    let race4 = calculate_ways_to_win(92, 1487);
    return (race1 * race2 * race3 * race4);
}

pub fn execute_day_6_b() -> i32 {
    return calculate_ways_to_win(40828492, 233101111101487);
}

pub fn calculate_ways_to_win(time: usize, record_distance: usize) -> i32 {
    let mut ways_to_win = 0;
    for wait_time in 1..time - 1 {
        let distance = wait_time * (time - wait_time);
        if distance > record_distance {
            ways_to_win += 1;
        }
    }

    return ways_to_win;
}

mod tests {

    use super::*;

    #[test]
    fn execute_day_6_a_test() {
        let result = execute_day_6_a();
        assert_eq!(result, 4);
    }

    #[test]
    fn execute_day_6_b_test() {
        let result = execute_day_6_b();
        assert_eq!(result, 71503);
    }

    #[test]
    fn execute_day_6_a_test_mini() {
        assert_eq!(calculate_ways_to_win(7, 9), 4);
        assert_eq!(calculate_ways_to_win(15, 40), 8);
        assert_eq!(calculate_ways_to_win(30, 200), 9);
    }

    #[test]
    fn calculate_ways_to_win_test() {
        let result = calculate_ways_to_win(7, 9);
        assert_eq!(result, 4);
    }
}
