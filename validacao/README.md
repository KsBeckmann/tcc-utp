# Validação diferencial do simulador

Código que produz o Quadro "Validação diferencial do simulador" do Capítulo 8 da monografia.

Para cada autômato em `automatos/`, todas as cadeias sobre o alfabeto do arquivo até um
comprimento máximo são simuladas pela Autosim 1.0.3, e o veredito é comparado com:

1. a definição da linguagem, escrita como predicado em `src/main.rs`;
2. um simulador de referência que propaga o conjunto de estados ativos com ε-fecho.

Qualquer divergência é listada no stderr e o programa termina com código 1.

## Como executar

```sh
cd validacao
cargo run --release
```

A primeira execução baixa e compila a Autosim (tag `v1.0.3`) e leva alguns minutos.

## Resultado (7 nov. 2026)

| Autômato | Tipo | Estados | Linguagem | Comp. máx. | Cadeias | Aceitas | Divergências | Tempo (ms) |
|---|---|---|---|---|---|---|---|---|
| `afd_contem_01` | AFD | 3 | contém 01 | 12 | 8191 | 8100 | 0 | 138 |
| `afd_pares` | AFD | 4 | nº par de 0s e de 1s | 12 | 8191 | 2731 | 0 | 145 |
| `afd_mult3` | AFD | 3 | múltiplos de 3 em binário | 12 | 8191 | 2737 | 0 | 131 |
| `afn_termina_01` | AFN | 3 | termina em 01 | 12 | 8191 | 2047 | 0 | 173 |
| `afn_terceiro_fim` | AFN | 4 | 3º símbolo do fim é 1 | 12 | 8191 | 4092 | 0 | 249 |
| `afd_terceiro_fim` | AFD | 8 | 3º símbolo do fim é 1 | 12 | 8191 | 4092 | 0 | 208 |
| `afn_eps_astar_bstar` | AFN | 2 | a*b* | 12 | 8191 | 91 | 0 | 55 |
| `afn_eps_mult_2_ou_3` | AFN | 6 | 0^k, k múltiplo de 2 ou 3 | 30 | 31 | 21 | 0 | 1 |

Total: 57368 cadeias simuladas, 0 divergências.

Os tempos variam conforme a máquina e incluem as duas referências; servem apenas como ordem
de grandeza.

Para conferir que o verificador detecta erros, basta alterar um dos predicados (por exemplo,
trocar `"ba"` por `"ab"` em `astar_bstar`): as divergências passam a ser listadas.
