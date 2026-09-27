# TCC-UTP — Ferramenta Baseada em DSL para o Ensino de Autômatos Finitos

Trabalho de Conclusão de Curso — Bacharelado em Ciência da Computação,
**Universidade Tuiuti do Paraná (UTP), Curitiba, 2026.**

* **Autores:** João Thomaz Vieira · Klaus Siegfried Beckmann
* **Orientador:** Prof. Diógenes Cogo Furlan
* **Título:** Ferramenta Baseada em Linguagem de Domínio Específico para o Apoio ao Ensino de Autômatos Finitos com Simulação Visual Interativa (Autosim)

Este repositório contém a **monografia e a apresentação de defesa** (LaTeX ABNT).
A implementação da ferramenta está em repositório separado:
**https://github.com/KsBeckmann/autosim**

## Conteúdo

* `trabalho.tex` — monografia completa (9 capítulos: Introdução, Autômatos, Compiladores/DSLs, Rust, Trabalhos Relacionados, Projeto da DSL, Implementação, Resultados, Conclusão)
* `apresentacao.tex` — slides de defesa em Beamer (~35 min)
* `trabalho.pdf` / `apresentacao.pdf` — PDFs compilados
* `referencias.bib` — bibliografia (Hopcroft, Sipser, Aho, Fowler, Morazán, etc.)
* `packages/abnt-UTP.sty` — adaptação ABNT para normas UTP
* `img/` — diagramas e screenshots da Autosim
* `outros_pdfs/` — cronograma, proposta de tema e normas UTP

## Pré-requisitos

TeX Live 2023 (ou superior) com `pdflatex` + `bibtex` + `make`.

## Como compilar

```bash
make trabalho       # gera trabalho.pdf (pdflatex -> bibtex -> pdflatex -> pdflatex)
make clean          # limpa auxiliares

# apresentação (2x para o sumário):
pdflatex apresentacao.tex
pdflatex apresentacao.tex
```

## Resumo

O ensino de autômatos finitos sofre com a abstração do formalismo e a falta de
retroalimentação imediata. Ferramentas como o JFLAP são exclusivamente visuais,
sem descrição textual versionável nem diagnósticos precisos. Este trabalho propõe
a **Autosim**: uma DSL textual em português (`.asl`) para AFDs/AFNs com
transições ε, com compilador pedagógico (análise léxica, sintática e semântica
com erros localizados) e interface gráfica interativa que anima a simulação
passo a passo. Validação: 51 testes automatizados + simulações com autômatos
clássicos da literatura, todos corretos.
