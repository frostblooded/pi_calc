use std::ops::Range;

pub fn split_range_to_count(range: &Range<u64>, subrange_count: u64) -> Vec<Range<u64>> {
    if subrange_count == 0 {
        return vec![];
    }

    let range_length = range.end - range.start;
    let items_per_subrange = range_length / subrange_count;
    let mut remainder = range_length % subrange_count;
    let mut result = Vec::with_capacity(subrange_count as usize);
    let mut current_index = range.start;

    while current_index < range.end {
        let subrange_start = current_index;
        let mut subrange_end = current_index + items_per_subrange;

        if remainder > 0 {
            subrange_end += 1;
            remainder -= 1;
        }

        result.push(subrange_start..subrange_end);
        current_index = subrange_end;
    }

    result
}

pub fn split_range_for_threading(range: &Range<u64>, thread_count: u64) -> Vec<Vec<Range<u64>>> {
    let split_ranges = split_range_to_count(range, thread_count * 2);
    let mut result: Vec<Vec<Range<u64>>> = vec![];

    for i in 0..(split_ranges.len() / 2) {
        result.push(vec![
            split_ranges[i].clone(),
            split_ranges[split_ranges.len() - i - 1].clone(),
        ]);
    }

    result
}

#[cfg(test)]
mod test_split_range_to_count {
    use super::*;

    #[test]
    fn returns_same_if_count_is_one() {
        let range: Range<u64> = 0..100;
        assert_eq!(split_range_to_count(&range, 1), vec![range]);
    }

    #[test]
    fn returns_split_range() {
        let range: Range<u64> = 0..100;
        let expected_split: Vec<Range<u64>> = vec![0..20, 20..40, 40..60, 60..80, 80..100];
        assert_eq!(split_range_to_count(&range, 5), expected_split);
    }

    #[test]
    fn last_subrange_inclues_remainder() {
        let range: Range<u64> = 0..20;
        let expected_split: Vec<Range<u64>> = vec![0..3, 3..6, 6..9, 9..12, 12..15, 15..18, 18..20];
        assert_eq!(split_range_to_count(&range, 7), expected_split);
    }

    #[test]
    fn last_subrange_inclues_remainder2() {
        let range: Range<u64> = 0..13;
        let expected_split: Vec<Range<u64>> =
            vec![0..2, 2..4, 4..6, 6..8, 8..10, 10..11, 11..12, 12..13];
        assert_eq!(split_range_to_count(&range, 8), expected_split);
    }

    #[test]
    fn handles_range_smaller_than_count() {
        let range: Range<u64> = 0..3;
        let expected_split: Vec<Range<u64>> = vec![0..1, 1..2, 2..3];
        assert_eq!(split_range_to_count(&range, 10), expected_split);
    }

    #[test]
    fn handles_empty_ranges() {
        let range: Range<u64> = 0..0;
        let expected_split: Vec<Range<u64>> = vec![];
        assert_eq!(split_range_to_count(&range, 20), expected_split);
    }

    #[test]
    fn handles_request_for_zero_subranges() {
        let range: Range<u64> = 0..20;
        let expected_split: Vec<Range<u64>> = vec![];
        assert_eq!(split_range_to_count(&range, 0), expected_split);
    }
}

#[cfg(test)]
mod test_split_range_for_threading {
    use super::*;

    #[test]
    fn returns_correctly() {
        let range: Range<u64> = 0..16;
        let expected_split: Vec<Vec<Range<u64>>> = vec![
            vec![0..2, 14..16],
            vec![2..4, 12..14],
            vec![4..6, 10..12],
            vec![6..8, 8..10],
        ];
        assert_eq!(split_range_for_threading(&range, 4), expected_split);
    }
}
