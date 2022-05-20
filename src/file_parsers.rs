use flate2::bufread::GzDecoder;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::ROMol;

/* this code taken from https://github.com/chrissly31415/rdkitcffi */

pub fn read_sdfile_gz(sd_file_gz_path: &str) -> Vec<Option<ROMol>> {
    let sd_file = std::fs::File::open(sd_file_gz_path).expect("Could not load file");
    // let mut sd_file_buf_reader = std::io::Gz::new(sd_file);
    let sd_file_decoder = flate2::bufread::GzDecoder::new(std::io::BufReader::new(sd_file));
    let mut sd_file_decoder_buf_read = std::io::BufReader::new(sd_file_decoder);

    let mut molblocks = Vec::new();
    let mut molblock_buf = Vec::new();
    while let Ok(_) = sd_file_decoder_buf_read.read_until(b'$', &mut molblock_buf) {
        sd_file_decoder_buf_read.consume(3);

        let molblock = std::str::from_utf8(&molblock_buf).unwrap();
        let mol = ROMol::new(&molblock, "");
        molblocks.push(mol);
    }

    molblocks
}

pub struct MolBlockIter<R: BufRead> {
    buf_read: R,
    buf: Vec<u8>,
}

impl<R: BufRead> MolBlockIter<R> {
    pub fn new(buf_read: R) -> Self {
        MolBlockIter {
            buf_read,
            buf: Vec::with_capacity(1024),
        }
    }
}

pub type GzBufReader = BufReader<flate2::bufread::GzDecoder<BufReader<File>>>;

impl MolBlockIter<GzBufReader> {
    pub fn from_gz_file(p: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = p.as_ref().to_owned();

        let file = std::fs::File::open(path).unwrap();
        let buf_reader = std::io::BufReader::new(file);

        let gz_decoder = GzDecoder::new(buf_reader);
        let gz_buf_reader = std::io::BufReader::new(gz_decoder);

        Ok(Self::new(gz_buf_reader))
    }
}

impl<R: BufRead> Iterator for MolBlockIter<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buf.clear();
            let read = self.buf_read.read_until(b'$', &mut self.buf).unwrap();
            if read == 0 {
                return None;
            } else if read == 1 {
                continue;
            } else {
                break;
            }
        }

        let block = std::str::from_utf8(&self.buf).unwrap();
        let block = block.trim();

        return Some(block.to_owned());
    }
}

/// read a classical .sdf file or sdf.gz
pub fn read_sdfile(sd_file_path: &str) -> Vec<Option<ROMol>> {
    let sd_file = std::fs::File::open(sd_file_path).expect("Could not load file");
    let mut sd_file_buf_reader = std::io::BufReader::new(sd_file);

    let mut molblocks = Vec::new();
    let mut molblock_buf = Vec::new();
    while let Ok(_) = sd_file_buf_reader.read_until(b'$', &mut molblock_buf) {
        sd_file_buf_reader.consume(3);

        let molblock = std::str::from_utf8(&molblock_buf).unwrap();
        let mol = ROMol::new(&molblock, "");
        molblocks.push(mol);
    }

    molblocks
}
