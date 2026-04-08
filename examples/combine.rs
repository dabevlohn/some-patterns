use nom::{character::complete::digit1, IResult, Parser};

fn parse_number(input: &str) -> IResult<&str, i32> {
    digit1.map(|s: &str| s.parse::<i32>().unwrap()).parse(input)
}

// Рекурсивная функция — возвращает () и принимает &str
fn next_num(input: &str) {
    match parse_number(input) {
        Ok((remaining, result)) => {
            println!("Результат: {}", result);
            println!("Остаток: '{}'", remaining);

            // Рекурсивный вызов, если есть остаток
            if !remaining.is_empty() {
                next_num(remaining);
            }
        }
        Err(e) => {
            println!("Ошибка парсинга: {:?}", e);
            // Выход из рекурсии при ошибке
        }
    }
}

fn main() {
    let input = "123456789+";
    next_num(input);
}
