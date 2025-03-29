use std::io::{self, Read, Seek};
use byteorder::{ReadBytesExt, LittleEndian};
use anyhow::{Result, Context};

/// Represents an OVL file structure
pub struct OVLFile {
    pub header: OVLHeader,
    pub sections: Vec<OVLSection>,
}

#[derive(Debug)]
pub struct OVLHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub section_count: u32,
}

#[derive(Debug)]
pub struct OVLSection {
    pub name: String,
    pub data: Vec<u8>,
    pub size: u32,
}

impl OVLFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let header = OVLHeader::read(reader).context("Failed to read OVL header")?;
        
        let mut sections = Vec::with_capacity(header.section_count as usize);
        for _ in 0..header.section_count {
            let section = OVLSection::read(reader).context("Failed to read OVL section")?;
            sections.push(section);
        }

        Ok(OVLFile { header, sections })
    }
}

impl OVLHeader {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if &magic != b"FLOV" {
            anyhow::bail!("Invalid OVL file magic number");
        }

        let version = reader.read_u32::<LittleEndian>()?;
        let section_count = reader.read_u32::<LittleEndian>()?;

        Ok(OVLHeader {
            magic,
            version,
            section_count,
        })
    }
}

impl OVLSection {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let size = reader.read_u32::<LittleEndian>()?;
        let name_length = reader.read_u32::<LittleEndian>()?;

        let mut name_bytes = vec![0u8; name_length as usize];
        reader.read_exact(&mut name_bytes)?;
        let name = String::from_utf8(name_bytes).context("Invalid section name encoding")?;

        let mut data = vec![0u8; size as usize];
        reader.read_exact(&mut data)?;

        Ok(OVLSection {
            name,
            data,
            size,
        })
    }
}