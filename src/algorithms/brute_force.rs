use crate::algorithms::common::{AlgResult, build_schedule, create_result};

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

pub fn brute_force(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
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

    create_result(matrix, best_seq, "Метод полного перебора")
}
