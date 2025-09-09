use std::str::FromStr; 

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
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.forename, self.surname)
    }
}

#[derive(Debug, PartialEq)]
pub struct Authors {
    authors: Vec<Author>, 
}

impl std::fmt::Display for Authors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iterator = self.authors.iter().peekable();
        
        while let Some(author) = iterator.next() {
            write!(f, "{}", author)?;

            if iterator.peek().is_some() {
                write!(f, " and ")?;
            }
        }

        Ok(())
    }
}

pub enum Month {
    Jan, Feb, Mar, Apr, May, Jun, 
    Jul, Aug, Sep, Oct, Nov, Dec, 
}

impl FromStr for Month {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "jan" | "january" | "1"   => Ok(Self::Jan),
            "feb" | "february" | "2"  => Ok(Self::Feb),
            "mar" | "march" | "3"     => Ok(Self::Mar),
            "apr" | "april" | "4"     => Ok(Self::Apr),
            "may" | "5"               => Ok(Self::May),
            "jun" | "june" | "6"      => Ok(Self::Jun),
            "jul" | "july" | "7"      => Ok(Self::Jul),
            "aug" | "august" | "8"    => Ok(Self::Aug),
            "sep" | "september" | "9" => Ok(Self::Sep),
            "oct" | "october" | "10"  => Ok(Self::Oct),
            "nov" | "november" | "11" => Ok(Self::Nov),
            "dec" | "december" | "12" => Ok(Self::Dec),
            _ => Err("Invalid month string".to_string()),
        }
    }
}

impl TryFrom<u8> for Month {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1  => Ok(Self::Jan),
            2  => Ok(Self::Feb), 
            3  => Ok(Self::Mar),
            4  => Ok(Self::Apr),
            5  => Ok(Self::May),
            6  => Ok(Self::Jun),
            7  => Ok(Self::Jul),
            8  => Ok(Self::Aug),
            9  => Ok(Self::Sep),
            10 => Ok(Self::Oct),
            11 => Ok(Self::Nov),
            12 => Ok(Self::Dec),
            _  => Err(format!("Invalid month number: {}", value)), 
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jan => write!(f, "January"),
            Self::Feb => write!(f, "February"),
            Self::Mar => write!(f, "March"),
            Self::Apr => write!(f, "April"),
            Self::May => write!(f, "May"),
            Self::Jun => write!(f, "June"),
            Self::Jul => write!(f, "July"),
            Self::Aug => write!(f, "August"),
            Self::Sep => write!(f, "September"),
            Self::Oct => write!(f, "October"),
            Self::Nov => write!(f, "November"),
            Self::Dec => write!(f, "December"),
        }
    }
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