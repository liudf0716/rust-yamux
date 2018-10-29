// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
// OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::marker::PhantomData;
use stream;
use super::{Data, WindowUpdate, Ping, GoAway};


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Data,
    WindowUpdate,
    Ping,
    GoAway
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Version(pub u8);


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Len(pub u32);


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Flags(pub u16);

impl Flags {
    pub fn contains(self, other: Flags) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn and(self, other: Flags) -> Flags {
        Flags(self.0 | other.0)
    }
}

/// Termination code for use with GoAway frames.
pub const CODE_TERM: u32 = 0;
/// Protocol error code for use with GoAway frames.
pub const ECODE_PROTO: u32 = 1;
/// Internal error code for use with GoAway frames.
pub const ECODE_INTERNAL: u32 = 2;


pub const SYN: Flags = Flags(1);
pub const ACK: Flags = Flags(2);
pub const FIN: Flags = Flags(4);
pub const RST: Flags = Flags(8);


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawHeader {
    pub version: Version,
    pub typ: Type,
    pub flags: Flags,
    pub stream_id: stream::Id,
    pub length: Len
}


#[derive(Clone, Debug)]
pub struct Header<T> {
    raw_header: RawHeader,
    header_type: PhantomData<T>
}

impl<T> Header<T> {
    pub(crate) fn assert(raw: RawHeader) -> Self {
        Header {
            raw_header: raw,
            header_type: PhantomData
        }
    }

    pub fn id(&self) -> stream::Id {
        self.raw_header.stream_id
    }

    pub fn flags(&self) -> Flags {
        self.raw_header.flags
    }

    pub fn into_raw(self) -> RawHeader {
        self.raw_header
    }
}

impl Header<Data> {
    pub fn data(id: stream::Id, len: u32) -> Self {
        Header {
            raw_header: RawHeader {
                version: Version(0),
                typ: Type::Data,
                flags: Flags(0),
                stream_id: id,
                length: Len(len)
            },
            header_type: PhantomData
        }
    }

    pub fn syn(&mut self) {
        self.raw_header.flags.0 |= SYN.0
    }

    pub fn ack(&mut self) {
        self.raw_header.flags.0 |= ACK.0
    }

    pub fn fin(&mut self) {
        self.raw_header.flags.0 |= FIN.0
    }

    pub fn rst(&mut self) {
        self.raw_header.flags.0 |= RST.0
    }

    pub fn len(&self) -> u32 {
        self.raw_header.length.0
    }
}

impl Header<WindowUpdate> {
    pub fn window_update(id: stream::Id, credit: u32) -> Self {
        Header {
            raw_header: RawHeader {
                version: Version(0),
                typ: Type::WindowUpdate,
                flags: Flags(0),
                stream_id: id,
                length: Len(credit)
            },
            header_type: PhantomData
        }
    }

    pub fn syn(&mut self) {
        self.raw_header.flags.0 |= SYN.0
    }

    pub fn ack(&mut self) {
        self.raw_header.flags.0 |= ACK.0
    }

    pub fn fin(&mut self) {
        self.raw_header.flags.0 |= FIN.0
    }

    pub fn rst(&mut self) {
        self.raw_header.flags.0 |= RST.0
    }

    pub fn credit(&self) -> u32 {
        self.raw_header.length.0
    }
}

impl Header<Ping> {
    pub fn ping(nonce: u32) -> Self {
        Header {
            raw_header: RawHeader {
                version: Version(0),
                typ: Type::Ping,
                flags: Flags(0),
                stream_id: stream::Id::new(0),
                length: Len(nonce)
            },
            header_type: PhantomData
        }
    }

    pub fn syn(&mut self) {
        self.raw_header.flags.0 |= SYN.0
    }

    pub fn ack(&mut self) {
        self.raw_header.flags.0 |= ACK.0
    }

    pub fn nonce(&self) -> u32 {
        self.raw_header.length.0
    }
}

impl Header<GoAway> {
    pub fn go_away(error_code: u32) -> Self {
        Header {
            raw_header: RawHeader {
                version: Version(0),
                typ: Type::GoAway,
                flags: Flags(0),
                stream_id: stream::Id::new(0),
                length: Len(error_code)
            },
            header_type: PhantomData
        }
    }

    pub fn error_code(&self) -> u32 {
        self.raw_header.length.0
    }
}

