#![allow(dead_code)]
/// Функція для побудови сходинок (потрібна для тестів)
pub fn build_staircase(n: i32) -> String {
    let mut output = String::new();
    for i in 1..=n {
        let row = format!("{}{}",
                          " ".repeat((n - i) as usize),
                          "#".repeat(i as usize)
        );
        output.push_str(&row);
        if i < n {
            output.push('\n');
        }
    }
    output
}

/// Основна функція для HackerRank
pub fn staircase(n: i32) {
    println!("{}", build_staircase(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_6() {
        let result = build_staircase(6);
        let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
        assert_eq!(result, expected);
    }
}