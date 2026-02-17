fn johnson_gen3(matrix: &Vec<Vec<i32>>) -> Result<Vec<usize>, String> {
    let mut jobs: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let (max_idx, _) = r.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
            (i, max_idx)
        })
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    Ok(jobs.into_iter().map(|(i, _)| i).collect())
}
