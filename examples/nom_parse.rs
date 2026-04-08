use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::map,
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult, Parser,
};

// Парсим число с возможными пробелами
fn parse_number(input: &str) -> IResult<&str, i32> {
    preceded(space0, map(digit1, |s: &str| s.parse::<i32>().unwrap())).parse(input)
}

// factor = number | '(' expr ')'
fn parse_factor(input: &str) -> IResult<&str, i32> {
    alt((
        parse_number,
        delimited(
            preceded(space0, char('(')),
            parse_expr,
            preceded(space0, char(')')),
        ),
    ))
    .parse(input)
}

// term = factor (('*' | '/') factor)*
fn parse_term(input: &str) -> IResult<&str, i32> {
    let (input, init) = parse_factor(input)?;

    let (input, ops) = many0(pair(
        preceded(space0, alt((char('*'), char('/')))),
        parse_factor,
    ))
    .parse(input)?;

    // Сворачиваем результат вручную
    let result = ops.into_iter().fold(init, |acc, (op, val)| match op {
        '*' => acc * val,
        '/' => acc / val,
        _ => unreachable!(),
    });

    Ok((input, result))
}

// expr = term (('+' | '-') term)*
fn parse_expr(input: &str) -> IResult<&str, i32> {
    let (input, init) = parse_term(input)?;

    let (input, ops) = many0(pair(
        preceded(space0, alt((char('+'), char('-')))),
        parse_term,
    ))
    .parse(input)?;

    // Сворачиваем результат вручную
    let result = ops.into_iter().fold(init, |acc, (op, val)| match op {
        '+' => acc + val,
        '-' => acc - val,
        _ => unreachable!(),
    });

    Ok((input, result))
}

fn main() {
    let tests = [
        "1 + 2 * (3 + 4)",   // 15
        "10 + 20",           // 30
        "2 * 3 + 4",         // 10
        "2 * (3 + 4)",       // 14
        "100 / 5 / 2",       // 10
        "(1 + 2) * (3 + 4)", // 21
        "42",                // 42
    ];

    for input in tests {
        match parse_expr(input) {
            Ok((remaining, result)) => {
                println!("{} = {} (остаток: '{}')", input, result, remaining);
            }
            Err(e) => println!("Ошибка парсинга '{}': {:?}", input, e),
        }
    }
}
