fn main() {
    // aviso de bienvenida.
    println!("Bienvenido Cual es tu nombre?🦝");
    // variable donde se almacena el nombre.
    let mut nombre: String = String::new();
    //librerias y input para el nombre.
    std::io::stdin().read_line(&mut nombre).unwrap();
    // recargar el texto para evitar los saltos de linea.
    nombre = nombre.trim().to_string();

    println!("Cual es tu nacionalidad?🦝");
    let mut nacionalidad: String = String::new();
    std::io::stdin().read_line(&mut nacionalidad).unwrap();
    nacionalidad = nacionalidad.trim().to_string();

    println!("Hola {} Es impresionante que seas de {}.", nombre,nacionalidad);
}
