// https://www.hackerrank.com/challenges/apple-and-orange/problem
#![allow(dead_code)]

pub fn count_fruits(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (usize, usize) {
    let apples_on_house = apples.iter()
        .filter(|&&d| s <= a + d && a + d <= t)
        .count();

    let oranges_on_house = oranges.iter()
        .filter(|&&d| s <= b + d && b + d <= t)
        .count();

    (apples_on_house, oranges_on_house)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apple_and_orange_logic() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        // Очікуємо, що на будинок впаде 1 яблуко та 1 апельсин
        assert_eq!(count_fruits(s, t, a, b, &apples, &oranges), (1, 1));
    }
}