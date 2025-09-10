use std::str::FromStr;

use super::error::ParseError;
use super::fields::*;

#[derive(Debug, PartialEq)]
pub enum BibTeXEntryKind {
    Article, Book, Booklet, Conference, Inbook, InCollection, 
    InProceedings, Manual, MasterThesis, Misc, PhdThesis,
    Proceedings, TechReport, Unpublished, Unknown   
}

impl FromStr for BibTeXEntryKind {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "article"       => Ok(Self::Article), 
            "book"          => Ok(Self::Book), 
            "booklet"       => Ok(Self::Booklet),
            "conference"    => Ok(Self::Conference),
            "inbook"        => Ok(Self::Inbook),
            "incollection"  => Ok(Self::InCollection),
            "inproceedings" => Ok(Self::InProceedings),
            "manual"        => Ok(Self::Manual),
            "mastersthesis" => Ok(Self::MasterThesis),
            "misc"          => Ok(Self::Misc),
            "phdthesis"     => Ok(Self::PhdThesis),
            "proceedings"   => Ok(Self::Proceedings),
            "techreport"    => Ok(Self::TechReport),
            "unpublished"   => Ok(Self::Unpublished),
            _ => Err(ParseError::InvalidEntryKind(String::from(s))),
        }
    }
}

impl std::fmt::Display for BibTeXEntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Article       => write!(f, "article"), 
            Self::Book          => write!(f, "book"),  
            Self::Booklet       => write!(f, "booklet"), 
            Self::Conference    => write!(f, "conference"), 
            Self::Inbook        => write!(f, "inbook"),
            Self::InCollection  => write!(f, "incollection"), 
            Self::InProceedings => write!(f, "inproceedings"), 
            Self::Manual        => write!(f, "manual"), 
            Self::MasterThesis  => write!(f, "masterthesis"), 
            Self::Misc          => write!(f, "misc"), 
            Self::PhdThesis     => write!(f, "phdthesis"), 
            Self::Proceedings   => write!(f, "proceedings"), 
            Self::TechReport    => write!(f, "techreport"), 
            Self::Unpublished   => write!(f, "unpublished"), 
            Self::Unknown       => write!(f, "unknown"), 
        }
    }
}

impl BibTeXEntryKind {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Debug, PartialEq)]
pub struct BibTeXEntry {
    /// The kind of entry
    pub kind: BibTeXEntryKind, 
    /// The key for the entry, for example: Anderson2004
    pub citekey: String, 
    /// The fields of the entry
    fields: Vec<BibTeXField>
}

macro_rules! get_field {
    ($fields:expr, $variant:ident) => {
        {
            let mut result = None;
            for field in $fields {
                if let BibTeXField::$variant(value) = field {
                    result = Some(value);
                    break;
                }
            }
            result
        }
    };
}

macro_rules! getters_impl {
    ( $( $fn_name:ident, $variant:ident -> $ret:ty );* $(;)? ) => {
        $(
            pub fn $fn_name(&self) -> Option<&$ret> {
                get_field!(&self.fields, $variant)
            }
        )*
    };
}

impl BibTeXEntry {
    pub fn new(kind: BibTeXEntryKind, citekey: String) -> Self {
        Self { kind, citekey, fields: Vec::new() }
    }

    pub fn add_field(&mut self, field: BibTeXField) {
        self.fields.push(field);
    }

    getters_impl!(
        address, Address -> String;
        annote, Annote -> String;
        author, Author -> Authors;
        book_title, BookTitle -> String; 
        chapter, Chapter -> u8;         
        crossref, Crossref -> String;
        doi, Doi -> String;
        edition, Edition -> u16;
        editor, Editor -> Authors;
        email, Email -> String;
        how_published, HowPublished -> String;
        institution, Institution -> String;
        journal, Journal -> String;
        day, Day -> u8;
        month, Month -> u8;
        year, Year -> u16;
        note, Note -> String;
        number, Number -> u16;
        organization, Organization -> String;
        pages, Pages -> Pages;
        publisher, Publisher -> String;
        school, School -> String;
        series, Series -> String;
        title, Title -> String;
        volume, Volume -> u8;
    );

    pub fn non_standard_field(&self, key: &str) -> Option<&String> {
        for field in &self.fields {
            if let BibTeXField::NonStandard { key: field_key, value } = field {
                if field_key.eq_ignore_ascii_case(key) {
                    return Some(value)
                }
            }
        }
        None
    }
}

impl std::fmt::Display for BibTeXEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "@{}{{{}", self.kind, self.citekey);

        todo!()
    }
}