///Find the maximum number of meeting rooms given arrays of meeting start and end timings

pub fn max_rooms(start_times: &[&str], end_times: &[&str]) -> usize {
    if start_times.len() == 0 || end_times.len() == 0 || start_times.len() != end_times.len() {
        return 0;
    }
    let mut start_times = reformat_timings(start_times);
    start_times.sort();
    let mut end_times = reformat_timings(end_times);
    end_times.sort();
    let mut room_count = 0;
    let mut max_room_count = 0;
    let mut start = 0;
    let mut end = 0;
    while start < start_times.len() && end < end_times.len() {
        if start_times[start] <= end_times[end] {
            room_count += 1;
            max_room_count = std::cmp::max(max_room_count, room_count);
            start += 1;
        } else {
            room_count -= 1;
            end += 1;
        }
    }
    max_room_count
}

//Reformat given array of timing in the format &["9:00", "10:45"] -> vec![900, 1045]
fn reformat_timings(timings: &[&str]) -> Vec<usize> {
    let mut result = Vec::<usize>::with_capacity(timings.len());
    for timing in timings {
        let colon_index = timing.chars().position(|c| c == ':').unwrap();
        let pre_colon = &timing[..colon_index];
        let post_colon = &timing[colon_index + 1..];
        let timing_reformatted = String::from(pre_colon) + post_colon;
        let timing_as_usize = timing_reformatted.parse::<usize>().unwrap();
        result.push(timing_as_usize);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::max_rooms;
    use super::reformat_timings;
    #[test]
    fn test_reformat_timings() {
        let timings = ["9:00", "10:45"];
        assert_eq!(reformat_timings(&timings), vec![900, 1045]);
    }
    #[test]
    fn test_max_rooms() {
        let start_times = ["9:00", "9:40", "9:50", "11:00", "15:00", "18:00"];
        let end_times = ["9:10", "12:00", "11:20", "11:30", "19:00", "20:00"];
        assert_eq!(max_rooms(&start_times, &end_times), 3);
    }
}
