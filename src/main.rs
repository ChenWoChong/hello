use nom::IResult;
use std::error::Error;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

fn main() -> Result<(), Box<dyn Error>> {
    let str = "abcdefg";
    let (remainning_input, output) = do_nothing_parser(str)?;
    assert_eq!(remainning_input, str);
    assert_eq!(output, "");
    Ok(())
}
