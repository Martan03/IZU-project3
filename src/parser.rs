use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use indexmap::IndexMap;

use crate::object::Object;

#[derive(Debug, PartialEq, Eq)]
pub enum ParserErr {
    ReadErr,
    NotEndedBlock,
    UnexpectedFormat,
    InvalidClassName(String),
    InvalidOrder,
    InvalidObject,
}

#[derive(Debug)]
pub struct Parser {
    pub attr: IndexMap<String, Vec<String>>,
    pub class: Vec<String>,
    pub object: Vec<Object>,
}

impl Parser {
    pub fn parse(filename: &str) -> Result<Self, ParserErr> {
        let mut parser = Self {
            attr: IndexMap::new(),
            class: vec![],
            object: vec![],
        };

        let file = File::open(filename).map_err(|_| ParserErr::ReadErr)?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines().filter_map(|l| l.ok()).into_iter();
        while let Some(line) = lines.next() {
            match line.trim() {
                "attributes {" => {
                    parser.parse_block(&mut lines, Parser::parse_attr)?
                }
                "classes {" => {
                    parser.parse_block(&mut lines, Parser::parse_class)?
                }
                "objects {" => {
                    parser.parse_block(&mut lines, Parser::parse_object)?
                }
                _ => return Err(ParserErr::UnexpectedFormat),
            }
        }

        Ok(parser)
    }

    fn parse_block<T, F>(
        &mut self,
        lines: &mut T,
        parse_line: F,
    ) -> Result<(), ParserErr>
    where
        T: Iterator<Item = String>,
        F: Fn(&mut Parser, &str) -> Result<(), ParserErr>,
    {
        while let Some(line) = lines.next() {
            let trim = line.trim();
            if trim == "}" {
                return Ok(());
            }

            if trim.is_empty() {
                continue;
            }

            parse_line(self, trim)?;
        }
        Err(ParserErr::NotEndedBlock)
    }

    /// Parses attributes
    fn parse_attr(&mut self, line: &str) -> Result<(), ParserErr> {
        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            return Err(ParserErr::UnexpectedFormat);
        }

        let values: Vec<String> =
            parts[1].split_whitespace().map(|s| s.to_string()).collect();
        self.attr.insert(parts[0].to_string(), values);
        Ok(())
    }

    /// Parses classes
    fn parse_class(&mut self, line: &str) -> Result<(), ParserErr> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 1 {
            return Err(ParserErr::InvalidClassName(line.to_string()));
        }

        self.class.push(line.to_string());
        Ok(())
    }

    fn parse_object(&mut self, line: &str) -> Result<(), ParserErr> {
        if self.attr.is_empty() || self.class.is_empty() {
            return Err(ParserErr::InvalidOrder);
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 + self.attr.len() {
            return Err(ParserErr::InvalidObject);
        }

        let id = parts[0]
            .parse::<usize>()
            .map_err(|_| ParserErr::InvalidObject)?;
        let attr: Vec<String> =
            parts[2..].iter().map(|s| s.to_string()).collect();
        self.object.push(Object::new(id, parts[1], attr));

        Ok(())
    }
}
