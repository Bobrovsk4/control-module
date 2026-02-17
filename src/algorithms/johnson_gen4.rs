fn johnson_gen4(matrix: &Vec<Vec<i32>>) -> Result<Vec<usize>, String> {
    let mut jobs: Vec<(usize, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r.iter().sum()))
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    Ok(jobs.into_iter().map(|(i, _)| i).collect())
}
