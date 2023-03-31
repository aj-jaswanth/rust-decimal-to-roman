pub fn decimal_to_roman(number: u64) -> String {
    let conversion = [
        (1000, 'M'),
        (500, 'D'),
        (100, 'C'),
        (50, 'L'),
        (10, 'X'),
        (5, 'V'),
        (1, 'I'),
    ];

    let mut number = number;
    let mut roman_literal = String::new();

    while number > 0 {
        for i in 0..conversion.len() {
            let (val_i, lit_i) = conversion[i];
            if number >= val_i {
                roman_literal.push(lit_i);
                number -= val_i;
                break;
            } else {
                for j in i + 1..conversion.len() {
                    let (val_j, lit_j) = conversion[j];
                    let diff = val_i - val_j;
                    if diff <= number && diff != val_j {
                        roman_literal.push(lit_j);
                        roman_literal.push(lit_i);
                        number -= diff;
                        break;
                    }
                }
            }
        }
    }
    roman_literal
}

#[cfg(test)]
mod test {
    use crate::roman_numerals::decimal_to_roman;

    #[test]
    fn converts_a_decimal_number_to_roman_literals() {
        assert_eq!(decimal_to_roman(1), "I");
        assert_eq!(decimal_to_roman(2), "II");
        assert_eq!(decimal_to_roman(3), "III");
        assert_eq!(decimal_to_roman(4), "IV");
        assert_eq!(decimal_to_roman(5), "V");
        assert_eq!(decimal_to_roman(6), "VI");
        assert_eq!(decimal_to_roman(7), "VII");
        assert_eq!(decimal_to_roman(8), "VIII");
        assert_eq!(decimal_to_roman(9), "IX");
        assert_eq!(decimal_to_roman(10), "X");
        assert_eq!(decimal_to_roman(24), "XXIV");
        assert_eq!(decimal_to_roman(31), "XXXI");
        assert_eq!(decimal_to_roman(40), "XL");
        assert_eq!(decimal_to_roman(50), "L");
        assert_eq!(decimal_to_roman(60), "LX");
        assert_eq!(decimal_to_roman(70), "LXX");
        assert_eq!(decimal_to_roman(80), "LXXX");
        assert_eq!(decimal_to_roman(90), "XC");
        assert_eq!(decimal_to_roman(100), "C");
        assert_eq!(decimal_to_roman(369), "CCCLXIX");
        assert_eq!(decimal_to_roman(400), "CD");
        assert_eq!(decimal_to_roman(448), "CDXLVIII");
        assert_eq!(decimal_to_roman(500), "D");
        assert_eq!(decimal_to_roman(950), "CML");
        assert_eq!(decimal_to_roman(1000), "M");
        assert_eq!(decimal_to_roman(1994), "MCMXCIV");
        assert_eq!(decimal_to_roman(1998), "MCMXCVIII");
        assert_eq!(decimal_to_roman(2751), "MMDCCLI");
    }
}
