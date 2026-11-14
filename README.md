# termo — Calculadora de Princípios da Termodinâmica

Projeto final (capstone) da disciplina de **Introdução à Computação — Universidade Presbiteriana Mackenzie**.

Um programa de terminal com menu para estudar e calcular:

| Opção | Tema | O que calcula |
|---|---|---|
| 1 | **Primeira Lei** (Conservação de Energia) | Q, W ou ΔU a partir de `Q = ΔU + W`; conversão cal → J |
| 2 | **Transformações** (Leis dos Gases) | Isotérmica (Boyle-Mariotte), Isobárica (Charles), Isocórica (Gay-Lussac), Adiabática |
| 3 | **Segunda Lei** (Máquinas Térmicas) | Rendimento de Carnot de refrigeradores/aquecedores e classificação BOM / MEDIANO / RUIM |
| 4 | **Máquina Centrífuga** *(só na versão Rust)* | Força, rotações e potência nos modos Turbo/Eco e capacidades P/G |

## Estrutura

```
python/   projeto original, exatamente como entregue (Google Colab)
rust/     a mesma calculadora reescrita em Rust
```

## Versão em Python (original)

```bash
python3 python/projeto_de_programacao.py
```

## Versão em Rust

Precisa do [Rust](https://www.rust-lang.org/tools/install) instalado.

```bash
cd rust
cargo run      # roda a calculadora
cargo test     # roda os testes automáticos das fórmulas
```

### O que muda na versão Rust

- **Menu em loop**: depois de cada cálculo volta ao menu (no Python o programa terminava).
- **Entradas validadas**: digitar uma letra não derruba o programa; aceita vírgula ou ponto decimal.
- **Dados faltando**: na Primeira Lei e nas Leis dos Gases, deixar mais de um campo vazio mostra uma mensagem em vez de dar erro.
- **Adiabática real**: usa `P·V^γ = constante` e `W = (P1·V1 − P2·V2)/(γ − 1)`, mostrando também o cálculo simplificado do projeto original para comparação.
- **Segunda Lei**: usa 273,15 para converter °C → K, impede fonte quente mais fria que a fria, e mostra o COP ideal dos refrigeradores.
- **Centrífuga**: a calculadora que existia no Python mas não estava no menu agora é a opção 4.
- **Testes**: cada fórmula tem testes unitários (`cargo test`).
