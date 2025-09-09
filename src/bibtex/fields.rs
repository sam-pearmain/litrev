use std::ops::{Index, IndexMut};

use super::parser::BibTeXParser;

#[derive(Debug, PartialEq)]
pub enum BibTeXField {
    Address(String), Annote(String), Author(Authors), Booktitle(String), 
    Chapter(u32), Crossref(String), Doi(String), Edition(u16), Editor(Authors), 
    Email(String), HowPublished(String), Institution(String), Journal(String), 
    Day(u8), Month(u8), Year(u16), Note(String), Number(u16), Organization(String), 
    Pages(Pages), Publisher(String), School(String), Series(String), Title(String), 
    Type(String), Volume(u8), NonStandard { key: String, value: String }, 
} 

impl BibTeXField {
    pub fn is_non_standard_field(&self) -> bool {
        matches!(self, Self::NonStandard { .. })
    }

    pub fn is_standard_field(&self) -> bool {
        !matches!(self, Self::NonStandard { .. })
    }
}

#[derive(Debug, PartialEq)]
pub struct Author {
    forename: String, 
    surname: String, 
    suffix: Option<String>,
    prefix: Option<String>,  
    is_lead_author: bool, 
}

#[derive(Debug, PartialEq)]
pub struct Authors {
    authors: Vec<Author>, 
}

#[derive(Debug, PartialEq)]
pub struct PageRange {
    start: i32, 
    end: Option<i32>, // if we just have one page then no end page is given 
}

#[derive(Debug, PartialEq)]
pub struct Pages {
    pages: Vec<PageRange>
}