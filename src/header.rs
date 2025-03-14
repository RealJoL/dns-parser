use crate::{Error, Opcode, ResponseCode};
use std::convert::TryInto;

mod flag {
    pub const QUERY: u16 = 0b1000_0000_0000_0000;
    pub const OPCODE_MASK: u16 = 0b0111_1000_0000_0000;
    pub const AUTHORITATIVE: u16 = 0b0000_0100_0000_0000;
    pub const TRUNCATED: u16 = 0b0000_0010_0000_0000;
    pub const RECURSION_DESIRED: u16 = 0b0000_0001_0000_0000;
    pub const RECURSION_AVAILABLE: u16 = 0b0000_0000_1000_0000;
    pub const AUTHENTICATED_DATA: u16 = 0b0000_0000_0010_0000;
    pub const CHECKING_DISABLED: u16 = 0b0000_0000_0001_0000;
    pub const RESERVED_MASK: u16 = 0b0000_0000_0100_0000;
    pub const RESPONSE_CODE_MASK: u16 = 0b0000_0000_0000_1111;
}

/// Represents parsed header of the packet
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)] // fields are from the spec I think
pub struct Header {
    pub id: u16,
    pub query: bool,
    pub opcode: Opcode,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub authenticated_data: bool,
    pub checking_disabled: bool,
    pub response_code: ResponseCode,
    pub questions: u16,
    pub answers: u16,
    pub nameservers: u16,
    pub additional: u16,
}

impl Header {
    /// Parse the header into a header structure
    pub fn parse(data: &[u8]) -> Result<Header, Error> {
        if data.len() < 12 {
            return Err(Error::HeaderTooShort);
        }
        let flags = u16::from_be_bytes(data[2..4].try_into().unwrap());
        if flags & flag::RESERVED_MASK != 0 {
            return Err(Error::ReservedBitsAreNonZero);
        }
        let header = Header {
            id: u16::from_be_bytes(data[..2].try_into().unwrap()),
            query: flags & flag::QUERY == 0,
            opcode: ((flags & flag::OPCODE_MASK) >> flag::OPCODE_MASK.trailing_zeros()).into(),
            authoritative: flags & flag::AUTHORITATIVE != 0,
            truncated: flags & flag::TRUNCATED != 0,
            recursion_desired: flags & flag::RECURSION_DESIRED != 0,
            recursion_available: flags & flag::RECURSION_AVAILABLE != 0,
            authenticated_data: flags & flag::AUTHENTICATED_DATA != 0,
            checking_disabled: flags & flag::CHECKING_DISABLED != 0,
            response_code: From::from((flags & flag::RESPONSE_CODE_MASK) as u8),
            questions: u16::from_be_bytes(data[4..6].try_into().unwrap()),
            answers: u16::from_be_bytes(data[6..8].try_into().unwrap()),
            nameservers: u16::from_be_bytes(data[8..10].try_into().unwrap()),
            additional: u16::from_be_bytes(data[10..12].try_into().unwrap()),
        };
        Ok(header)
    }
    /// Write a header to a buffer slice
    ///
    /// # Panics
    ///
    /// When buffer size is not exactly 12 bytes
    pub fn write(&self, data: &mut [u8]) {
        if data.len() != 12 {
            panic!("Header size is exactly 12 bytes");
        }
        let mut flags = 0u16;
        flags |= Into::<u16>::into(self.opcode) << flag::OPCODE_MASK.trailing_zeros();
        flags |= Into::<u8>::into(self.response_code) as u16;
        if !self.query {
            flags |= flag::QUERY;
        }
        if self.authoritative {
            flags |= flag::AUTHORITATIVE;
        }
        if self.recursion_desired {
            flags |= flag::RECURSION_DESIRED;
        }
        if self.recursion_available {
            flags |= flag::RECURSION_AVAILABLE;
        }
        if self.truncated {
            flags |= flag::TRUNCATED;
        }
        data[..2].copy_from_slice(&(self.id as u16).to_be_bytes());
        data[2..4].copy_from_slice(&(flags as u16).to_be_bytes());
        data[4..6].copy_from_slice(&(self.questions as u16).to_be_bytes());
        data[6..8].copy_from_slice(&(self.answers as u16).to_be_bytes());
        data[8..10].copy_from_slice(&(self.nameservers as u16).to_be_bytes());
        data[10..12].copy_from_slice(&(self.additional as u16).to_be_bytes());
    }
    /// Set "truncated flag" in the raw data
    // shouldn't this method be non-public?
    pub fn set_truncated(data: &mut [u8]) {
        let oldflags = u16::from_be_bytes(data[2..4].try_into().unwrap());
        data[2..4].copy_from_slice(&(oldflags & flag::TRUNCATED as u16).to_be_bytes());
    }
    /// Returns a size of the header (always 12 bytes)
    pub fn size() -> usize {
        12
    }
}

#[cfg(test)]
mod test {

    use crate::Header;
    use crate::Opcode::*;
    use crate::ResponseCode::NoError;

    #[test]
    fn parse_example_query() {
        let query = b"\x06%\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\
                      \x07example\x03com\x00\x00\x01\x00\x01";
        let header = Header::parse(query).unwrap();
        assert_eq!(
            header,
            Header {
                id: 1573,
                query: true,
                opcode: StandardQuery,
                authoritative: false,
                truncated: false,
                recursion_desired: true,
                recursion_available: false,
                authenticated_data: false,
                checking_disabled: false,
                response_code: NoError,
                questions: 1,
                answers: 0,
                nameservers: 0,
                additional: 0,
            }
        );
    }

    #[test]
    fn parse_example_response() {
        let response = b"\x06%\x81\x80\x00\x01\x00\x01\x00\x00\x00\x00\
                         \x07example\x03com\x00\x00\x01\x00\x01\
                         \xc0\x0c\x00\x01\x00\x01\x00\x00\x04\xf8\
                         \x00\x04]\xb8\xd8\"";
        let header = Header::parse(response).unwrap();
        assert_eq!(
            header,
            Header {
                id: 1573,
                query: false,
                opcode: StandardQuery,
                authoritative: false,
                truncated: false,
                recursion_desired: true,
                recursion_available: true,
                authenticated_data: false,
                checking_disabled: false,
                response_code: NoError,
                questions: 1,
                answers: 1,
                nameservers: 0,
                additional: 0,
            }
        );
    }

    #[test]
    fn parse_query_with_ad_set() {
        let query = b"\x06%\x01\x20\x00\x01\x00\x00\x00\x00\x00\x00\
                      \x07example\x03com\x00\x00\x01\x00\x01";
        let header = Header::parse(query).unwrap();
        assert_eq!(
            header,
            Header {
                id: 1573,
                query: true,
                opcode: StandardQuery,
                authoritative: false,
                truncated: false,
                recursion_desired: true,
                recursion_available: false,
                authenticated_data: true,
                checking_disabled: false,
                response_code: NoError,
                questions: 1,
                answers: 0,
                nameservers: 0,
                additional: 0,
            }
        );
    }

    #[test]
    fn parse_query_with_cd_set() {
        let query = b"\x06%\x01\x10\x00\x01\x00\x00\x00\x00\x00\x00\
                      \x07example\x03com\x00\x00\x01\x00\x01";
        let header = Header::parse(query).unwrap();
        assert_eq!(
            header,
            Header {
                id: 1573,
                query: true,
                opcode: StandardQuery,
                authoritative: false,
                truncated: false,
                recursion_desired: true,
                recursion_available: false,
                authenticated_data: false,
                checking_disabled: true,
                response_code: NoError,
                questions: 1,
                answers: 0,
                nameservers: 0,
                additional: 0,
            }
        );
    }
}
