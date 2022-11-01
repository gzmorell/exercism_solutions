use std::io::{Read, Write};

pub mod io;

pub trait IsSized<'a> {}
impl<'a, T: ?Sized> IsSized<'a> for T {}

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    idx: usize,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Self
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Self {
            key: key.as_ref(),
            idx: 0,
        }
    }

    #[inline]
    fn inc(&mut self) -> u8 {
        let byte = self.key[self.idx];
        self.idx = (self.idx + 1) % self.key.len();
        byte
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            *byte ^= self.inc();
        }
    }

    pub fn munge<'borrow, Data>(
        &'borrow mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + IsSized<'a> + 'borrow
    where
        Data: IntoIterator,
        Data::IntoIter: 'borrow,
        Data::Item: std::ops::BitXor<u8, Output = u8>,
    {
        data.into_iter().map(move |byte| byte ^ self.inc())
    }

    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        io::Reader {
            xorcism: self,
            reader,
        }
    }

    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        io::Writer {
            xorcism: self,
            writer,
        }
    }
}
