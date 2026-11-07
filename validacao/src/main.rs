//! Validação diferencial do simulador da Autosim.
//!
//! Para cada autômato de `automatos/`, todas as cadeias sobre o alfabeto do
//! arquivo até um comprimento máximo são simuladas pela Autosim e o veredito
//! é comparado com duas referências independentes:
//!
//! 1. a definição da linguagem, escrita como um predicado sobre a cadeia;
//! 2. um simulador de referência que propaga o conjunto de estados ativos com
//!    ε-fecho, conforme a definição formal de AFN-ε (Seção 2.1).
//!
//! Uso: `cargo run --release` a partir deste diretório.

use std::collections::BTreeSet;
use std::fs;
use std::time::Instant;

use autosim::lexer::tokenize_spanned;
use autosim::parser::{Automaton, Program, Symbol, parse};
use autosim::runtime::{Simulator, Verdict};
use autosim::sema::analyse;

struct Caso {
    arquivo: &'static str,
    automato: &'static str,
    linguagem: &'static str,
    comprimento_max: usize,
    pertence: fn(&str) -> bool,
}

fn pares(s: &str) -> bool {
    s.matches('0').count() % 2 == 0 && s.matches('1').count() % 2 == 0
}

fn mult3(s: &str) -> bool {
    s.chars().fold(0u32, |r, c| (2 * r + u32::from(c == '1')) % 3) == 0
}

fn terceiro_fim(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    c.len() >= 3 && c[c.len() - 3] == '1'
}

fn astar_bstar(s: &str) -> bool {
    !s.contains("ba")
}

fn mult_2_ou_3(s: &str) -> bool {
    s.len() % 2 == 0 || s.len() % 3 == 0
}

const CASOS: &[Caso] = &[
    Caso { arquivo: "binario.asl", automato: "afd_contem_01", linguagem: "contém 01", comprimento_max: 12, pertence: |s| s.contains("01") },
    Caso { arquivo: "binario.asl", automato: "afd_pares", linguagem: "nº par de 0s e de 1s", comprimento_max: 12, pertence: pares },
    Caso { arquivo: "binario.asl", automato: "afd_mult3", linguagem: "múltiplos de 3 em binário", comprimento_max: 12, pertence: mult3 },
    Caso { arquivo: "binario.asl", automato: "afn_termina_01", linguagem: "termina em 01", comprimento_max: 12, pertence: |s| s.ends_with("01") },
    Caso { arquivo: "binario.asl", automato: "afn_terceiro_fim", linguagem: "3º símbolo do fim é 1", comprimento_max: 12, pertence: terceiro_fim },
    Caso { arquivo: "binario.asl", automato: "afd_terceiro_fim", linguagem: "3º símbolo do fim é 1", comprimento_max: 12, pertence: terceiro_fim },
    Caso { arquivo: "ab.asl", automato: "afn_eps_astar_bstar", linguagem: "a*b*", comprimento_max: 12, pertence: astar_bstar },
    Caso { arquivo: "unario.asl", automato: "afn_eps_mult_2_ou_3", linguagem: "0^k, k múltiplo de 2 ou 3", comprimento_max: 30, pertence: mult_2_ou_3 },
];

fn carregar(arquivo: &str) -> Program {
    let caminho = format!("{}/automatos/{arquivo}", env!("CARGO_MANIFEST_DIR"));
    let fonte = fs::read_to_string(&caminho).expect("leitura do arquivo-fonte");
    let spanned = tokenize_spanned(&fonte).unwrap_or_else(|_| panic!("erro léxico em {arquivo}"));
    let (tokens, spans): (Vec<_>, Vec<_>) = spanned.into_iter().unzip();
    let programa = parse(&tokens, &spans).unwrap_or_else(|_| panic!("erro sintático em {arquivo}"));
    analyse(&programa, &spans).unwrap_or_else(|_| panic!("erro semântico em {arquivo}"));
    programa
}

/// ε-fecho de um conjunto de estados.
fn fecho(a: &Automaton, mut conjunto: BTreeSet<String>) -> BTreeSet<String> {
    let mut pendentes: Vec<String> = conjunto.iter().cloned().collect();
    while let Some(q) = pendentes.pop() {
        for t in &a.transitions {
            if t.from.value == q && t.symbol.value == Symbol::Epsilon && conjunto.insert(t.to.value.clone()) {
                pendentes.push(t.to.value.clone());
            }
        }
    }
    conjunto
}

/// Simulador de referência: propaga o conjunto de estados ativos.
fn referencia(a: &Automaton, cadeia: &str) -> bool {
    let mut ativos = fecho(a, BTreeSet::from([a.initial.value.clone()]));
    for c in cadeia.chars() {
        let proximos = a
            .transitions
            .iter()
            .filter(|t| ativos.contains(&t.from.value) && t.symbol.value == Symbol::Char(c))
            .map(|t| t.to.value.clone())
            .collect();
        ativos = fecho(a, proximos);
    }
    a.finals.iter().any(|f| ativos.contains(&f.value))
}

/// Todas as cadeias sobre `alfabeto` com comprimento de 0 a `max`.
fn cadeias(alfabeto: &[char], max: usize) -> Vec<String> {
    let mut todas = vec![String::new()];
    let mut nivel = vec![String::new()];
    for _ in 0..max {
        nivel = nivel
            .iter()
            .flat_map(|s| alfabeto.iter().map(move |c| format!("{s}{c}")))
            .collect();
        todas.extend(nivel.iter().cloned());
    }
    todas
}

fn main() {
    println!("| Autômato | Tipo | Estados | Linguagem | Comp. máx. | Cadeias | Aceitas | Divergências | Tempo (ms) |");
    println!("|---|---|---|---|---|---|---|---|---|");
    let (mut total, mut total_div) = (0usize, 0usize);
    for caso in CASOS {
        let programa = carregar(caso.arquivo);
        let alfabeto: Vec<char> = programa.alphabet.iter().map(|c| c.value).collect();
        let automato = programa
            .automata
            .iter()
            .find(|a| a.name.value == caso.automato)
            .expect("autômato não encontrado");

        let lista = cadeias(&alfabeto, caso.comprimento_max);
        let (mut aceitas, mut divergencias) = (0usize, 0usize);
        let inicio = Instant::now();
        for s in &lista {
            let mut sim = Simulator::new(automato.clone(), s);
            let autosim = sim.run_to_end() == Verdict::Accepted;
            let definicao = (caso.pertence)(s);
            let ref_conjunto = referencia(automato, s);
            if autosim != definicao || autosim != ref_conjunto {
                divergencias += 1;
                eprintln!(
                    "DIVERGÊNCIA {} {s:?}: autosim={autosim} definição={definicao} referência={ref_conjunto}",
                    caso.automato
                );
            }
            aceitas += usize::from(autosim);
        }
        let ms = inicio.elapsed().as_millis();
        let tipo = if format!("{:?}", automato.kind) == "Dfa" { "AFD" } else { "AFN" };
        println!(
            "| `{}` | {tipo} | {} | {} | {} | {} | {aceitas} | {divergencias} | {ms} |",
            caso.automato,
            automato.states.len(),
            caso.linguagem,
            caso.comprimento_max,
            lista.len(),
        );
        total += lista.len();
        total_div += divergencias;
    }
    println!("\nTotal: {total} cadeias simuladas, {total_div} divergências.");
    if total_div > 0 {
        std::process::exit(1);
    }
}
