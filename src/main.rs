use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::i32;
use nom::combinator::map_res;
use nom::sequence::{delimited, separated_pair};
use nom::{IResult, Parser};
use std::error::Error;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

fn parse_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let str = "abcdefg";
    let (remainning_input, output) = do_nothing_parser(str)?;
    assert_eq!(remainning_input, str);
    assert_eq!(output, "");

    let (leftover_input, output) = parse_input(str)?;
    assert_eq!(leftover_input, "defg");
    assert_eq!(output, "abc");

    let (_, parsed) = parse_coordinate("(3, 5)")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate("(2, -4)")?;
    assert_eq!(parsed, Coordinate { x: 2, y: -4 });

    let parsing_error = parse_coordinate("(2,)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("(,3)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("Ferris");
    assert!(parsing_error.is_err());

    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(", "), i32).parse(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, (x, y)) = delimited(tag("("), parse_integer_pair, tag(")")).parse(input)?;
    Ok((remaining, Coordinate { x, y }))
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

pub fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

pub fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

pub fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223
            }
        ))
    );
}
