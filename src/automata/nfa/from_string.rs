use std::collections::BTreeMap;


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
) -> impl Fn(&'a str) -> IResult<&'a str, (&'a str, &'a str, &'a str, &'a str, &'a str)> + '_ {
    move |input| tuple((multispace0, tag(label), multispace0, tag(":"), multispace0))(input)
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
    let (input, from) = delimited(multispace0, read_num, multispace0)(input)?;
    let (input, _) = tag("--")(input)?;
    let (input, ch) = opt(read_char)(input)?;
    let (input, _) = preceded(multispace0, tag("-->"))(input)?;
    let (input, to) = preceded(multispace0, read_num)(input)?;

    Ok((input, (from, to, ch.unwrap_or('ε'))))
}

fn read_all_transictions(input: &str) -> IResult<&str, Vec<(usize, usize, char)>> {
    separated_list0(multispace0, read_transiction)(input)
}

impl TryFrom<&str> for NFA {
    // TODO: maybe we can change it
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let out = tuple((
            read_start_stete,
            read_finish_states,
            read_num_states,
            read_all_transictions,
        ))(value);

        if let Ok((input, (start_state, end_states, num_states, all_transitions))) = out {
            let mut transitions: Vec<BTreeMap<char, Vec<usize>>> =
                (0..num_states).map(|_| BTreeMap::new()).collect();
            for (from, to, ch) in all_transitions.into_iter() {
                if from >= num_states {
                    return Err(String::from("from out of num_state"));
                } else if to >= num_states {
                    return Err(String::from("to out of num_state"));
                } else {
                    transitions[from]
                        .entry(ch)
                        .and_modify(|x| x.push(to))
                        .or_insert(vec![to]);
                }
            }
            Ok(Self {
                start_state,
                num_states,
                end_states,
                transitions,
                used_alphabet: ALPHABET.chars().collect(),
            })
        } else {
            Err(String::from("error while reading nfa"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_to_nfa() {
        //todo
        NFA::try_from(concat!(
            "start_state: 0\n",
            "finish_states: [ 2 ]\n",
            " num_states: 5\n",
            "0 --'b'--> 1\n",
            "1 ----> 2\n",
            "1 ----> 3\n",
            "3 -- 'a' --> 4\n",
            "4 ----> 3\n",
            "4 ----> 2\n"
        ));
    }

    #[test]
    fn test_edge() {
        assert_eq!(read_transiction(" 1 ----> 3"), Ok(("", (1, 3, 'ε'))));
        assert_eq!(read_transiction(" 2 -- 'a'--> 3"), Ok(("", (2, 3, 'a'))));
        assert_eq!(read_transiction(" 2-- 'a' --> 3 "), Ok((" ", (2, 3, 'a'))));
        assert_eq!(read_transiction("2 ----> 3"), Ok(("", (2, 3, 'ε'))));
    }

    #[test]
    fn test_start_state() {
        assert_eq!(read_start_stete("start_state : 100\n"), Ok(("\n", 100)));
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
            Ok(("20", ("", "start_state", "  ", ":", " ")))
        );
        assert_eq!(
            custom_label("start_state")("   start_state: 30"),
            Ok(("30", ("   ", "start_state", "", ":", " ")))
        );
        assert_eq!(
            custom_label("start_state")("start_state : 10"),
            Ok(("10", ("", "start_state", " ", ":", " ")))
        );
        assert_eq!(
            custom_label("start_state")("start_state :10"),
            Ok(("10", ("", "start_state", " ", ":", "")))
        );
    }
}
