use std::str::FromStr;

use super::fields::*;

#[derive(Debug, PartialEq)]
pub enum BibTeXEntryKind {
    Article, Book, Booklet, Conference, Inbook, InCollection, 
    InProceedings, Manual, MasterThesis, Misc, PhdThesis,
    Proceedings, TechReport, Unpublished, Unknown   
}

impl FromStr for BibTeXEntryKind {
    type Err = String;
    
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
            _               => Ok(Self::Unknown),
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

#[derive(Debug, PartialEq)]
pub struct BibTeXEntry {
    /// The kind of entry
    kind: BibTeXEntryKind, 
    /// The key for the entry, for example: Anderson2004
    citekey: String, 
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

impl BibTeXEntry {
    pub fn new(kind: BibTeXEntryKind, citekey: String) -> Self {
        Self { kind, citekey, fields: Vec::new() }
    }

    pub fn add_field(&mut self, field: BibTeXField) {
        self.fields.push(field);
    }

    pub fn title(&self) -> Option<&String> {
        get_field!(&self.fields, Title)
    }

    pub fn authors(&self) -> Option<&Authors> {
        get_field!(&self.fields, Author)
    }

    pub fn day(&self) -> Option<&u8> {
        get_field!(&self.fields, Day)
    }

    pub fn month(&self) -> Option<&u8> {
        get_field!(&self.fields, Month)
    }

    pub fn year(&self) -> Option<&u16> {
        get_field!(&self.fields, Year)
    }

    pub fn doi(&self) -> Option<&String> {
        get_field!(&self.fields, Doi)
    }

    pub fn date(&self) -> Option<Date> {
        todo!()
    }
}

impl std::fmt::Display for BibTeXEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "@{}{{{}", self.kind, self.citekey)

        todo!()
    }
}