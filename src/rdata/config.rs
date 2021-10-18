use crate::rdata::*;

/// The enumeration that represents implemented types of DNS resource records data
#[derive(Debug, PartialEq)]
pub enum RData<'a> {
    A(A),
    AAAA(Aaaa),
    CNAME(Cname<'a>),
    MX(Mx<'a>),
    NS(Ns<'a>),
    PTR(Ptr<'a>),
    SOA(Soa<'a>),
    SRV(Srv<'a>),
    TXT(Txt<'a>),
    OPT(&'a [u8]),
}
