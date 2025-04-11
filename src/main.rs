use std::io;
fn main() {
    let mut primeira_execucao = true;
    loop {
        if primeira_execucao {
            println!(
                "\n************************************************************************************************\n\
                 * Este programa calcula o n-ésimo número de Fibonacci usando a fórmula de Binet.               *\n\
                 * A fórmula é a seguinte: F(n) = (φ^n - (1 - φ)^n) / √5, onde φ é a razão áurea.               *\n\
                 * O programa aceita números inteiros positivos e retorna o número de Fibonacci correspondente. *\n\
                 * Para sair, digite 'sair'.                                                                    *\n\
            ************************************************************************************************\n"
            );
        }
        primeira_execucao = false;
        let mut numero = String::new();
        
        io::stdin()
            .read_line(&mut numero)
            .expect("\n---------------------\n\
            Falha ao ler o numero\n\
            ---------------------\n");

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
                println!("\n----------------------------------------------------\n\
                Por favor, digite apenas números inteiros positivos.\n\
                ----------------------------------------------------\n");
                continue;
            }
        };

        let resultado = formula(numero);
        
        println!("\n----------------------------------------------------\n\
        O número de Fibonacci na posição {} é: {}\n\
        ----------------------------------------------------\n", numero, resultado);
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
