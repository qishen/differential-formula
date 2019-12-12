extern crate num;
use crate::term::*;
use crate::expression::*;

use std::str::FromStr;
use std::sync::Arc;
use nom::IResult;
use nom::character::streaming::*;
use nom::number::complete::*;

use num::*;

named!(composite<&str, Term>, 
    do_parse!(
        t: alphanumeric1 >>
        args: delimited!(
            char!('('), 
            separated_list!(tag!(","), alt!(composite | atom)), 
            char!(')')
        ) >>
        (parse_term(t, args))
    )
);

named!(atom<&str, Term>,
    alt!(atom_string | atom_bool | atom_integer | atom_float)
);

named!(atom_integer<&str, Term>,
    map!(
        tuple!(opt!(alt!(char!('+') | char!('-'))), digit1),
        |(sign, num_str)| {
            let num = match sign {
                Some(sign_char) => { sign_char.to_string() + &num_str.to_string() },
                None => { num_str.to_string() }
            };

            Atom::Int(BigInt::from_str(&num[..]).unwrap()).into() 
        }
    )
);

named!(atom_string<&str, Term>,
    map!(
        delimited!(char!('"'), alphanumeric0, char!('"')), 
        |atom_str| { Atom::Str(atom_str.to_string()).into() }
    )
);

named!(atom_float<&str, Term>,
    map!(
        float, 
        |float_num| { Atom::Float(BigRational::from_f32(float_num).unwrap()).into() }
    )
);

named!(atom_bool<&str, Term>,
    map!(
        alt!(tag!("true") | tag!("false")), 
        |x| {
            match x {
                "true" => Atom::Bool(true).into(),
                _ => Atom::Bool(false).into(),
            }
        }
    )
);


fn parse_term(sort: &str, args: Vec<Term>) -> Term {
    Composite {
        sort: x,
        arguments: args.into_iter().map(|x| Arc::new(x)).collect(),
        alias: None,
    }.into()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        let output = composite("Node(\"hi\")ttt");
        let output2 = atom("\"helloworld\"");
        let output3 = atom("123E-02");
        let output4 = atom("-11223344 ");
        println!("{:?}", output);
    }

    #[test]
    fn test_composite() {

    }
}

