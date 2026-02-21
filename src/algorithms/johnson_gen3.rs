use crate::{
    algorithms::common::{AlgResult, create_result},
    gantt_chart::draw_gantt,
};

pub fn johnson_gen3(matrix: &Vec<Vec<i32>>) -> Result<(AlgResult, i32), String> {
    let mut jobs: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let (max_idx, _) = r.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
            (i, max_idx)
        })
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    let orig_seq: Vec<usize> = (0..matrix.len()).collect();
    let orig_result = create_result(
        matrix,
        orig_seq,
        "Джонсон приоритет «узкого места» (исходный)",
    );

    let final_result = create_result(
        matrix,
        sequence,
        "Джонсон приоритет «узкого места» (финальный)",
    );

    draw_gantt(&orig_result.clone()?, &matrix.clone(), "orig.svg");
    draw_gantt(&final_result.clone()?, &matrix.clone(), "final.svg");

    Ok((final_result.unwrap(), orig_result.unwrap().makespan))
}
