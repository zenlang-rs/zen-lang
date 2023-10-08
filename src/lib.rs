// XML Parser

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String, // Name of the Tag
    attributes: Vec<(String, String)>, // (identifier, value)
    children :Vec<Element>,
}

fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())), // Advance by 1-grapheme steps
        _ => Err(input),
    }
}


// Returns a closure function, rather than the text
fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str>
{
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
    }
}

/// First character is alphabetic and next other can be alphanumeric or '-'
fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        }
        else {
            break;
        }
    }

    let next_index = matched.len();

    Ok((&input[next_index..], matched))
}

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));

    assert_eq!(Ok((" Hello Robert!", ())), parse_joe("Hello Joe! Hello Robert!"));

    assert_eq!(Err("Hello Mike!"), parse_joe("Hello Mike!"));
}

#[test]
fn identifier_parser() {
    assert_eq!(
        Ok(("", "i-am-an-identifier".to_string())),
        identifier("i-am-an-identifier")
    );

    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );

    assert_eq!(
        Err("!not at all an identifier"),
        identifier("!not at all an identifier")
    );

}
