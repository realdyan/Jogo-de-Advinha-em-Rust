use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {


    println!("Jogo de advinha! Tente adivinhar o número entre 1 e 100.");
    let numero_secreto: i32 = rand::thread_rng().gen_range(1..=100);
    let mut tentativas = 0;

    loop {
        tentativas += 1;
        println!("\nInsira seu palpite (ou 'q' para sair):");

        let mut tentativa = String::new();
        if io::stdin().read_line(&mut tentativa).is_err() {
            println!("Falha ao ler a linha. Tente novamente.");
            continue;
        }

        if tentativa.len() > 128 {
            println!("Entrada muito longa. Tente algo mais curto.");
            continue;
        }

        let entrada = tentativa.trim();
        if entrada.eq_ignore_ascii_case("q") {
            println!("Saindo. O número era {}.", numero_secreto);
            break;
        }

        if entrada.is_empty() {
            println!("Entrada vazia. Digite um número.");
            continue;
        }

        // Parse seguro para i32 (evita panic em "-1") e validação de intervalo
        let numero_i32: i32 = match entrada.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Por favor, insira um número válido (1-100).");
                continue;
            }
        };

        if numero_i32 < 1 || numero_i32 > 100 {
            println!("Número fora do intervalo. Use 1 a 100.");
            continue;
        }

        println!("Você escolheu: {}", numero_i32);

        match numero_i32.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Parabéns! Você acertou em {} tentativas.", tentativas);
                break;
            }
        }
    }

    println!("Deseja jogar novamente? (s/n):");
    let mut resposta = String::new();
    if io::stdin().read_line(&mut resposta).is_err() {
        println!("Falha ao ler a linha. Encerrando o jogo.");
        break;
    }

    let resposta = resposta.trim();
    if resposta.eq_ignore_ascii_case("s") {
        continue;
    } else {
        println!("Obrigado por jogar!");
        break;
    }
}
}