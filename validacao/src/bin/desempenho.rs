//! Medição de desempenho do simulador da Autosim (gráfico de desempenho da apresentação).
//!
//! Mede só a Autosim (criação do simulador e execução até o veredito), sem as referências da
//! validação diferencial:
//!
//! 1. tempo médio por cadeia em cada autômato, sobre as mesmas cadeias da validação;
//! 2. crescimento do tempo com o comprimento da cadeia, em um AFN.
//!
//! Cada medida é a mediana de 5 repetições. Uso: `cargo run --release --bin desempenho`.

use std::fs;
use std::time::Instant;

use autosim::lexer::tokenize_spanned;
use autosim::parser::{Automaton, Program, parse};
use autosim::runtime::Simulator;
use autosim::sema::analyse;

const CASOS: &[(&str, &str, usize)] = &[
    ("binario.asl", "afd_contem_01", 12),
    ("binario.asl", "afd_pares", 12),
    ("binario.asl", "afd_mult3", 12),
    ("binario.asl", "afn_termina_01", 12),
    ("binario.asl", "afn_terceiro_fim", 12),
    ("binario.asl", "afd_terceiro_fim", 12),
    ("ab.asl", "afn_eps_astar_bstar", 12),
    ("unario.asl", "afn_eps_mult_2_ou_3", 30),
];
const REPETICOES: usize = 5;

fn carregar(arquivo: &str) -> Program {
    let caminho = format!("{}/automatos/{arquivo}", env!("CARGO_MANIFEST_DIR"));
    let fonte = fs::read_to_string(&caminho).expect("leitura do arquivo-fonte");
    let spanned = tokenize_spanned(&fonte).unwrap_or_else(|_| panic!("erro léxico em {arquivo}"));
    let (tokens, spans): (Vec<_>, Vec<_>) = spanned.into_iter().unzip();
    let programa = parse(&tokens, &spans).unwrap_or_else(|_| panic!("erro sintático em {arquivo}"));
    analyse(&programa, &spans).unwrap_or_else(|_| panic!("erro semântico em {arquivo}"));
    programa
}

fn automato<'a>(p: &'a Program, nome: &str) -> &'a Automaton {
    p.automata.iter().find(|a| a.name.value == nome).expect("autômato não encontrado")
}

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

/// Tempo total (em microssegundos) para simular todas as cadeias; mediana de REPETICOES.
fn medir(a: &Automaton, lista: &[String]) -> f64 {
    let mut tempos: Vec<f64> = (0..REPETICOES)
        .map(|_| {
            let inicio = Instant::now();
            for s in lista {
                let mut sim = Simulator::new(a.clone(), s);
                std::hint::black_box(sim.run_to_end());
            }
            inicio.elapsed().as_secs_f64() * 1e6
        })
        .collect();
    tempos.sort_by(f64::total_cmp);
    tempos[REPETICOES / 2]
}

/// Gerador pseudoaleatório simples e determinístico (as medições se repetem iguais).
fn aleatorias(n: usize, quantidade: usize, semente: u64) -> Vec<String> {
    let mut x = semente;
    (0..quantidade)
        .map(|_| {
            (0..n)
                .map(|_| {
                    x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    if (x >> 33) & 1 == 0 { '0' } else { '1' }
                })
                .collect()
        })
        .collect()
}

fn main() {
    println!("# 1. Tempo médio por cadeia (todas as cadeias até o comprimento indicado)");
    println!("automato;estados;cadeias;total_ms;us_por_cadeia");
    for (arquivo, nome, max) in CASOS {
        let p = carregar(arquivo);
        let alfabeto: Vec<char> = p.alphabet.iter().map(|c| c.value).collect();
        let a = automato(&p, nome);
        let lista = cadeias(&alfabeto, *max);
        let us = medir(a, &lista);
        println!(
            "{nome};{};{};{:.1};{:.2}",
            a.states.len(),
            lista.len(),
            us / 1000.0,
            us / lista.len() as f64
        );
    }

    println!("\n# 2. Crescimento com o comprimento (afn_terceiro_fim, 200 cadeias aleatórias)");
    println!("comprimento;us_por_cadeia");
    let p = carregar("binario.asl");
    let a = automato(&p, "afn_terceiro_fim");
    for n in (100..=1000).step_by(100) {
        let lista = aleatorias(n, 200, 42 + n as u64);
        let us = medir(a, &lista);
        println!("{n};{:.1}", us / lista.len() as f64);
    }
}
