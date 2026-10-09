use crate::{
    file_parsers::{open_gz, SdfRecords},
    GzBufReader, RWMol,
};
use std::{io::BufRead, path::Path};

pub struct VerboseMolBlockIter<R: BufRead> {
    records: SdfRecords<R>,
    sanitize: bool,
    remove_hs: bool,
    strict_parsing: bool,
}

impl<R: BufRead> VerboseMolBlockIter<R> {
    pub fn new(reader: R, sanitize: bool, remove_hs: bool, strict_parsing: bool) -> Self {
        Self {
            records: SdfRecords::new(reader),
            sanitize,
            remove_hs,
            strict_parsing,
        }
    }
}

impl VerboseMolBlockIter<GzBufReader> {
    pub fn from_gz_file(
        path: impl AsRef<Path>,
        sanitize: bool,
        remove_hs: bool,
        strict_parsing: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::new(
            open_gz(path)?,
            sanitize,
            remove_hs,
            strict_parsing,
        ))
    }
}

impl<R: BufRead> Iterator for VerboseMolBlockIter<R> {
    type Item = Result<(RWMol, String), String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.records.next_record().map(|record| {
            let record = record?;
            let mol = RWMol::try_from_mol_block(
                &record,
                self.sanitize,
                self.remove_hs,
                self.strict_parsing,
            )
            .map_err(|e| e.to_string())?;
            Ok((mol, record))
        })
    }
}

impl<R: BufRead> std::iter::FusedIterator for VerboseMolBlockIter<R> {}
