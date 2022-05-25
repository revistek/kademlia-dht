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

    pub fn is_match(&self, other: &Identifier) -> bool {
        // The first 7 elements/bits are always the same. They may be some kind of header.
        let prefix_length = self.value.len();

        if prefix_length <= other.value.len() {
            return other.value[0..prefix_length] == self.value;
        }

        false
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

    #[test]
    fn is_match() {
        let mut id = Identifier::new(b"101").unwrap();
        let mut other = Identifier::new(b"101").unwrap();

        assert_eq!(id.value.len() == other.value.len(), true);
        assert_eq!(id.is_match(&other), true);

        other = Identifier::new(b"1").unwrap();

        assert_eq!(id.value.len() > other.value.len(), true);
        assert_eq!(id.is_match(&other), false);

        other = Identifier::new(b"17").unwrap();
        id = Identifier::new(b"1").unwrap();

        assert_eq!(id.value.len() < other.value.len(), true);
        assert_eq!(id.is_match(&other), true);
    }
}
