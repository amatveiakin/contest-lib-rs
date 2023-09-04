use crate::io;


pub fn reader_from_string(input: impl ToString) -> io::Reader<std::io::Cursor<Vec<u8>>> {
    io::Reader::new(std::io::Cursor::new(input.to_string().into_bytes()))
}
