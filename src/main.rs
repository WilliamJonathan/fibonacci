use std::io;
fn main() {
    println!("Digite para saber o n-ésimo numero de Fibonacci:");
    
    let mut numero = String::new();
    
    io::stdin()
        .read_line(&mut numero)
        .expect("Falha ao ler o numero");
    
    let numero: u32 = numero.trim().parse()
        .expect("Por favor digite um numero");

    let resultado = formula(numero);
    
    println!("O numero de Fibonacci desta posição é: {}", resultado);
}

// Formula para encontrar o n-ésimo número de Fibonacci
fn formula(n: u32) -> f64 {
    let raiz_de_cinco  =  (5 as f64).sqrt();
    let primeira_parte =  (1 as f64 / raiz_de_cinco) * ((1 as f64 + raiz_de_cinco) / 2 as f64 );
    let primeira_parte =  primeira_parte.powf(n as f64);

    println!("Primeira parte: {}", primeira_parte);

    let segunda_parte  =  (1 as f64  / raiz_de_cinco) * ((1 as f64  - raiz_de_cinco) / 2 as f64 );
    let segunda_parte  =  segunda_parte.powf(n as f64);

    let resultado = primeira_parte - segunda_parte;

    return resultado;
}
