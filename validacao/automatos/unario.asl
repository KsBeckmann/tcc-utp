// AFN-ε sobre alfabeto unário que aceita 0^k com k múltiplo de 2 ou de 3
// (SIPSER, 2012): o estado inicial escolhe, por transições ε, um de dois ciclos.

alfabeto { '0' }

automato AFN afn_eps_mult_2_ou_3 {
    estados { i, a0, a1, b0, b1, b2 }
    inicial i
    finais { a0, b0 }

    transicoes {
        i -> a0 com eps
        i -> b0 com eps
        a0 -> a1 com '0'
        a1 -> a0 com '0'
        b0 -> b1 com '0'
        b1 -> b2 com '0'
        b2 -> b0 com '0'
    }
}

simular afn_eps_mult_2_ou_3 com "000000"
