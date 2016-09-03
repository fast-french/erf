use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::mem;

pub struct ErfKey {
    pub res_ref: String,
    pub res_id: u32,
    pub res_type: u16,
    pub unused: u16,
}

pub struct ResType {
    pub ext: &'static str,
    pub category: &'static str,
}

pub struct ResourceListElement {
    pub offset_to_resource: u32,
    pub resource_size: u32,
}

impl ResourceListElement {
    pub fn from (file: &mut File, nb: usize) -> Vec<ResourceListElement> {
        let mut result: Vec<ResourceListElement> = Vec::new();
        for _ in 0..nb {
            let element = ResourceListElement {
                offset_to_resource: dword_as_u32(file),
                resource_size: dword_as_u32(file),
            };
            result.push(element);
        }
        result
    }

    pub fn display(&self) {
        println!("Offset: {}", self.offset_to_resource);
        println!("Size: {}", self.resource_size);
    }
}

impl ResType {
    pub fn new(category: &'static str, ext: &'static str) -> ResType {
        ResType {
            category: category,
            ext: ext,
        }
    }
}

impl ErfKey {
    pub fn from (file: &mut File, nb: usize) -> Vec<ErfKey> {
        let mut result: Vec<ErfKey> = Vec::new();
        for _ in 0..nb {
            let element = ErfKey {
                res_ref: string(file, 16),
                res_id: dword_as_u32(file),
                res_type: word_as_u16(file),
                unused: word_as_u16(file),
            };
            result.push(element);
        }
        result
    }

    pub fn file_name(&self) -> String {
        self.res_ref.clone() + "." + self.file_extension().ext
    }

    pub fn file_extension(&self) -> ResType {
        match self.res_type {
            1    => ResType::new("binary", "bmp"),
            3    => ResType::new("binary", "tga"),
            4    => ResType::new("binary", "wav"),
            6    => ResType::new("binary", "plt"),
            7    => ResType::new("ini", "ini"),
            10   => ResType::new("text", "txt"),
            2002 => ResType::new("model", "mdl"),
            2009 => ResType::new("text", "nss"),
            2010 => ResType::new("binary", "ncs"),
            2012 => ResType::new("gff", "are"),
            2013 => ResType::new("text", "set"),
            2014 => ResType::new("gff", "ifo"),
            2015 => ResType::new("gff", "bic"),
            2016 => ResType::new("model", "wok"),
            2017 => ResType::new("text", "2da"),
            2022 => ResType::new("text", "txi"),
            2023 => ResType::new("gff", "git"),
            2025 => ResType::new("gff", "uti"),
            2027 => ResType::new("gff", "utc"),
            2029 => ResType::new("gff", "dlg"),
            2030 => ResType::new("gff", "itp"),
            2032 => ResType::new("gff", "utt"),
            2033 => ResType::new("binary", "dds"),
            2035 => ResType::new("gff", "uts"),
            2036 => ResType::new("binary", "ltr"),
            2037 => ResType::new("gff", "gff"),
            2038 => ResType::new("gff", "fac"),
            2040 => ResType::new("gff", "ute"),
            2042 => ResType::new("gff", "utd"),
            2044 => ResType::new("gff", "utp"),
            2045 => ResType::new("text", "dft"),
            2046 => ResType::new("gff", "gic"),
            2047 => ResType::new("gff", "gui"),
            2051 => ResType::new("gff", "utm"),
            2052 => ResType::new("model", "dwk"),
            2053 => ResType::new("model", "pwk"),
            2056 => ResType::new("gff", "jrl"),
            2058 => ResType::new("gff", "utw"),
            2060 => ResType::new("binary", "ssf"),
            2064 => ResType::new("binary", "ndb"),
            2065 => ResType::new("gff", "ptm"),
            2066 => ResType::new("gff", "ptt"),
            _ => panic!("File type not recognized"),
        }
    }

    pub fn display(&self) {
        println!("{}", self.file_name());
    }
}

pub struct StringListElement {
    pub language_id: u32,
    pub string_size: u32,
    pub string: String,
}

impl StringListElement {
    pub fn from(file: &mut File, nb: usize) -> Vec<StringListElement> {
        let mut result: Vec<StringListElement> = Vec::new();
        for _ in 0..nb {
            let mut element = StringListElement {
                language_id: dword_as_u32(file),
                string_size: dword_as_u32(file),
                string: String::new(),
            };
            element.string = string(file, element.string_size as usize);
            result.push(element);
        }
        result
    }

    pub fn display(&self) {
        println!("LanguageID = {}", self.language_id);
        println!("StringSize = {}", self.string_size);
        println!("Value = {}", self.string);
    }
}

pub struct Header {
    pub file_type: String,
    pub file_version: String,
    pub language_count: u32,
    pub localized_string_size: u32,
    pub entry_count: u32,
    pub offset_to_localized_string: u32,
    pub offset_to_key_list: u32,
    pub offset_to_resource_list: u32,
    pub build_year: u32,
    pub build_day: u32,
    pub description_str_ref: u32,
}

impl Header {
    pub fn from(file: &mut File) -> Header {
        Header {
            file_type: dword_as_string(file),
            file_version: dword_as_string(file),
            language_count: dword_as_u32(file),
            localized_string_size: dword_as_u32(file),
            entry_count: dword_as_u32(file),
            offset_to_localized_string: dword_as_u32(file),
            offset_to_key_list: dword_as_u32(file),
            offset_to_resource_list: dword_as_u32(file),
            build_year: dword_as_u32(file),
            build_day: dword_as_u32(file),
            description_str_ref: dword_as_u32(file),
        }
    }

    pub fn display(&self) {
        println!("FileType = {}", self.file_type);
        println!("FileVersion = {}", self.file_version);
        println!("LanguageCount = {}", self.language_count);
        println!("LocalizedStringSize = {}", self.localized_string_size);
        println!("EntryCount = {}", self.entry_count);
        println!("OffsetToLocalizedString = {}", self.offset_to_localized_string);
        println!("OffsetToKeyList = {}", self.offset_to_key_list);
        println!("OffsetToResourceList = {}", self.offset_to_resource_list);
        println!("BuildYear = {}", self.build_year);
        println!("BuildDay = {}", self.build_day);
        println!("DescriptionStrRef = {}", self.description_str_ref);
    }
}

pub fn string(file: &mut File, nb: usize) -> String {
    file.bytes().take(nb)
        .map(|byte| byte.expect("byte as character") as char)
        .collect::<String>()
}

pub fn dword_as_string(file: &mut File) -> String {
    string(file, 4)
}

pub fn dword_and_dump(file: &mut File, nb: i64) {
    for _ in 0..nb {
        let mut buffer = [0u8; 4];
        let _ = file.read_exact(&mut buffer).expect("Can't read DWORD value");
    }
}

pub fn word_as_u16(file: &mut File) -> u16 {
    let mut buffer = [0u8; 2];
    file.read_exact(&mut buffer).expect("Can't read HWORD value");
    unsafe {
        let result: u16 = mem::transmute(buffer);
        result
    }
}

pub fn dword_as_u32(file: &mut File) -> u32 {
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer).expect("Can't read DWORD value");
    unsafe {
        let result: u32 = mem::transmute(buffer);
        result
    }
}
