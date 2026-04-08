// Given an array of positive integers, return the number of elements that are strictly greater than the average of all previous elements. Skip the first element.

// fn countResponseTimeRegressions(responseTimes: &[i32]) -> i32 {
fn count_response_time_regressions(response_times: &[i32]) -> i32 {
    if response_times.len() < 2 {
        return 0;
    }

    let mut ans = 0;
    let mut sum = response_times[0] as u64;
    for idx in 1..response_times.len() {
        if response_times[idx] as u64 > sum / idx as u64 {
            ans += 1;
        }
        sum += response_times[idx] as u64;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let response_times = [100, 200, 150, 300];
        assert_eq!(count_response_time_regressions(&response_times), 2);
    }

    #[test]
    fn test_2() {
        let response_times = [100, 200, 150, 300];
        assert_eq!(count_response_time_regressions(&response_times), 2);
    }

    #[test]
    fn test_short_inputs() {
        let empty: [i32; 0] = [];
        let single = [42];
        assert_eq!(count_response_time_regressions(&empty), 0);
        assert_eq!(count_response_time_regressions(&single), 0);
    }

    #[test]
    fn test_large_values_no_overflow() {
        let response_times = [i32::MAX, i32::MAX, i32::MAX];
        assert_eq!(count_response_time_regressions(&response_times), 0);
    }
}
