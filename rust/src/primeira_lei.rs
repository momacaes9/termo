//! 1. Primeira Lei da Termodinâmica (Conservação de Energia): Q = ΔU + W

use crate::entrada::ler_decimal_opcional;

pub fn converter_joule(calorias: f64) -> f64 {
    calorias * 4.184
}

/// ΔU = Q - W
pub fn calc_du(q: f64, w: f64) -> f64 {
    q - w
}

/// W = Q - ΔU
pub fn calc_w(q: f64, du: f64) -> f64 {
    q - du
}

/// Q = ΔU + W
pub fn calc_q(du: f64, w: f64) -> f64 {
    du + w
}

#[derive(Debug, PartialEq)]
pub enum Resultado {
    Calor(f64),
    Trabalho(f64),
    VariacaoInterna(f64),
    ConversaoJoule(f64),
    DadosInsuficientes,
}

/// Decide qual grandeza calcular a partir das que o usuário informou.
/// No Python, deixar duas vazias causava erro (`None + float`);
/// aqui esse caso vira `DadosInsuficientes`.
pub fn resolver(q: Option<f64>, w: Option<f64>, du: Option<f64>) -> Resultado {
    match (q, w, du) {
        (None, Some(w), Some(du)) => Resultado::Calor(calc_q(du, w)),
        (Some(q), None, Some(du)) => Resultado::Trabalho(calc_w(q, du)),
        (Some(q), Some(w), None) => Resultado::VariacaoInterna(calc_du(q, w)),
        (Some(q), Some(_), Some(_)) => Resultado::ConversaoJoule(converter_joule(q)),
        _ => Resultado::DadosInsuficientes,
    }
}

pub fn executar() {
    println!("1. Primeira Lei (Conservação de Energia)");
    println!("   Q = ΔU + W\n");

    let q = ler_decimal_opcional("Insira a quantidade de Calor (Deixe vazio se não souber): ");
    let w = ler_decimal_opcional(
        "Insira a quantidade de Trabalho/Energia trocada (Deixe vazio se não souber): ",
    );
    let du =
        ler_decimal_opcional("Insira a Quantidade de Variação Interna (Deixe vazio se não souber): ");

    println!("{}", "-".repeat(30));
    match resolver(q, w, du) {
        Resultado::Calor(v) => println!("O valor da Quantidade de calor (Q) é: {v} J"),
        Resultado::Trabalho(v) => println!("O valor de Energia trocada/Trabalho (W) é: {v} J"),
        Resultado::VariacaoInterna(v) => {
            println!("O valor da Variação de Energia Interna (ΔU) é: {v} J")
        }
        Resultado::ConversaoJoule(v) => {
            println!("Conversão do calor inserido (cal) para Joule: {v} J")
        }
        Resultado::DadosInsuficientes => {
            println!("Dados insuficientes. Informe pelo menos 2 dos 3 valores.")
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn calcula_cada_grandeza() {
        assert_eq!(resolver(None, Some(30.0), Some(70.0)), Resultado::Calor(100.0));
        assert_eq!(resolver(Some(100.0), None, Some(70.0)), Resultado::Trabalho(30.0));
        assert_eq!(resolver(Some(100.0), Some(30.0), None), Resultado::VariacaoInterna(70.0));
    }

    #[test]
    fn tudo_preenchido_converte_para_joule() {
        assert_eq!(
            resolver(Some(10.0), Some(1.0), Some(1.0)),
            Resultado::ConversaoJoule(41.84)
        );
    }

    #[test]
    fn faltando_dois_valores_nao_quebra() {
        assert_eq!(resolver(Some(10.0), None, None), Resultado::DadosInsuficientes);
        assert_eq!(resolver(None, None, None), Resultado::DadosInsuficientes);
    }
}
