/// Utility functions used across the project.

/// Calculate the Sørensen–Dice coefficient for two strings.
/// Returns a value between 0.0 (no similarity) and 1.0 (identical).
pub fn dice_coefficient(input: &str, compare_to: &str) -> f64 {
    let ibg = bi_grams(input);
    let cbg = bi_grams(compare_to);
    let matches = ibg.iter().filter(|bg| cbg.contains(bg)).count();
    if ibg.is_empty() && cbg.is_empty() {
        1.0
    } else {
        2.0 * matches as f64 / (ibg.len() + cbg.len()) as f64
    }
}

fn bi_grams(input: &str) -> Vec<String> {
    let mut s = String::with_capacity(input.len() + 2);
    s.push('$');
    s.push_str(input);
    s.push('%');
    let chars: Vec<char> = s.chars().collect();
    let mut bg = Vec::with_capacity(chars.len().saturating_sub(1));
    for i in 0..chars.len().saturating_sub(1) {
        bg.push(format!("{}{}", chars[i], chars[i + 1]));
    }
    bg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_strings() {
        assert!((dice_coefficient("abc", "abc") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn partial_match() {
        let coeff = dice_coefficient("abc", "abd");
        assert!((coeff - 0.5).abs() < f64::EPSILON);
    }
}
