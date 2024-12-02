use nom::{character::complete::line_ending, multi::separated_list1, IResult, Parser};

pub mod number;

pub fn parse_input<'a, F, O>(input: &'a str, f: F) -> IResult<&'a str, Vec<O>>
where
    F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    separated_list1(line_ending, f)(input)
}
