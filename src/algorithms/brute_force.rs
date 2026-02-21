use crate::{
    algorithms::common::{AlgResult, build_schedule, create_result},
    gantt_chart::draw_gantt,
};

fn generate_perms(n: usize, cur: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
    if cur.len() == n {
        res.push(cur.clone());
        return;
    }
    for i in 0..n {
        if !cur.contains(&i) {
            cur.push(i);
            generate_perms(n, cur, res);
            cur.pop();
        }
    }
}

pub fn brute_force(matrix: &Vec<Vec<i32>>) -> Result<(AlgResult, i32), String> {
    let n = matrix.len();
    if n > 10 {
        return Err("Слишком много задач (>10)".into());
    }

    let mut best_seq = (0..n).collect();
    let mut best_makespan = i32::MAX;

    let mut perms = Vec::new();
    generate_perms(n, &mut Vec::new(), &mut perms);

    for seq in perms {
        let (_, makespan, _) = build_schedule(matrix, &seq)?;
        if makespan < best_makespan {
            best_makespan = makespan;
            best_seq = seq;
        }
    }

    let orig_seq: Vec<usize> = (0..matrix.len()).collect();
    let orig_result = create_result(matrix, orig_seq, "Метод полного перебора (исходный)");

    let final_result = create_result(matrix, best_seq, "Метод полного перебора (финальный)");

    draw_gantt(&orig_result.clone()?, &matrix.clone(), "orig.svg");
    draw_gantt(&final_result.clone()?, &matrix.clone(), "final.svg");

    Ok((final_result.unwrap(), orig_result.unwrap().makespan))
}
