//
// Copyright 2018 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # Types used in db files
//! Offset an unsigned 48 bit integer used as file offset
//! U24 an unsigned 24 bit integer for data element sizes

use page::PAGE_SIZE;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use std::io::Cursor;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
/// Pointer to persistent data. Limited to 2^48
pub struct Offset(u64);

impl Ord for Offset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Offset {
    fn partial_cmp(&self, other: &Offset) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<u64> for Offset {
    fn from(n: u64) -> Self {
        Offset(n & 0xffffffffffffu64)
    }
}

impl<'a> From<&'a [u8]> for Offset {
    fn from(slice: &'a [u8]) -> Self {
        Offset::from(Cursor::new(slice).read_u48::<BigEndian>().unwrap())
    }
}

/// can read offsets from this
pub trait OffsetReader {
    /// read offset
    fn read_offset (&mut self) -> Offset;
}

impl OffsetReader for Cursor<Vec<u8>> {
    fn read_offset(&mut self) -> Offset {
        Offset(self.read_u48::<BigEndian>().unwrap())
    }
}

impl Offset {
    /// serialize to a vector of bytes
    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.write_u48::<BigEndian>(self.0).unwrap();
        v
    }

    /// convert to a number
    pub fn as_u64 (&self) -> u64 {
        return self.0;
    }


    /// offset of the page of this offset
    pub fn this_page(&self) -> Offset {
        Offset::from((self.0/ PAGE_SIZE as u64)* PAGE_SIZE as u64)
    }

    /// page offset after this offset
    pub fn next_page(&self) -> Offset {
        Offset::from((self.0/ PAGE_SIZE as u64 + 1)* PAGE_SIZE as u64)
    }

    /// compute page number of an offset
    pub fn page_number(&self) -> u64 {
        self.0/PAGE_SIZE as u64
    }

    /// position within the offset's page
    pub fn in_page_pos(&self) -> usize {
        (self.0 - (self.0/ PAGE_SIZE as u64)* PAGE_SIZE as u64) as usize
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
pub(crate) struct U24 (usize);

impl From<usize> for U24 {
    fn from(n: usize) -> Self {
        U24(n & 0xffffffusize)
    }
}

impl<'a> From<&'a [u8]> for U24 {
    fn from(slice: &'a [u8]) -> Self {
        U24::from(Cursor::new(slice).read_u24::<BigEndian>().unwrap() as usize)
    }
}

impl U24 {
    pub fn as_usize (&self) -> usize {
        return self.0;
    }

    pub fn serialize (&self, mut into: &mut [u8]) {
        into.write_u24::<BigEndian>(self.0 as u32).unwrap();
    }
}

