use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::RWMol;
use flate2::bufread::GzDecoder;

pub(crate) struct SdfRecords<R> {
    reader: R,
    done: bool,
}

impl<R: BufRead> SdfRecords<R> {
    pub(crate) fn new(reader: R) -> Self {
        Self {
            reader,
            done: false,
        }
    }

    pub(crate) fn next_record(&mut self) -> Option<Result<String, String>> {
        if self.done {
            return None;
        }
        let mut record = Vec::new();
        let mut line = Vec::new();
        loop {
            line.clear();
            match self.reader.read_until(b'\n', &mut line) {
                Ok(0) => {
                    self.done = true;
                    if record.iter().all(u8::is_ascii_whitespace) {
                        return None;
                    }
                    break;
                }
                Ok(_) => {
                    let delimiter = line.strip_suffix(b"\n").unwrap_or(&line);
                    let delimiter = delimiter.strip_suffix(b"\r").unwrap_or(delimiter);
                    if delimiter == b"$$$$" {
                        if record.iter().all(u8::is_ascii_whitespace) {
                            record.clear();
                            continue;
                        }
                        break;
                    }
                    record.extend_from_slice(&line);
                }
                Err(error) => {
                    self.done = true;
                    return Some(Err(format!("SDF read failed: {error}")));
                }
            }
        }
        Some(String::from_utf8(record).map_err(|e| format!("SDF record is not UTF-8: {e}")))
    }
}

pub struct MolBlockIter<R: BufRead> {
    records: SdfRecords<R>,
    sanitize: bool,
    remove_hs: bool,
    strict_parsing: bool,
}

impl<R: BufRead> MolBlockIter<R> {
    pub fn new(reader: R, sanitize: bool, remove_hs: bool, strict_parsing: bool) -> Self {
        Self {
            records: SdfRecords::new(reader),
            sanitize,
            remove_hs,
            strict_parsing,
        }
    }
}

pub type GzBufReader = BufReader<GzDecoder<BufReader<File>>>;

pub(crate) fn open_gz(path: impl AsRef<Path>) -> std::io::Result<GzBufReader> {
    Ok(BufReader::new(GzDecoder::new(BufReader::new(File::open(
        path,
    )?))))
}

impl MolBlockIter<GzBufReader> {
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

impl<R: BufRead> Iterator for MolBlockIter<R> {
    type Item = Result<RWMol, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.records.next_record().map(|record| {
            let record = record?;
            RWMol::try_from_mol_block(&record, self.sanitize, self.remove_hs, self.strict_parsing)
                .map_err(|e| e.to_string())
        })
    }
}

impl<R: BufRead> std::iter::FusedIterator for MolBlockIter<R> {}
