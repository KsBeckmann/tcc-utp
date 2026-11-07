# Correções do PG II — versão de outubro

Devolutiva da orientadora sobre redação, formatação e estrutura (sem avaliação de conteúdo técnico).
As páginas citadas se referem ao arquivo `Outubro.pdf`.

**Prazo: entrega para a banca em 14/11 (trabalho pronto, no OneDrive).**

## Resumo e introdução

### Resumo (p. 3)
- [x] Trocar "propõe o desenvolvimento" e "Espera-se" por formulações que correspondam à ferramenta já implementada e aos testes apresentados
- [x] Sintetizar os resultados efetivos
- [x] Manter os benefícios pedagógicos (ainda não avaliados) como expectativas

### Sumário e formatação
- [x] Corrigir o Sumário: o título "Fundamentação Teórica" perdeu a numeração
- [x] Corrigir a folha 14: o título "Fundamentação Teórica" ficou no meio de uma folha em branco

### Introdução (pp. 11–13)
- [x] Explicitar a pergunta que orienta a pesquisa
- [x] Anunciar os apêndices com os manuais

### Apêndices (novos — o arquivo termina nas referências)
- [x] Instruções de instalação
- [x] Instruções de execução
- [x] Como escrever exemplos na DSL
- [x] Como interpretar os diagnósticos de erro

### Objetivos (p. 12)
- [x] Compatibilizar com a entrega: foi prometida formalização em BNF e EBNF, mas o cap. 6 só apresenta EBNF — incluir a BNF **ou** justificar o ajuste de escopo
- [x] Diferenciar a validação funcional da avaliação da eficácia pedagógica com estudantes (proposta para trabalho futuro)

## Coerência e conclusão

### Ferramentas existentes (pp. 3, 11 e 61)
- [x] Revisar a afirmação geral de que nenhuma ferramenta tem descrição textual: conflita com o FSM, descrito na p. 53 como DSL
- [x] Incluir a comparação com o FSM ao delimitar a contribuição do projeto

### Gramática × testes
- [x] Regra de alfabeto (p. 70) exige ao menos um caractere, mas o teste da p. 87 aceita `alfabeto { }` — decidir o comportamento desejado
- [x] Ajustar a especificação **ou** a implementação, e o teste correspondente

### Precisão técnica do texto
- [x] pp. 76 e 88: esclarecer se os intervalos de localização da AST usam índices de caracteres ou de tokens, e em qual etapa ocorre a conversão
- [x] pp. 78–83: distinguir o acompanhamento de um caminho escolhido da manutenção do conjunto de estados ativos, delimitando cada modo

### Remissões
- [x] pp. 30, 32 e 39: a linguagem do projeto é remetida ao cap. 3, mas está no cap. 6
- [x] p. 52: a interface é remetida ao cap. 6, mas a implementação está no cap. 7
- [x] p. 86: trocar "três categorias" por quatro, conforme o Quadro 8

### Tempo verbal
- [x] pp. 59–60: atualizar as escolhas descritas no futuro, considerando as bibliotecas efetivamente usadas no cap. 7

### Conclusão (pp. 92–93)
- [x] Acrescentar as dificuldades efetivamente encontradas e como foram enfrentadas
- [x] Restringir as afirmações sobre solução das dificuldades de aprendizagem ao que foi demonstrado (testes de funcionamento não comprovam ganho de aprendizagem)

## Revisão linguística e referências

### Ortografia e estilo
- [ ] p. 38: "léxicamente" → "lexicamente"
- [ ] pp. 41 e 44: "ponto-e-vírgula" → "ponto e vírgula"
- [ ] Uniformizar "não determinístico" sem hífen em todo o texto
- [ ] Uniformizar "não terminais" sem hífen em todo o texto
- [ ] p. 25: eliminar a redundância "no caso geral, em geral"
- [ ] p. 75: corrigir o token Seta, exibido como dois traços, para `->`

### Referências
- [x] pp. 74–75 e 85: incluir citação e referência da biblioteca Ariadne
- [ ] p. 94: completar a referência de Nunes (2017) com instituição e tipo de trabalho

## Entrega final (14/11, OneDrive)
- [ ] Texto monográfico completo: pré-textuais, introdução, fundamentação teórica, trabalhos relacionados, metodologia de construção da solução, ferramentas para implementação, resultados e discussões, conclusões e pós-textuais
- [ ] Formatação conforme normas ABNT/UTP
- [ ] Programa implementado (e instalador, se necessário)
