use std::fmt::{Display, Formatter};
use crate::storage::{Storage, Scan, Range};
use crate::storage::error::{Error, Result};

pub struct Wisc {}

impl Display for Wisc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "wisc")
    }
}

impl Storage for Wisc {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        todo!()
    }

    fn set(&mut self, key: &[u8], value: Vec<u8>) -> Result<()> {
        todo!()
    }

    fn delete(&mut self, key: &[u8]) -> Result<()> {
        todo!()
    }

    fn flush(&mut self) -> Result<()> {
        todo!()
    }

    fn scan(&self, range: Range) -> Scan {
        todo!()
    }
}