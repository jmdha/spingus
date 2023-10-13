use crate::shared::{remove_comments, spaced};
use crate::term::{parse_term, Term};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;

pub type SASPlan = Vec<Term>;

pub fn parse_sas(input: &str) -> Result<SASPlan, String> {
    let clean = remove_comments(input);
    let result = many0(delimited(spaced(char('(')), parse_term, spaced(char(')'))))(&clean);
    match result {
        Ok(result) => Ok(result.1),
        Err(str) => Err(str.to_string()),
    }
}

pub fn export_sas(input: &SASPlan) -> String {
    let mut s = "".to_string();
    for step in input {
        s += "(";
        s += &step.name;
        for parameter in &step.parameters {
            s += " ";
            s += &parameter;
        }
        s += ")\n"
    }
    s
}

#[cfg(test)]
mod test {
    use crate::{sas_plan::parse_sas, term::Term};

    #[test]
    fn parse_empty() {
        assert_eq!(Ok(vec![]), parse_sas(""));
    }

    #[test]
    fn parse_comment() {
        assert_eq!(Ok(vec![]), parse_sas("; noise"));
    }

    #[test]
    fn parse_single() {
        assert_eq!(
            Ok(vec![Term {
                name: "name".to_string(),
                parameters: vec!["param".to_string()]
            }]),
            parse_sas("(NAME PARAM)")
        );
    }

    #[test]
    fn parse_multiple() {
        assert_eq!(
            Ok(vec![
                Term {
                    name: "name1".to_string(),
                    parameters: vec!["param1".to_string()]
                },
                Term {
                    name: "name2".to_string(),
                    parameters: vec!["param2".to_string(), "param3".to_string()]
                }
            ]),
            parse_sas("(NAME1 PARAM1)\n(NAME2 PARAM2 PARAM3)")
        );
    }
}
