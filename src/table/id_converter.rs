use std::string::FromUtf8Error;


pub fn base10_to_base26(mut number: usize) -> Result<String, FromUtf8Error> {
    let mut bytes = Vec::new();
    let n = number / 26;
    for _ in 0..=n {
        let remainder = number % 26;
        bytes.push(remainder as u8);
        number /= 26;
    }
    let b = bytes
        .iter()
        .rev()
        .map(|byte| *byte + 64)
        .collect::<Vec<u8>>();
    String::from_utf8(b)
}

pub fn base26_to_base10(letter: String) -> u32 {
    let base: u32 = 26;
    letter
        .as_bytes()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(mut index, mut byte)| {
            let letter = (byte - 64) as u32;
            (index, letter)
        })
        .fold(0, |accumulator, (index, letter)| {
            let b = letter * base.pow(index as u32);
            accumulator + b
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base10_to_base26() {
        let mut number = 3;
        assert_eq!("C", base10_to_base26(number).unwrap());
        number = 28;
        assert_eq!("AB", base10_to_base26(number).unwrap());
    }

    #[test]
    fn test_base26_to_base10() {
        let mut letter = String::from("C");
        assert_eq!(3, base26_to_base10(letter));
        letter = String::from("AB");
        assert_eq!(28, base26_to_base10(letter));
    }
}