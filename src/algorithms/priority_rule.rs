use crate::{
    algorithms::common::{AlgResult, create_result},
    gantt_chart::draw_gantt,
};

pub fn priority_rule(matrix: &Vec<Vec<i32>>) -> Result<(AlgResult, i32), String> {
    if matrix[0].len() != 2 {
        return Err("Нужно 2 станка".into());
    }
    let max_val = matrix.iter().flatten().copied().max().unwrap_or(1);
    let mut jobs: Vec<(usize, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let (a, b) = (r[0], r[1]);
            let sign = if a < b { 1 } else { -1 };
            let p = sign * (max_val - a.min(b));
            (i, p)
        })
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    let orig_seq: Vec<usize> = (0..matrix.len()).collect();
    let orig_result = create_result(matrix, orig_seq, "Метод приоритетов (исходный)");

    let final_result = create_result(matrix, sequence, "Метод приоритетов (финальный)");

    draw_gantt(&orig_result.clone()?, &matrix.clone(), "orig.svg");
    draw_gantt(&final_result.clone()?, &matrix.clone(), "final.svg");

    Ok((final_result.unwrap(), orig_result.unwrap().makespan))
}
