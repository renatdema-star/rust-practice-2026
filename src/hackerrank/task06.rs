// https://www.hackerrank.com/challenges/kangaroo/problem
#![allow(dead_code)]

/// Функція перевіряє, чи зустрінуться два кенгуру в одній точці.
pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо другий кенгуру швидший або такий же, перший його ніколи не наздожене
    if v1 <= v2 {
        return String::from("NO");
    }

    // Перевіряємо, чи ділиться різниця відстаней на різницю швидкостей без остачі
    if (x2 - x1) % (v1 - v2) == 0 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_logic() {
        // Кенгуру зустрінуться
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
        // Кенгуру ніколи не зустрінуться
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }
}
