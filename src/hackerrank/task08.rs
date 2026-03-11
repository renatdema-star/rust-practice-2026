#![allow(dead_code)]

pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut max_s = scores[0];
    let mut min_s = scores[0];
    let mut max_c = 0;
    let mut min_c = 0;

    for &s in scores.iter().skip(1) {
        if s > max_s {
            max_s = s;
            max_c += 1;
        } else if s < min_s {
            min_s = s;
            min_c += 1;
        }
    }

    vec![max_c, min_c]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }
}