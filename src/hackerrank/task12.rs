pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    let mut max_height = i32::MIN;
    let mut count = 0;

    for &height in candles {
        if height > max_height {
            max_height = height;
            count = 1;
        } else if height == max_height {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_cake_candles_example() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_birthday_cake_candles_all_same() {
        let candles = vec![4, 4, 4, 4];
        assert_eq!(birthday_cake_candles(&candles), 4);
    }

    #[test]
    fn test_birthday_cake_candles_single() {
        let candles = vec![5];
        assert_eq!(birthday_cake_candles(&candles), 1);
    }
}