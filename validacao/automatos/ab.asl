// AFN-ε que reconhece a* b*, o mesmo do exemplo distribuído com a Autosim.

alfabeto { 'a', 'b' }

automato AFN afn_eps_astar_bstar {
    estados { q0, q1 }
    inicial q0
    finais { q0, q1 }

    transicoes {
        q0 -> q0 com 'a'
        q0 -> q1 com eps
        q1 -> q1 com 'b'
    }
}

simular afn_eps_astar_bstar com "aabb"
