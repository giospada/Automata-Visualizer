use super::NFA;

use nom::{
    bytes::complete::take,
    bytes::complete::take_while1,
    bytes::streaming::tag,
    character::complete::{char, one_of},
    character::complete::{digit1, multispace0},
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn decimal(input: &str) -> IResult<&str, &str> {
    take_while1(move |c: char| c.is_digit(10))(input)
}

fn str_to_usize(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input, 10)
}

fn read_num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<usize>())(input)
}

fn custom_label<'a>(
    label: &str,
) -> impl Fn(&'a str) -> IResult<&'a str, (&'a str, &'a str, &'a str, &'a str)> + '_ {
    move |input| tuple((tag(label), multispace0, tag(":"), multispace0))(input)
}

fn read_start_stete(input: &str) -> IResult<&str, usize> {
    preceded(custom_label("start_state"), read_num)(input)
}
fn read_num_states(input: &str) -> IResult<&str, usize> {
    preceded(custom_label("num_states"), read_num)(input)
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

fn read_char(input: &str) -> IResult<&str, char> {
    preceded(
        multispace0,
        delimited(char('\''), one_of(ALPHABET), char('\'')),
    )(input)
}

fn read_num_array(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("["),
        separated_list0(tag(","), delimited(multispace0, read_num, multispace0)),
        tag("]"),
    )(input)
}

fn read_finish_states(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(custom_label("num_states"), read_num_array)(input)
}

fn read_transiction(input: &str) -> IResult<&str, (usize, usize, char)> {
    // 1 -- 'a' --> 3
    let (input, from) = delimited(multispace0, read_num, multispace0)(input)?;
    let (input, _) = tag("--")(input)?;
    let (input, ch) = opt(read_char)(input)?;
    let (input, _) = preceded(multispace0, tag("-->"))(input)?;
    let (input, to) = preceded(multispace0, read_num)(input)?;
    //TODO aggiungere espilon
    Ok((input, (from, to, ch.unwrap_or('ε'))))
}

impl From<String> for NFA {
    fn from(input: String) -> Self {
        //let (input,start_state) = start_state(input.as_str())?;
        //let (inuput,num_states) ;
        //let (input,end_states);
        Self {
            start_state: todo!(),
            num_states: todo!(),
            end_states: todo!(),
            transitions: todo!(),
            used_alphabet: todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_edge() {
        assert_eq!(read_transiction(" 1 ----> 3"), Ok(("", (1, 3, 'ε'))));
        assert_eq!(read_transiction(" 2 -- 'a'--> 3"), Ok(("", (2, 3, 'a'))));
        assert_eq!(read_transiction(" 2-- 'a' --> 3  "), Ok((" ", (2, 3, 'a'))));
        assert_eq!(read_transiction("2 ----> 3"), Ok(("", (2, 3, 'ε'))));
    }

    #[test]
    fn test_number_list() {
        assert_eq!(
            read_num_array("[ 10, 20 , 30 ]"),
            Ok(("", vec![10, 20, 30]))
        );
    }
    #[test]
    fn test_number() {
        assert_eq!(read_num("10"), Ok(("", 10)));
        assert_eq!(read_num("1234"), Ok(("", 1234)));
    }

    #[test]
    fn test_label() {
        assert_eq!(
            custom_label("start_state")("start_state  : 20"),
            Ok(("20", ("start_state", "  ", ":", " ")))
        );
        assert_eq!(
            custom_label("start_state")("start_state: 30"),
            Ok(("30", ("start_state", "", ":", " ")))
        );
        assert_eq!(
            custom_label("start_state")("start_state : 10"),
            Ok(("10", ("start_state", " ", ":", " ")))
        );
        assert_eq!(
            custom_label("start_state")("start_state :10"),
            Ok(("10", ("start_state", " ", ":", "")))
        );
        assert_eq!(read_start_stete("start_state : 100\n"), Ok(("\n", 100)));
    }
}
