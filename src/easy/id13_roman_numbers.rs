use std::collections::HashMap;

struct RomanNumbers;

impl RomanNumbers {
    fn from_roman_to_int32(rn: String) -> i32 {
        let v = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ];

        let iter: std::vec::IntoIter<(char, i32)> = v.into_iter();

        let hm: HashMap<char, i32> = iter.collect();

        let mut sum = 0;
        let mut last = 0;

        for c in rn.chars() {
            if let Some(&v) = hm.get(&c) {
                if v > last {
                    sum += v - last - last;
                } else {
                    sum += v
                }
                last = v
            }
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(RomanNumbers::from_roman_to_int32(String::from("III")), 3);
    assert_eq!(RomanNumbers::from_roman_to_int32(String::from("IV")), 4);
    assert_eq!(RomanNumbers::from_roman_to_int32(String::from("IX")), 9);
    assert_eq!(RomanNumbers::from_roman_to_int32(String::from("LVIII")), 58);
    assert_eq!(
        RomanNumbers::from_roman_to_int32(String::from("MCMXCIV")),
        1994
    );
}
