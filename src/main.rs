use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
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
    separated_pair(i32, tag(", "), i32)(input)
}
// fn parse_coordinate(input: &str) -> IResult<&str, >

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, (x, y)) = delimited(tag("("), parse_integer_pair, tag(")"))(input)?;
    Ok((remaining, Coordinate { x, y }))
}
