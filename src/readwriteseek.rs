use std::io::{Read, Seek, Write};

pub trait ReadWriteSeek: Read + Write + Seek {}

impl<T> ReadWriteSeek for T where T: Read + Write + Seek {}
