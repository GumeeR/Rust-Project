fn main() {
    println!("     ----------");
    println!("     Tabla de datos🦝");
    println!("     ----------");

    let mut data_name:Vec<String> = Vec::new();
    let mut data_last_name:Vec<String> = Vec::new();
    let mut data_age:Vec<String> = Vec::new();
    let mut data_country:Vec<String> = Vec::new();
    let mut option_user:String = String::new();


    loop {
        let mut name:String = String::new();
        let mut last_name:String = String::new();
        let mut age:String = String::new();
        let mut country:String = String::new();

        println!("Cual es tu nombre🦝?");
        std::io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();
        data_name.push(name);


        println!("Cual es tu apellido🦝?");
        std::io::stdin().read_line(&mut last_name).unwrap();
        last_name = last_name.trim().to_string();
        data_last_name.push(last_name);

        println!("Cual es tu edad🦝?");
        std::io::stdin().read_line(&mut age).unwrap();
        age = age.trim().to_string();
        data_age.push(age);


        println!("De que pais eres🦝?");
        std::io::stdin().read_line(&mut country).unwrap();
        country = country.trim().to_string();
        data_country.push(country);
    
        loop {
            println!("----------------------------------------");
            option_user ="".trim().to_string();
            println!("Oprime 'E' para salir o 'C' para continuar...");
            std::io::stdin().read_line(&mut option_user).unwrap();
            option_user = option_user.trim().to_string();
            println!("----------------------------------------");
            if option_user == "E" || option_user == "e" || option_user == "C" || option_user == "c"{
                break;
            }
            println!("'{}': Esta opcion no valida.", option_user);

        }
        if option_user == "E" || option_user == "e"{
            
            break;
        }


    }
    println!("Nombre    Apellido    Edad   Origen");
    println!("-----------------------------------");
    for i in 0..data_name.len(){
        println!("{}    {}    {}   {}",data_name[i], data_last_name[i], data_age[i],data_country[i]);
    }
    println!("       --------");
    println!("       ADIOS.");
    println!("       --------");
}
