//! 4. Calculadora da Máquina Industrial Modelo Centrífuga
//! (no Python essa parte existia mas nunca era chamada pelo menu).

use crate::entrada::{ler_decimal, ler_linha};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Modo {
    Turbo,
    Eco,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Capacidade {
    Pequena,
    Grande,
}

/// Modo Eco reduz as rotações em 50%; capacidade Pequena reduz a força em 50%.
pub fn aplicar_modificadores(forca: f64, rotacoes: f64, modo: Modo, cap: Capacidade) -> (f64, f64) {
    let rotacoes = if modo == Modo::Eco { rotacoes * 0.5 } else { rotacoes };
    let forca = if cap == Capacidade::Pequena { forca * 0.5 } else { forca };
    (forca, rotacoes)
}

/// Mesmo modelo do projeto em Python: potência = força × rotações.
pub fn calcular_potencia(forca: f64, rotacoes: f64) -> f64 {
    forca * rotacoes
}

fn selecionar_modo() -> Modo {
    loop {
        match ler_linha("Selecione o modo (Turbo ou Eco): ").to_lowercase().as_str() {
            "turbo" => return Modo::Turbo,
            "eco" => return Modo::Eco,
            _ => println!("Inválido! Digite Turbo ou Eco."),
        }
    }
}

fn selecionar_capacidade() -> Capacidade {
    loop {
        match ler_linha("Selecione a capacidade industrial (P = Pequena / G = Grande): ")
            .to_uppercase()
            .as_str()
        {
            "P" => return Capacidade::Pequena,
            "G" => return Capacidade::Grande,
            _ => println!("Inválido! Digite P ou G."),
        }
    }
}

pub fn executar() {
    println!("4. Máquina Industrial Modelo Centrífuga\n");

    let forca_base = ler_decimal("Digite a força base (N): ");
    let rotacoes_base = ler_decimal("Digite as rotações base (RPM): ");
    let modo = selecionar_modo();
    let cap = selecionar_capacidade();

    let (forca, rotacoes) = aplicar_modificadores(forca_base, rotacoes_base, modo, cap);
    let potencia = calcular_potencia(forca, rotacoes);

    println!("\n--- Resultado ---");
    println!("Força aplicada:   {forca:.2} N");
    println!("Rotações:         {rotacoes:.2} RPM");
    println!("Potência gerada:  {potencia:.2} W");
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn modificadores() {
        assert_eq!(aplicar_modificadores(100.0, 1000.0, Modo::Turbo, Capacidade::Grande), (100.0, 1000.0));
        assert_eq!(aplicar_modificadores(100.0, 1000.0, Modo::Eco, Capacidade::Grande), (100.0, 500.0));
        assert_eq!(aplicar_modificadores(100.0, 1000.0, Modo::Eco, Capacidade::Pequena), (50.0, 500.0));
    }

    #[test]
    fn potencia() {
        assert_eq!(calcular_potencia(50.0, 500.0), 25_000.0);
    }
}
