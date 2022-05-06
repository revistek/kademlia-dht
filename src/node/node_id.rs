// The Node ID object.
// Represents a 160-bit number.
#[derive(Debug)]
pub struct NodeId([u8; 20]);

#[derive(Debug)]
pub enum IdErrors {
    ValueOutOfRange(String),
}

impl NodeId {
    //
    pub fn from_number(number: &str) -> Result<NodeId, IdErrors> {
        let number_str = str::replace(number, ',', ""); // Clean string. Remove all commas. The
                                                        // string is expected to represent a whole
                                                        // number value.
        let number_bytes = number_str.into_bytes();
        let mut new_id = NodeId([0; 20]); // The node ID object to be returned.
        let mut temp: u128 = 0;
        let mut id_arr_idx: usize = 19; // The index of the array to update.
        let ascii_base_codepoint = 48; // The base ASCII codepoint for numbers (i.e., 0x30 since
                                       // number = 0x30 + number).
        for (idx, x) in number_bytes.iter().rev().enumerate() {
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
                    return Err(IdErrors::ValueOutOfRange(String::from(
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
