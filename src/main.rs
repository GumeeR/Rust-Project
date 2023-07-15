fn main() {
    let numero_1 = 123;  
    let numero_2 = 123;

    let suma = numero_1 + numero_2;
    
    loop {
        //mostrar los dos numeros en consola
        println!("primer numero {}, segundo numero {}", numero_1, numero_2);
        
        //Obtener del usuario el numero que represnta la suma
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();
        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();
        
        
        if suma_usuario_int == suma {
            println!("La suma es correcta 🦝");
            break;
        } else {
            println!("La suma es incorrecta 🦝. Respuesta correcta: {}", suma)
        }
    }
}