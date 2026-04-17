// Given an array of intervals where each interval has a start and end time, return the maximum number of non-overlapping intervals.

// fn maximizeNonOverlappingMeetings(meetings: &[Vec<i32>]) -> i32 {
fn maximize_non_overlapping_meetings(meetings: & [Vec<i32>]) -> i32 {
    let mut meetings = meetings.to_vec();
    meetings.sort_by_key(|x| x[1]);
    let mut count = 0;
    let mut end = std::i32::MIN;
    for meeting in meetings {
        if meeting[0] >= end {
            count += 1;
            end = meeting[1];
        }
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let meetings = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        assert_eq!(maximize_non_overlapping_meetings(&meetings), 3);
    }
    #[test]
    fn test_2() {        let meetings = vec![
            vec![0, 5],
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![3, 5],
            vec![4, 6],
        ];
        assert_eq!(maximize_non_overlapping_meetings(&meetings), 4);
    }
}
