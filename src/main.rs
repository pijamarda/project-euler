fn main() {
    println!("Hello, world!");
    let respuesta1 = problema1();    
    println!("Problema 1 {}", respuesta1);
}

fn problema1() -> &'static str {
    let respuesta = "Respuesta";
    return &respuesta;
}
