use nom::{character::complete::digit1, IResult, Parser};

fn parse_number(input: &str) -> IResult<&str, i32> {
    digit1.map(|s: &str| s.parse::<i32>().unwrap()).parse(input)
}

fn main() {
    let input = "1 + 2 * (3 + 4)";
    fn next_num(input: &str) {
        match parse_number(input) {
            Ok((remaining, result)) => {
                println!("Результат: {}", result); // 15
                println!("Остаток: '{:?}'", next_num(remaining));
            }
            Err(e) => println!("Ошибка: {:?}", e),
        }
    }
}
