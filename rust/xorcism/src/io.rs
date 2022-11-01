use crate::Xorcism;
use std::io::{self, Read, Write};

pub struct Reader<'a, R> {
    pub(crate) xorcism: Xorcism<'a>,
    pub(crate) reader: R,
}

impl<'a, R> Read for Reader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes = self.reader.read(buf)?;
        self.xorcism.munge_in_place(buf);
        Ok(bytes)
    }
}

pub struct Writer<'a, W> {
    pub(crate) xorcism: Xorcism<'a>,
    pub(crate) writer: W,
}

impl<'a, W> Write for Writer<'a, W>
where
    W: Write,
{
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        for ref munged in self.xorcism.munge(_buf) {
            written += self.writer.write(std::slice::from_ref(munged))?;
        }
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
