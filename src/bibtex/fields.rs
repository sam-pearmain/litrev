use std::str::FromStr; 
use std::convert::TryFrom;

use super::parser::BibTeXParser;

#[derive(Debug, PartialEq)]
pub enum BibTeXField {
    Address(String), Annote(String), Author(Authors), BookTitle(String), 
    Chapter(u8), Crossref(String), Doi(String), Edition(u16), Editor(Authors), 
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

pub enum Day {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun,
}

impl FromStr for Day {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "1" | "mon" | "monday"    => Ok(Self::Mon), 
            "2" | "tue" | "tuesday"   => Ok(Self::Tue), 
            "3" | "wed" | "wednesday" => Ok(Self::Wed), 
            "4" | "thu" | "thursday"  => Ok(Self::Thu), 
            "5" | "fri" | "friday"    => Ok(Self::Fri), 
            "6" | "sat" | "saturday"  => Ok(Self::Sat), 
            "7" | "sun" | "sunday"    => Ok(Self::Sun), 
            _ => Err(format!("invalid day string: {s}"))
        }
    }
}

macro_rules! impl_try_from_for_day {
    ( $( $t:ty ),* ) => {
        $(
            impl TryFrom<$t> for Day {
                type Error = String;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value {
                        1 => Ok(Day::Mon),
                        2 => Ok(Day::Tue),
                        3 => Ok(Day::Wed),
                        4 => Ok(Day::Thu),
                        5 => Ok(Day::Fri),
                        6 => Ok(Day::Sat),
                        7 => Ok(Day::Sun),
                        _ => Err(format!("invalid day number: {}", value)),
                    }
                }
            }
        )*
    };
}

impl_try_from_for_day!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mon => write!(f, "monday"), 
            Self::Tue => write!(f, "tuesday"), 
            Self::Wed => write!(f, "wednesday"), 
            Self::Thu => write!(f, "thursday"), 
            Self::Fri => write!(f, "friday"), 
            Self::Sat => write!(f, "saturday"), 
            Self::Sun => write!(f, "sunday"),
        }
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
            _ => Err(format!("invalid month string: {s}")),
        }
    }
}

macro_rules! impl_try_from_for_month {
    ( $( $t:ty ),* ) => {
        $(
            impl TryFrom<$t> for Month {
                type Error = String;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value {
                        1 => Ok(Self::Jan),
                        2 => Ok(Self::Feb),
                        3 => Ok(Self::Mar),
                        4 => Ok(Self::Apr),
                        5 => Ok(Self::May),
                        6 => Ok(Self::Jun),
                        7 => Ok(Self::Jul),
                        8 => Ok(Self::Aug),
                        9 => Ok(Self::Sep),
                        10 => Ok(Self::Oct),
                        11 => Ok(Self::Nov),
                        12 => Ok(Self::Dec),
                        _ => Err(format!("invalid month number: {}", value)),
                    }
                }
            }
        )*
    };
}

impl_try_from_for_month!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);


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