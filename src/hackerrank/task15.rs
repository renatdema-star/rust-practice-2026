pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);

    if from_front < from_back {
        from_front
    } else {
        from_back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count_6_2() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_page_count_5_3() {
        assert_eq!(page_count(5, 3), 1);
    }

    #[test]
    fn test_page_count_6_5() {
        assert_eq!(page_count(6, 5), 1);
    }
}