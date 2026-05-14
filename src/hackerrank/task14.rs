pub fn bon_appetit(bill: &[i32], k: i32, b: i32) -> String {
    let actual_share = (bill.iter().sum::<i32>() - bill[k as usize]) / 2;

    if b == actual_share {
        "Bon Appetit".to_string()
    } else {
        (b - actual_share).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bon_appetit_fair() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 7), "Bon Appetit");
    }

    #[test]
    fn test_bon_appetit_overcharged() {
        let bill = vec![3, 10, 2, 9];
        assert_eq!(bon_appetit(&bill, 1, 12), "5");
    }
}