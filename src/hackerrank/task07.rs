// https://www.hackerrank.com/challenges/between-two-sets/problem
#![allow(dead_code)]

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    // Знаходимо діапазон для перевірки: від найбільшого в a до найменшого в b
    let max_a = *a.iter().max().unwrap_or(&1);
    let min_b = *b.iter().min().unwrap_or(&100);

    let mut count = 0;

    // Перебираємо всі числа в цьому діапазоні
    for x in max_a..=min_b {
        // Перевіряємо обидві умови через ітератори
        let is_multiple_of_a = a.iter().all(|&num| x % num == 0);
        let is_factor_of_b = b.iter().all(|&num| num % x == 0);

        if is_multiple_of_a && is_factor_of_b {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        // Очікуємо 3 числа: 4, 8, 16
        assert_eq!(get_total_x(&a, &b), 3);
    }
}