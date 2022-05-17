// An unsigened integer identifier of variable length, heap allocated.
#[derive(Debug, PartialEq)]
pub struct Identifier(Vec<u8>);

#[derive(Debug, PartialEq)]
pub enum IdentifierError {
    InvalidValue(String),
}

impl Identifier {
    // Create the identifier from a decimal integer string.
    pub fn from_decimal(number: &str) -> Result<Identifier, IdentifierError> {
        let number_str = str::replace(number, ',', ""); // Clean string - Remove all commas.

        let is_number = number_str.chars().all(char::is_numeric);
        if is_number == false {
            return Err(IdentifierError::InvalidValue(String::from(
                "The input string is invalid.",
            )));
        }

        let number_bytes = number_str.into_bytes();

        let mut number_vector = Vec::new(); // The number as a vector of u8 values.

        //let mut new_id = Identifier(Vec::new()); // The Identifier object to be returned.
        let mut temp: u128 = 0;
        //let mut id_arr_idx: usize = 19; // The index of the array to update.
        let ascii_base_codepoint = 48; // The base ASCII codepoint for numbers (i.e., 0x30 since
                                       // number = 0x30 + number).

        for (idx, x) in number_bytes.iter().rev().enumerate() {
            // Iterate through the byte array to build the Id.
            let current_val = ((x - ascii_base_codepoint) as u32) * ((10 as u32).pow(idx as u32));
            temp += current_val as u128;

            while temp > 255 {
                if temp < 256 {
                    number_vector.push(temp as u8);
                    temp = 0;
                } else {
                    number_vector.push(255 as u8);
                    temp -= 255;
                }
            }
        }

        if temp > 0 {
            // Add remainder.
            number_vector.push(temp as u8);
        }

        // The u8 values were added right to left. Reverse it.
        number_vector.reverse();
        Ok(Identifier(number_vector))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_decimal_invalid_input() {
        assert_eq!(
            Identifier::from_decimal("1234567890J"),
            Err(IdentifierError::InvalidValue(String::from(
                "The input string is invalid."
            )))
        );
    }

    #[test]
    fn from_decimal_success() {
        assert_eq!(
            Identifier::from_decimal("510"),
            Ok(Identifier(vec![255 as u8, 255 as u8]))
        );
    }

    #[test]
    fn from_decimal_success_with_remainder() {
        assert_eq!(
            Identifier::from_decimal("530"),
            Ok(Identifier(vec![20 as u8, 255 as u8, 255 as u8]))
        );
    }
}
