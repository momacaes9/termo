//! Calculadora de Princípios da Termodinâmica — versão em Rust.
//! Porte do projeto em Python (pasta `python/`) com o mesmo menu.

mod centrifuga;
mod entrada;
mod primeira_lei;
mod segunda_lei;
mod transformacoes;

const MENU: &str = "
Se deseja estudar a Primeira Lei (Conservação de Energia) - Digite 1
Se deseja estudar as Transformações (Leis dos Gases) - Digite 2
Se deseja estudar a Segunda Lei (Entropia e Máquinas Térmicas) - Digite 3
Se deseja usar a Calculadora da Máquina Centrífuga - Digite 4
Digite 0 para sair.
> ";

fn main() {
    println!("MENU DE ESCOLHA - SEJA BEM-VINDO A CALCULADORA DE PRINCIPIOS DA TERMODINÂMICA!");

    // Diferente do Python, o menu volta depois de cada cálculo.
    loop {
        println!("\nEscolha o Segmento que deseja estudar:");
        match entrada::ler_inteiro_entre(MENU, 0, 4) {
            0 => {
                println!("Finalizando o programa. Obrigado!");
                break;
            }
            1 => primeira_lei::executar(),
            2 => transformacoes::executar(),
            3 => segunda_lei::executar(),
            _ => centrifuga::executar(),
        }
    }
}
