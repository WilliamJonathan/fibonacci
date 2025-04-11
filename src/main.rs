use std::io;
fn main() {
    loop {
        println!("Digite para saber o n-ésimo numero de Fibonacci (ou digite 'sair' para encerrar):");
    
        let mut numero = String::new();
        
        io::stdin()
            .read_line(&mut numero)
            .expect("Falha ao ler o numero");

        let numero = numero.trim();

        // verifica se o usuário deseja sair
        if numero.eq_ignore_ascii_case("sair") {
            println!("Saindo...");
            break;
        }
        
        // Verifica se o número é um inteiro positivo
        let numero: u32 = match  numero.trim().parse() {
            Ok(valor) => valor,
            Err(_) => {
                println!("Por favor, digite apenas números inteiros positivos.");
                continue;
            }
        };

        let resultado = formula(numero);
        
        println!("O numero de Fibonacci desta posição é: {}", resultado);
    }
}

// Formula para encontrar o n-ésimo número de Fibonacci
fn formula(n: u32) -> u64 {
    let raiz_de_cinco  =  (5 as f64).sqrt();
    let primeira_parte =  (1.0 / raiz_de_cinco) * ((1.0 + raiz_de_cinco) / 2.0 ).powf(n as f64);

    let segunda_parte  =  (1.0  / raiz_de_cinco) * ((1.0  - raiz_de_cinco) / 2.0 ).powf(n as f64);

    let resultado = primeira_parte - segunda_parte;

    resultado.round() as u64
}
