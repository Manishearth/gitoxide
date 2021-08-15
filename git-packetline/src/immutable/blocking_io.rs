use std::io;

use crate::{
    encode,
    immutable::{Band, Error, Text},
    Channel, PacketLine,
};

impl<'a> Band<'a> {
    /// Serialize this instance to `out`, returning the amount of bytes written.
    ///
    /// The data written to `out` can be decoded with [`Borrowed::decode_band()]`.
    pub fn write_to(&self, out: impl io::Write) -> io::Result<usize> {
        match self {
            Band::Data(d) => encode::band_to_write(Channel::Data, d, out),
            Band::Progress(d) => encode::band_to_write(Channel::Progress, d, out),
            Band::Error(d) => encode::band_to_write(Channel::Error, d, out),
        }
    }
}

impl<'a> Text<'a> {
    /// Serialize this instance to `out`, appending a newline if there is none, returning the amount of bytes written.
    pub fn write_to(&self, out: impl io::Write) -> io::Result<usize> {
        encode::text_to_write(self.0, out)
    }
}

impl<'a> Error<'a> {
    /// Serialize this line as error to `out`.
    ///
    /// This includes a marker to allow decoding it outside of a side-band channel, returning the amount of bytes written.
    pub fn write_to(&self, out: impl io::Write) -> io::Result<usize> {
        encode::error_to_write(self.0, out)
    }
}

impl<'a> PacketLine<'a> {
    /// Serialize this instance to `out` in git `packetline` format, returning the amount of bytes written to `out`.
    pub fn write_to(&self, out: impl io::Write) -> io::Result<usize> {
        match self {
            PacketLine::Data(d) => encode::data_to_write(d, out),
            PacketLine::Flush => encode::flush_to_write(out),
            PacketLine::Delimiter => encode::delim_to_write(out),
            PacketLine::ResponseEnd => encode::response_end_to_write(out),
        }
    }
}
