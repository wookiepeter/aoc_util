use std::ops::{Range, RangeBounds};

pub fn combine_ranges<T>(ranges: &Vec<Range<T>>) -> Vec<Range<T>>
where
    T: Copy + Clone + Ord,
{
    let mut ranges = ranges.clone();
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = vec![];

    let mut current_range = ranges[0].clone();

    for range in &ranges[1..] {
        if range.start <= current_range.end {
            // Ranges overlap, so we extend the current range to include the new range
            current_range.end = current_range.end.max(range.end);
        } else {
            // Ranges do not overlap, so we push the current range to the result and start a new current range
            result.push(current_range);
            current_range = range.clone();
        }
    }

    result.push(current_range);
    result
}

pub fn ranges_overlap<T>(a: Range<T>, b: Range<T>) -> bool
where
    T: PartialOrd,
{
    a.contains(&b.start) || a.contains(&b.end) || b.contains(&a.start) || b.contains(&a.end)
}
