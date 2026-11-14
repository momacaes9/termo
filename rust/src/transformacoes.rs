//! 2. Transformações Termodinâmicas (Leis dos Gases)

use crate::entrada::{ler_decimal, ler_decimal_opcional, ler_inteiro_entre};

pub fn celsius_para_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

/// Lei de Boyle-Mariotte (isotérmica): P1·V1 = P2·V2.
/// Recebe `[P1, V1, P2, V2]` com exatamente um valor `None`.
/// Devolve a posição do valor que faltava e o valor calculado.
pub fn resolver_produto(v: [Option<f64>; 4]) -> Option<(usize, f64)> {
    match v {
        [None, Some(v1), Some(p2), Some(v2)] => Some((0, p2 * v2 / v1)),
        [Some(p1), None, Some(p2), Some(v2)] => Some((1, p2 * v2 / p1)),
        [Some(p1), Some(v1), None, Some(v2)] => Some((2, p1 * v1 / v2)),
        [Some(p1), Some(v1), Some(p2), None] => Some((3, p1 * v1 / p2)),
        _ => None,
    }
}

/// Leis de Charles (isobárica) e Gay-Lussac (isocórica): X1/T1 = X2/T2.
/// Recebe `[X1, T1, X2, T2]` com exatamente um valor `None`.
pub fn resolver_razao(v: [Option<f64>; 4]) -> Option<(usize, f64)> {
    match v {
        [None, Some(t1), Some(x2), Some(t2)] => Some((0, x2 * t1 / t2)),
        [Some(x1), None, Some(x2), Some(t2)] => Some((1, x1 * t2 / x2)),
        [Some(x1), Some(t1), None, Some(t2)] => Some((2, x1 * t2 / t1)),
        [Some(x1), Some(t1), Some(x2), None] => Some((3, x2 * t1 / x1)),
        _ => None,
    }
}

/// Adiabática reversível de gás ideal: P·V^γ = constante.
/// Devolve (P2, W), onde W é o trabalho realizado PELO gás:
/// W = (P1·V1 − P2·V2) / (γ − 1).
pub fn adiabatica(p1: f64, v1: f64, v2: f64, gama: f64) -> (f64, f64) {
    let p2 = p1 * (v1 / v2).powf(gama);
    let w = (p1 * v1 - p2 * v2) / (gama - 1.0);
    (p2, w)
}

/// Versão simplificada usada no projeto em Python (pressão constante).
pub fn trabalho_simplificado(p: f64, v1: f64, v2: f64) -> f64 {
    p * (v2 - v1)
}

/// Pergunta as 4 grandezas e resolve a que ficou vazia.
fn calcular_lei(
    nomes: [&str; 4],
    unidades: [&str; 4],
    resolver: fn([Option<f64>; 4]) -> Option<(usize, f64)>,
) {
    let mut valores = [None; 4];
    for i in 0..4 {
        valores[i] = ler_decimal_opcional(&format!(
            "Insira {} em {} (Deixe vazio se não souber): ",
            nomes[i], unidades[i]
        ));
    }
    println!("{}", "-".repeat(40));

    let preenchidos = valores.iter().flatten().count();
    if preenchidos == 4 {
        println!("Todos os dados foram fornecidos — nada a calcular.");
        return;
    }
    if preenchidos < 3 {
        println!("Dados insuficientes para calcular. Forneça exatamente 3 valores.");
        return;
    }
    if valores.iter().flatten().any(|&v| v <= 0.0) {
        println!("Pressão, volume e temperatura (em K) precisam ser maiores que zero.");
        return;
    }

    match resolver(valores) {
        Some((i, resultado)) => {
            println!("O valor de {} é: {resultado:.4} {}", nomes[i], unidades[i])
        }
        None => println!("Não foi possível calcular com esses dados."),
    }
}

pub fn executar() {
    println!("2. Transformações Termodinâmicas\n");

    let t = ler_inteiro_entre(
        "\nEscolha o tipo de Transformação:\n\
         1 - Isotérmica   (temperatura constante) - Lei de Boyle-Mariotte\n\
         2 - Isobárica    (pressão constante)     - Lei de Charles\n\
         3 - Isocórica    (volume constante)      - Lei de Gay-Lussac\n\
         4 - Adiabática   (sem troca de calor)    - Q = 0\n> ",
        1,
        4,
    );

    match t {
        1 => {
            println!("\nIsotérmica — Lei de Boyle-Mariotte: P1 × V1 = P2 × V2");
            println!("(Temperatura constante, pressão e volume são inversamente proporcionais)\n");
            calcular_lei(
                ["a Pressão inicial P1", "o Volume inicial V1", "a Pressão final P2", "o Volume final V2"],
                ["Pa", "m³", "Pa", "m³"],
                resolver_produto,
            );
        }
        2 => {
            println!("\nIsobárica — Lei de Charles: V1/T1 = V2/T2");
            println!("(Pressão constante, volume e temperatura são diretamente proporcionais)");
            println!("ATENÇÃO: As temperaturas devem estar em Kelvin (K)!");
            println!("(Dica: 25 °C = {} K)\n", celsius_para_kelvin(25.0));
            calcular_lei(
                ["o Volume inicial V1", "a Temperatura inicial T1", "o Volume final V2", "a Temperatura final T2"],
                ["m³", "K", "m³", "K"],
                resolver_razao,
            );
        }
        3 => {
            println!("\nIsocórica — Lei de Gay-Lussac: P1/T1 = P2/T2");
            println!("(Volume constante, pressão e temperatura são diretamente proporcionais)");
            println!("ATENÇÃO: As temperaturas devem estar em Kelvin (K)!\n");
            calcular_lei(
                ["a Pressão inicial P1", "a Temperatura inicial T1", "a Pressão final P2", "a Temperatura final T2"],
                ["Pa", "K", "Pa", "K"],
                resolver_razao,
            );
        }
        _ => {
            println!("\nAdiabática — Sem troca de calor (Q = 0), logo W = −ΔU");
            println!("Modelo: gás ideal, processo reversível, P·V^γ = constante\n");

            let p1 = ler_decimal("Insira a Pressão inicial P1 em Pa: ");
            let v1 = ler_decimal("Insira o Volume inicial V1 em m³: ");
            let v2 = ler_decimal("Insira o Volume final V2 em m³: ");
            let gama = ler_decimal_opcional(
                "Insira γ = Cp/Cv (vazio = 1.4, ar/gás diatômico; 1.67 = monoatômico): ",
            )
            .unwrap_or(1.4);

            println!("{}", "-".repeat(40));
            if p1 <= 0.0 || v1 <= 0.0 || v2 <= 0.0 || gama <= 1.0 {
                println!("P1, V1 e V2 devem ser maiores que zero, e γ maior que 1.");
                return;
            }
            let (p2, w) = adiabatica(p1, v1, v2, gama);
            println!("Pressão final (P2): {p2:.4} Pa");
            println!("Trabalho realizado pelo gás (W): {w:.4} J");
            println!("Variação de energia interna (ΔU = −W): {:.4} J", -w);
            println!(
                "(Comparação: o cálculo simplificado do projeto em Python, P1·(V2−V1), daria {:.4} J)",
                trabalho_simplificado(p1, v1, v2)
            );
        }
    }
    println!("{}", "-".repeat(40));
}

#[cfg(test)]
mod testes {
    use super::*;

    fn perto(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9 * b.abs().max(1.0)
    }

    #[test]
    fn boyle_mariotte_resolve_qualquer_incognita() {
        // P1=100, V1=2, P2=50, V2=4
        assert_eq!(resolver_produto([None, Some(2.0), Some(50.0), Some(4.0)]), Some((0, 100.0)));
        assert_eq!(resolver_produto([Some(100.0), None, Some(50.0), Some(4.0)]), Some((1, 2.0)));
        assert_eq!(resolver_produto([Some(100.0), Some(2.0), None, Some(4.0)]), Some((2, 50.0)));
        assert_eq!(resolver_produto([Some(100.0), Some(2.0), Some(50.0), None]), Some((3, 4.0)));
    }

    #[test]
    fn charles_gay_lussac_resolve_qualquer_incognita() {
        // X1=2, T1=300, X2=4, T2=600
        assert_eq!(resolver_razao([None, Some(300.0), Some(4.0), Some(600.0)]), Some((0, 2.0)));
        assert_eq!(resolver_razao([Some(2.0), None, Some(4.0), Some(600.0)]), Some((1, 300.0)));
        assert_eq!(resolver_razao([Some(2.0), Some(300.0), None, Some(600.0)]), Some((2, 4.0)));
        assert_eq!(resolver_razao([Some(2.0), Some(300.0), Some(4.0), None]), Some((3, 600.0)));
    }

    #[test]
    fn dados_faltando_devolvem_none() {
        assert_eq!(resolver_produto([None, None, Some(1.0), Some(1.0)]), None);
        assert_eq!(resolver_razao([Some(1.0); 4]), None);
    }

    #[test]
    fn adiabatica_expansao_realiza_trabalho_positivo() {
        // Gás monoatômico (γ = 5/3) dobrando de volume.
        let (p2, w) = adiabatica(100_000.0, 1.0, 2.0, 5.0 / 3.0);
        assert!(perto(p2, 100_000.0 * 0.5_f64.powf(5.0 / 3.0)));
        assert!(w > 0.0);
        // Conferência: W = P1V1/(γ−1) · (1 − (V1/V2)^(γ−1))
        let esperado = 100_000.0 / (2.0 / 3.0) * (1.0 - 0.5_f64.powf(2.0 / 3.0));
        assert!(perto(w, esperado));
    }

    #[test]
    fn conversao_celsius() {
        assert!(perto(celsius_para_kelvin(0.0), 273.15));
    }
}
