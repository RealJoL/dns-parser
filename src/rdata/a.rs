use std::{convert::TryInto, net::Ipv4Addr};

use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Record(pub Ipv4Addr);

impl<'a> super::Record<'a> for Record {
    const TYPE: isize = 1;

    fn parse(rdata: &'a [u8], _original: &'a [u8]) -> super::RDataResult<'a> {
        let rdata: [u8; 4] = rdata.try_into().map_err(|_| Error::WrongRdataLength)?;
        let address = Ipv4Addr::from(rdata);
        let record = Record(address);
        Ok(super::RData::A(record))
    }
}
