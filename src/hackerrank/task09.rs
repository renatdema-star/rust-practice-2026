pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut max_id = 0;

    for id in 1..=5 {
        if counts[id] > max_count {
            max_count = counts[id];
            max_id = id as i32;
        }
    }

    max_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_example_1() {
        let arr = vec![1, 1, 2, 2, 3];
        assert_eq!(migratory_birds(&arr), 1);
    }

    #[test]
    fn test_migratory_birds_example_2() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_example_3() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }
}