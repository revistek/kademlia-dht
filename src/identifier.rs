// An unsigened integer identifier of variable length, heap allocated.
use bitvec::prelude::BitVec;
use bitvec::prelude::Msb0;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    value: BitVec<u8, Msb0>,
}

#[derive(Debug, PartialEq)]
pub enum IdentifierError {
    InvalidValue(String),
}

impl Identifier {
    pub fn new(id: &[u8]) -> Result<Identifier, IdentifierError> {
        if id.len() == 0 {
            return Err(IdentifierError::InvalidValue(String::from(
                "The ID cannot be nothing.",
            )));
        }

        Ok(Identifier {
            value: BitVec::<u8, Msb0>::from_slice(id),
        })
    }

    pub fn get_id(&self) -> &BitVec<u8, Msb0> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid() {
        let id = Identifier::new(b"101").unwrap();

        assert_eq!(id.value, BitVec::<u8, Msb0>::from_slice(b"101"));
    }

    #[test]
    fn new_invalid() {
        assert_eq!(
            Identifier::new(b""),
            Err(IdentifierError::InvalidValue(String::from(
                "The ID cannot be nothing.",
            )))
        );
    }

    #[test]
    fn get_id() {
        let id = Identifier::new(b"101").unwrap();
        let id_val = BitVec::<u8, Msb0>::from_slice(b"101");

        assert_eq!(id.get_id(), &id_val);
    }
}
