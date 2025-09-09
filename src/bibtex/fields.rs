
pub enum Suffix {

}

#[derive(Debug, PartialEq)]
pub struct Name {
    name: String, 
    is_forename: bool, 
    is_abbreviated: bool, 
}

#[derive(Debug, PartialEq)]
pub struct Author {
    forename: Name, 
    surname: Name, 
    suffix: Option<String>, 
    is_lead_author: bool, 
}

#[derive(Debug)]
pub struct Authors {
    authors: Vec<Author>, 
}

#[derive(Debug, PartialEq)]
pub struct PageRange {
    start: i32, 
    end: Option<i32>, // if we just have one page then no end page is given 
}

#[derive(Debug)]
pub struct Pages {
    pages: Vec<PageRange>
}