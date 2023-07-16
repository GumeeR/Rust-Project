use regex::Regex;

fn make_operation(reg: Regex, mut expresion: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }
    loop {
        //Aplicar operaciones
        let caps = reg.captures(expresion.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();

        let cap_expresion = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expresion = expresion.replace(cap_expresion, &result.to_string());
    }
    expresion
}

fn main() {
    //Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_less = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();

    //Traer datos del usuario
    println!("Por favor introduce tu expresion");
    let mut expresion = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();

    //Multiplicacion
    expresion = make_operation(re_mult, expresion, "*");
    //Division
    expresion = make_operation(re_div, expresion, "/");
    //Suma
    expresion = make_operation(re_add, expresion, "+");
    //Resta
    expresion = make_operation(re_less, expresion, "-");

    //Mostrar resultados
    println!("Resultados: {}", expresion);
}
