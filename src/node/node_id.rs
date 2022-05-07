// The Node ID object.
// Represents a 160-bit number.
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct NodeId([u8; 20]);

#[derive(Debug, PartialEq)]
pub enum IdError {
    ValueOutOfRange(String),
    InvalidValue(String),
}

const MIN: NodeId = NodeId([0; 20]);
const MAX: NodeId = NodeId([255; 20]);

fn remove_hex_header(hex_string: &str) -> &str {
    // Remove the hex string header (i.e., 0x).
    let mut characters = hex_string.chars();
    characters.next();
    characters.next();
    characters.as_str()
}

impl NodeId {
    // Builds an Id based on a base-10 number representation.
    pub fn from_number(number: &str) -> Result<NodeId, IdError> {
        let number_str = str::replace(number, ',', ""); // Clean string. Remove all commas. The
                                                        // string is expected to represent a whole
                                                        // number value.
        let is_number = number_str.chars().all(char::is_numeric);
        if is_number == false {
            return Err(IdError::InvalidValue(String::from(
                "The input string is invalid.",
            )));
        }

        let number_bytes = number_str.into_bytes();

        let mut new_id = NodeId([0; 20]); // The node ID object to be returned.
        let mut temp: u128 = 0;
        let mut id_arr_idx: usize = 19; // The index of the array to update.
        let ascii_base_codepoint = 48; // The base ASCII codepoint for numbers (i.e., 0x30 since
                                       // number = 0x30 + number).

        for (idx, x) in number_bytes.iter().rev().enumerate() {
            // Iterate through the byte array to build the Id.
            let current_val = ((x - ascii_base_codepoint) as u32) * ((10 as u32).pow(idx as u32));
            temp += current_val as u128;

            while temp > 255 {
                if temp < 256 {
                    new_id.0[id_arr_idx] = temp as u8;
                    temp = 0;
                } else {
                    new_id.0[id_arr_idx] = 255;
                    temp -= 255;
                }

                if temp > 0 && id_arr_idx == 0 {
                    return Err(IdError::ValueOutOfRange(String::from(
                        "The value is to large!",
                    )));
                }

                id_arr_idx -= 1;
            }
        }

        if temp > 0 {
            new_id.0[id_arr_idx] = temp as u8;
        }

        Ok(new_id)
    }

    // Builds an Id based on a hexadecimal number representation (i.e., 0x####).
    pub fn from_hex(hex_number: &str) -> Result<NodeId, IdError> {
        let re = Regex::new("^0x[0-9A-Fa-f]+$").unwrap();
        let number_string;

        if re.is_match(hex_number) == true {
            number_string = remove_hex_header(hex_number);
        } else {
            // Invalid hex string.
            return Err(IdError::InvalidValue(String::from(
                "The input is not a valid hex string.",
            )));
        }

        let number_bytes = number_string.to_string().into_bytes();

        let mut new_id = NodeId([0; 20]); // The node ID object to be returned.
        let mut temp: u128 = 0;
        let mut id_arr_idx: usize = 19; // The index of the array to update.
        let ascii_num_base_codepoint = 48; // The base ASCII codepoint for numbers (i.e., 0x30 since
                                           // number = 0x30 + number).
        let ascii_cap_letter_base_codepoint = 65; // The base ASCII codepoint for capital letters
                                                  // (i.e., 0x41 since letter = 0x41 + letter offset).
        let ascii_letter_base_codepoint = 97; // The base ASCII codepoint for lowercase letters
                                              // (i.e., 0x61 since letter = 0x61 + letter offset).

        for (idx, x) in number_bytes.iter().rev().enumerate() {
            // Iterate through the byte array to build the Id.
            let current_val;

            if x >= &ascii_letter_base_codepoint {
                current_val =
                    ((x - ascii_letter_base_codepoint + 10) as u32) * ((16 as u32).pow(idx as u32));
            } else if x >= &ascii_cap_letter_base_codepoint {
                current_val = ((x - ascii_cap_letter_base_codepoint + 10) as u32)
                    * ((16 as u32).pow(idx as u32));
            } else {
                current_val =
                    ((x - ascii_num_base_codepoint) as u32) * ((16 as u32).pow(idx as u32));
            }
            temp += current_val as u128;

            while temp > 255 {
                if temp < 256 {
                    new_id.0[id_arr_idx] = temp as u8;
                    temp = 0;
                } else {
                    new_id.0[id_arr_idx] = 255;
                    temp -= 255;
                }

                if temp > 0 && id_arr_idx == 0 {
                    return Err(IdError::ValueOutOfRange(String::from(
                        "The value is to large!",
                    )));
                }

                id_arr_idx -= 1;
            }
        }

        if temp > 0 {
            new_id.0[id_arr_idx] = temp as u8;
        }

        Ok(new_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_number_invalid_input() {
        assert_eq!(
            NodeId::from_number("1234567890J"),
            Err(IdError::InvalidValue(String::from(
                "The input string is invalid."
            )))
        );
    }

    #[test]
    fn from_number_value_too_large() {
        assert_eq!(
            NodeId::from_number("1234567890"),
            Err(IdError::ValueOutOfRange(String::from(
                "The value is to large!",
            )))
        );
    }

    #[test]
    fn from_number_success() {
        assert_eq!(
            NodeId::from_number("510"),
            Ok(NodeId([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255
            ]))
        );
    }

    #[test]
    fn from_number_success_partial_remaining() {
        assert_eq!(
            NodeId::from_number("530"),
            Ok(NodeId([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 255, 255
            ]))
        );
    }

    #[test]
    fn from_hex_invalid_input() {
        assert_eq!(
            NodeId::from_hex("NotAHexString"),
            Err(IdError::InvalidValue(String::from(
                "The input is not a valid hex string."
            )))
        );
    }

    #[test]
    fn from_hex_value_too_large() {
        assert_eq!(
            NodeId::from_hex("0x75BCD15"),
            Err(IdError::ValueOutOfRange(String::from(
                "The value is to large!",
            )))
        );

        assert_eq!(
            NodeId::from_hex("0x75bcd15"),
            Err(IdError::ValueOutOfRange(String::from(
                "The value is to large!",
            )))
        );
    }

    #[test]
    fn from_hex_success() {
        assert_eq!(
            NodeId::from_hex("0x1FE"),
            Ok(NodeId([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255
            ]))
        );

        assert_eq!(
            NodeId::from_hex("0x1fe"),
            Ok(NodeId([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255
            ]))
        );
    }

    #[test]
    fn from_hex_success_partial_remaining() {
        assert_eq!(
            NodeId::from_hex("0x212"),
            Ok(NodeId([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 255, 255
            ]))
        );
    }
}
