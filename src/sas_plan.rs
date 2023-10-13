use crate::shared::spaced;
use crate::term::{parse_term, Term};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;

pub type SASPlan = Vec<Term>;

pub fn parse_sas(input: &str) -> Result<SASPlan, String> {
    let result = many0(delimited(spaced(char('(')), parse_term, spaced(char(')'))))(&input);
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
