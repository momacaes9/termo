//! Funções para ler dados do teclado com validação.
//!
//! No Python, `int(input())` quebra o programa se o usuário digitar uma letra.
//! Aqui cada função repete a pergunta até receber um valor válido.

use std::io::{self, Write};

/// Mostra `prompt` e devolve a linha digitada, sem espaços nas pontas.
/// Se a entrada acabar (Ctrl+D), encerra o programa.
pub fn ler_linha(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("falha ao escrever no terminal");

    let mut linha = String::new();
    let lidos = io::stdin().read_line(&mut linha).unwrap_or(0);
    if lidos == 0 {
        println!("\nFinalizando o programa. Obrigado!");
        std::process::exit(0);
    }
    linha.trim().to_string()
}

/// Converte texto em número aceitando vírgula ou ponto como separador decimal.
pub fn para_decimal(texto: &str) -> Option<f64> {
    let valor: f64 = texto.trim().replace(',', ".").parse().ok()?;
    if valor.is_finite() {
        Some(valor)
    } else {
        None
    }
}

pub fn ler_inteiro_entre(prompt: &str, min: i64, max: i64) -> i64 {
    loop {
        match ler_linha(prompt).parse::<i64>() {
            Ok(n) if n >= min && n <= max => return n,
            _ => println!("Valor inválido!!! Digite um número de {min} a {max}."),
        }
    }
}

pub fn ler_decimal(prompt: &str) -> f64 {
    loop {
        match para_decimal(&ler_linha(prompt)) {
            Some(v) => return v,
            None => println!("Valor inválido! Digite um número (ex.: 12.5)."),
        }
    }
}

/// Igual a `ler_decimal`, mas deixar vazio devolve `None`
/// (equivale ao "Deixe vazio se não souber" do Python).
pub fn ler_decimal_opcional(prompt: &str) -> Option<f64> {
    loop {
        let texto = ler_linha(prompt);
        if texto.is_empty() {
            return None;
        }
        match para_decimal(&texto) {
            Some(v) => return Some(v),
            None => println!("Valor inválido! Digite um número ou deixe vazio."),
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn aceita_virgula_e_ponto() {
        assert_eq!(para_decimal("12,5"), Some(12.5));
        assert_eq!(para_decimal(" 3.0 "), Some(3.0));
        assert_eq!(para_decimal("-4"), Some(-4.0));
    }

    #[test]
    fn rejeita_texto_invalido() {
        assert_eq!(para_decimal("abc"), None);
        assert_eq!(para_decimal(""), None);
        assert_eq!(para_decimal("inf"), None);
    }
}
