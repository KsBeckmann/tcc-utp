# Validação diferencial do simulador

Código que produz o Quadro "Validação diferencial do simulador" do Capítulo 8 da monografia.

Para cada autômato em `automatos/`, todas as cadeias sobre o alfabeto do arquivo até um
comprimento máximo são simuladas pela Autosim 1.0.4, e o veredito é comparado com:

1. a definição da linguagem, escrita como predicado em `src/main.rs`;
2. um simulador de referência que propaga o conjunto de estados ativos com ε-fecho.

Qualquer divergência é listada no stderr e o programa termina com código 1.

## Como executar

```sh
cd validacao
cargo run --release
```

A primeira execução baixa e compila a Autosim (tag `v1.0.4`) e leva alguns minutos.

## Resultado (8 nov. 2026, Autosim 1.0.4)

| Autômato | Tipo | Estados | Linguagem | Comp. máx. | Cadeias | Aceitas | Divergências | Tempo (ms) |
|---|---|---|---|---|---|---|---|---|
| `afd_contem_01` | AFD | 3 | contém 01 | 12 | 8191 | 8100 | 0 | 67 |
| `afd_pares` | AFD | 4 | nº par de 0s e de 1s | 12 | 8191 | 2731 | 0 | 69 |
| `afd_mult3` | AFD | 3 | múltiplos de 3 em binário | 12 | 8191 | 2737 | 0 | 63 |
| `afn_termina_01` | AFN | 3 | termina em 01 | 12 | 8191 | 2047 | 0 | 85 |
| `afn_terceiro_fim` | AFN | 4 | 3º símbolo do fim é 1 | 12 | 8191 | 4092 | 0 | 120 |
| `afd_terceiro_fim` | AFD | 8 | 3º símbolo do fim é 1 | 12 | 8191 | 4092 | 0 | 102 |
| `afn_eps_astar_bstar` | AFN | 2 | a*b* | 12 | 8191 | 91 | 0 | 27 |
| `afn_eps_mult_2_ou_3` | AFN | 6 | 0^k, k múltiplo de 2 ou 3 | 30 | 31 | 21 | 0 | 0 |

Total: 57368 cadeias simuladas, 0 divergências.

Os tempos variam conforme a máquina e incluem as duas referências; servem apenas como ordem
de grandeza.

Para conferir que o verificador detecta erros, basta alterar um dos predicados (por exemplo,
trocar `"ba"` por `"ab"` em `astar_bstar`): as divergências passam a ser listadas.

## Desempenho (gráfico da apresentação)

`src/bin/desempenho.rs` mede só a Autosim, sem as duas referências: o tempo médio por cadeia em
cada autômato (as mesmas cadeias da validação) e o crescimento do tempo com o comprimento da
cadeia, em 200 cadeias aleatórias por comprimento. Cada valor é a mediana de 5 repetições.

```sh
cargo run --release --bin desempenho
```

Resultado em 14 nov. 2026 (Autosim 1.0.4, notebook dos autores):

| Autômato | µs por cadeia |
|---|---|
| `afd_contem_01` | 6,1 |
| `afd_pares` | 6,2 |
| `afd_mult3` | 5,6 |
| `afn_termina_01` | 8,0 |
| `afn_terceiro_fim` | 11,1 |
| `afd_terceiro_fim` | 8,9 |
| `afn_eps_astar_bstar` | 2,4 |
| `afn_eps_mult_2_ou_3` | 13,5 |

Crescimento com o comprimento (`afn_terceiro_fim`, média de duas execuções), em µs por cadeia:
100 → 96,8; 200 → 189,7; 300 → 260,8; 400 → 406,6; 500 → 478,4; 600 → 615,9; 700 → 626,1;
800 → 947,1; 900 → 1.020,1; 1.000 → 1.104,0. O crescimento é aproximadamente linear, coerente
com a busca, que visita no máximo |Q|·(n+1) pares.
