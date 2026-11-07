// Autômatos clássicos sobre o alfabeto binário, usados na validação
// diferencial do Capítulo 8 (Resultados e Discussões).

alfabeto { '0', '1' }

// Cadeias que contêm a subcadeia 01 (HOPCROFT; MOTWANI; ULLMAN, 2006)
automato AFD afd_contem_01 {
    estados { q0, q1, q2 }
    inicial q0
    finais { q2 }

    transicoes {
        q0 -> q0 com '1'
        q0 -> q1 com '0'
        q1 -> q1 com '0'
        q1 -> q2 com '1'
        q2 -> q2 com '0'
        q2 -> q2 com '1'
    }
}

// Cadeias com número par de 0s e número par de 1s (HOPCROFT; MOTWANI; ULLMAN, 2006)
automato AFD afd_pares {
    estados { pp, pi, ip, ii }
    inicial pp
    finais { pp }

    transicoes {
        pp -> ip com '0'
        pp -> pi com '1'
        pi -> ii com '0'
        pi -> pp com '1'
        ip -> pp com '0'
        ip -> ii com '1'
        ii -> pi com '0'
        ii -> ip com '1'
    }
}

// Representações binárias de múltiplos de 3 (estado = resto da divisão por 3);
// a cadeia vazia é aceita, pois representa o valor 0
automato AFD afd_mult3 {
    estados { r0, r1, r2 }
    inicial r0
    finais { r0 }

    transicoes {
        r0 -> r0 com '0'
        r0 -> r1 com '1'
        r1 -> r2 com '0'
        r1 -> r0 com '1'
        r2 -> r1 com '0'
        r2 -> r2 com '1'
    }
}

// Cadeias terminadas em 01 (HOPCROFT; MOTWANI; ULLMAN, 2006)
automato AFN afn_termina_01 {
    estados { q0, q1, q2 }
    inicial q0
    finais { q2 }

    transicoes {
        q0 -> q0 com '0'
        q0 -> q0 com '1'
        q0 -> q1 com '0'
        q1 -> q2 com '1'
    }
}

// Cadeias cujo terceiro símbolo a partir do fim é 1 (SIPSER, 2012)
automato AFN afn_terceiro_fim {
    estados { q1, q2, q3, q4 }
    inicial q1
    finais { q4 }

    transicoes {
        q1 -> q1 com '0'
        q1 -> q1 com '1'
        q1 -> q2 com '1'
        q2 -> q3 com '0'
        q2 -> q3 com '1'
        q3 -> q4 com '0'
        q3 -> q4 com '1'
    }
}

// AFD equivalente ao anterior, obtido pela construção de subconjuntos: cada
// estado guarda os três últimos símbolos lidos (completados à esquerda com 0)
automato AFD afd_terceiro_fim {
    estados { s000, s001, s010, s011, s100, s101, s110, s111 }
    inicial s000
    finais { s100, s101, s110, s111 }

    transicoes {
        s000 -> s000 com '0'
        s000 -> s001 com '1'
        s001 -> s010 com '0'
        s001 -> s011 com '1'
        s010 -> s100 com '0'
        s010 -> s101 com '1'
        s011 -> s110 com '0'
        s011 -> s111 com '1'
        s100 -> s000 com '0'
        s100 -> s001 com '1'
        s101 -> s010 com '0'
        s101 -> s011 com '1'
        s110 -> s100 com '0'
        s110 -> s101 com '1'
        s111 -> s110 com '0'
        s111 -> s111 com '1'
    }
}

simular afd_contem_01 com "1101"
