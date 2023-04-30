use num_format::{Locale, ToFormattedString};

pub fn format_token_amount(amount: u64, decimals: u32, round_to: usize) -> String {
    let divisor = 10u64.pow(decimals);
    let whole_part = amount / divisor;
    let frac_part = amount % divisor;

    let rounding_factor = 10u64.pow(decimals - round_to as u32);
    let rounded_frac_part = (frac_part + rounding_factor / 2) / rounding_factor;

    let frac_str = format!("{:0width$}", rounded_frac_part, width = round_to);
    let frac_str = frac_str.trim_end_matches('0');

    if frac_str.is_empty() {
        format!("{}.0", whole_part)
    } else {
        format!("{}.{}", whole_part, frac_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_token_amount() {
        let token_a: u64 = 1_000_000_000_000_000_000;
        let token_b: u64 = 8_889_842_822_521_928;
        let decimals: u32 = 18;
        let round_to: usize = 5;

        let formatted_a = format_token_amount(token_a, decimals, round_to);
        let formatted_b = format_token_amount(token_b, decimals, round_to);

        assert_eq!(formatted_a, "1.0");
        assert_eq!(formatted_b, "0.00889");
    }
}
