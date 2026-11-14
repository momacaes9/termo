//! 3. Segunda Lei (Entropia e Máquinas Térmicas)

use crate::entrada::{ler_decimal, ler_inteiro_entre};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Maquina {
    Refrigerador,
    Aquecedor,
}

#[derive(Debug, PartialEq)]
pub enum Classificacao {
    Bom,
    Mediano,
    Ruim,
}

/// Rendimento de Carnot em %: η = (1 − Tfria/Tquente) × 100, temperaturas em K.
pub fn rendimento(t_fria_k: f64, t_quente_k: f64) -> f64 {
    (1.0 - t_fria_k / t_quente_k) * 100.0
}

/// Coeficiente de desempenho ideal de um refrigerador: COP = Tf / (Tq − Tf).
pub fn cop_refrigerador(t_fria_k: f64, t_quente_k: f64) -> f64 {
    t_fria_k / (t_quente_k - t_fria_k)
}

/// Mesmas faixas do projeto em Python.
pub fn classificar(maquina: Maquina, rend: f64) -> Classificacao {
    let (bom, mediano) = match maquina {
        Maquina::Refrigerador => (70.0, 50.0),
        Maquina::Aquecedor => (60.0, 40.0),
    };
    if rend >= bom {
        Classificacao::Bom
    } else if rend >= mediano {
        Classificacao::Mediano
    } else {
        Classificacao::Ruim
    }
}

pub fn executar() {
    println!("3. Segunda Lei (Entropia e Máquinas Térmicas)\n");

    let n = ler_inteiro_entre("Insira a quantidade de maquinários que deseja avaliar: ", 1, 1000);

    for i in 1..=n {
        println!();
        let codigo = ler_inteiro_entre(
            &format!("Insira o código do {i}º maquinário (1 = refrigerador, 2 = aquecedor): "),
            1,
            2,
        );
        let maquina = if codigo == 1 { Maquina::Refrigerador } else { Maquina::Aquecedor };
        let nome = match maquina {
            Maquina::Refrigerador => "refrigerador",
            Maquina::Aquecedor => "aquecedor",
        };
        println!("O equipamento é um {nome}");

        let (tf, tq) = loop {
            let tf = ler_decimal("Insira a temperatura da fonte fria (em °C): ") + 273.15;
            let tq = ler_decimal("Insira a temperatura da fonte quente (em °C): ") + 273.15;
            if tf <= 0.0 {
                println!("Temperatura abaixo do zero absoluto! Tente de novo.");
            } else if tq <= tf {
                println!("A fonte quente precisa estar mais quente que a fonte fria. Tente de novo.");
            } else {
                break (tf, tq);
            }
        };

        let rend = rendimento(tf, tq);
        println!("O rendimento (Carnot) dessa máquina é de {rend:.2}%");
        if maquina == Maquina::Refrigerador {
            println!("COP ideal do refrigerador: {:.2}", cop_refrigerador(tf, tq));
        }

        let texto = match classificar(maquina, rend) {
            Classificacao::Bom => "BOM",
            Classificacao::Mediano => "MEDIANO",
            Classificacao::Ruim => "RUIM",
        };
        println!("O rendimento energético de seu {nome} está {texto}");
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn rendimento_de_carnot() {
        assert_eq!(rendimento(300.0, 600.0), 50.0);
        assert_eq!(rendimento(300.0, 300.0), 0.0);
    }

    #[test]
    fn cop() {
        assert_eq!(cop_refrigerador(250.0, 300.0), 5.0);
    }

    #[test]
    fn faixas_de_classificacao() {
        use Classificacao::*;
        use Maquina::*;
        assert_eq!(classificar(Refrigerador, 70.0), Bom);
        assert_eq!(classificar(Refrigerador, 55.0), Mediano);
        assert_eq!(classificar(Refrigerador, 49.9), Ruim);
        assert_eq!(classificar(Aquecedor, 60.0), Bom);
        assert_eq!(classificar(Aquecedor, 40.0), Mediano);
        assert_eq!(classificar(Aquecedor, 10.0), Ruim);
    }
}
