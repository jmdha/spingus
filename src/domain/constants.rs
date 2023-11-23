use nom::{bytes::complete::tag_no_case, IResult};

use crate::shared::spaced;

use super::parameter::{parse_parameters, Parameters};

pub fn parse_constants(input: &str) -> IResult<&str, Parameters> {
    let (remainder, _) = spaced(tag_no_case(":constants"))(input)?;
    parse_parameters(remainder)
}

#[cfg(test)]
mod test {
    use crate::domain::{constants::parse_constants, parameter::Parameter};
    #[test]
    fn parse_typed() {
        assert_eq!(
            Ok((
                "",
                vec![Parameter::Typed {
                    name: "kitchen".to_string(),
                    type_name: "place".to_string()
                }]
            )),
            parse_constants(":constants kitchen - place")
        );
    }
}
