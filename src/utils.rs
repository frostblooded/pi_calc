pub fn range_to_subranges(n: u64, wanted_tasks_count: u64) -> (Vec<u64>, Vec<u64>) {
    let iters_per_task = ((n as f64) / (wanted_tasks_count as f64)).ceil() as u64;
    let remaining_iters = n % wanted_tasks_count;
    let tasks_count = n / iters_per_task + 1;

    // We are excluding the iters for the last task, because they
    // are added additionally as remainder.
    let iter_range: Vec<_> = (0..(tasks_count - 2)).collect();

    let mut start_indexes: Vec<u64> = iter_range.iter().map(|i| i * iters_per_task).collect();
    start_indexes.push(n - iters_per_task - remaining_iters);

    let mut end_indexes: Vec<u64> = iter_range
        .iter()
        .map(|i| (i + 1) * iters_per_task - 1)
        .collect();
    end_indexes.push(n - 1);

    (start_indexes, end_indexes)
}
