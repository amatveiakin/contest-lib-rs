use std::collections::{HashSet, BTreeMap};
use std::vec;

use contest_lib_rs::binary_heaps::MinHeap;
use contest_lib_rs::bitset::Bitset;
use contest_lib_rs::io::prelude::*;
use contest_lib_rs::relax::RelaxMinMax;

fn parse_weekday(day: &str) -> usize {
    match day {
        "Monday" => 0,
        "Tuesday" => 1,
        "Wednesday" => 2,
        "Thursday" => 3,
        "Friday" => 4,
        "Saturday" => 5,
        "Sunday" => 6,
        _ => unreachable!(),
    }
}

fn get_next_workday(workdays: &Bitset, next_work_weekday: &mut [u64]) -> u64 {
    let mut day = u64::MAX;
    for weekday in 0..7 {
        if workdays.get(weekday) {
            day.relax_min(next_work_weekday[weekday]);
        }
    }
    day
}

#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(read: &mut Reader<R>, write: &mut W) {
    let [n, m, k] = read.usizes();
    let mut workdays = vec![];
    for _ in 0..n {
        let mut my_workdays = Bitset::new(7);
        let t = read.usize();
        for _ in 0..t {
            let day = read.word();
            my_workdays.set(parse_weekday(&day), true);
        }
        workdays.push(my_workdays);
    }
    let holidays = read.vec_u64(m).into_iter().map(|d| d - 1).collect::<HashSet<_>>();
    let mut projects_data = vec![];
    for _ in 0..k {
        let p = read.usize();
        let a = read.vec_u32(p).into_iter().map(|worker| worker - 1).collect::<Vec<_>>();
        projects_data.push(a);
    }

    let mut projects = projects_data.iter().map(|pr| &pr[..]).collect::<Vec<_>>();
    let mut queues = vec![MinHeap::new(); n];
    for (pid, &pr) in projects.iter().enumerate() {
        queues[pr[0] as usize].push(pid);
    }
    let mut next_work_weekday = vec![0u64; 7];
    for weekday in 0..7 {
        let mut day = weekday;
        while holidays.contains(&day) {
            day += 7;
        }
        next_work_weekday[weekday as usize] = day;
    }
    let mut active = BTreeMap::new();
    for worker in 0..n {
        if !queues[worker].is_empty() {
            let next_workday = get_next_workday(&workdays[worker], &mut next_work_weekday);
            active.entry(next_workday).or_insert(HashSet::new()).insert(worker);
        }
    }

    let mut project_finish = vec![None; k];
    while let Some((day, workers)) = active.pop_first() {
        for weekday in 0..7 {
            let mut d = next_work_weekday[weekday as usize];
            while d <= day {
                d += 7;
            }
            while holidays.contains(&d) {
                d += 7;
            }
            next_work_weekday[weekday as usize] = d;
        }

        assert!(next_work_weekday.iter().all(|&d| d >= day));
        let mut next_work = vec![];
        for worker in workers {
            let pid = queues[worker].pop().unwrap();
            let pr = &mut projects[pid];
            *pr = &pr[1..];
            if pr.is_empty() {
                project_finish[pid] = Some(day);
            } else {
                next_work.push((pid, pr[0] as usize));
                let next_worker = pr[0] as usize;
                let next_workday = get_next_workday(&workdays[next_worker], &mut next_work_weekday);
                active.entry(next_workday).or_insert(HashSet::new()).insert(next_worker);
            }
            if !queues[worker].is_empty() {
                let next_workday = get_next_workday(&workdays[worker], &mut next_work_weekday);
                active.entry(next_workday).or_insert(HashSet::new()).insert(worker);
            }
        }
        for (pid, worker) in next_work {
            queues[worker].push(pid);
        }
    }
    emitln!(write, project_finish.into_iter().map(|d| d.unwrap() + 1).collect::<Vec<_>>());
}

fn main() {
    let mut read = Reader::new(std::io::stdin().lock());
    let mut write = std::io::stdout().lock();
    solve(&mut read, &mut write);
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use pretty_assertions::assert_eq;
    use contest_lib_rs::testing::solution_testing::prelude::*;

    #[test]
    fn test() {
        assert_trimmed_eq!(&run_solver(solve, "\
        3 5 4
        2 Saturday Sunday
        2 Tuesday Thursday
        4 Monday Wednesday Friday Saturday
        4 7 13 14 15
        5 1 1 3 3 2
        3 2 3 2
        5 3 3 3 1 1
        8 3 3 3 3 3 3 3 3
        "), "25 9 27 27");
    }
}
