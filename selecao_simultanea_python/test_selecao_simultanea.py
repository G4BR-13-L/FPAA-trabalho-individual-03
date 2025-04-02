import pytest
from main import max_min_select

def test_array_vazio():
    assert max_min_select([]) is None

def test_array_um_elemento():
    assert max_min_select([42]) == (42, 42)

def test_array_dois_elementos_crescente():
    assert max_min_select([1, 2]) == (2, 1)

def test_array_dois_elementos_decrescente():
    assert max_min_select([2, 1]) == (2, 1)

def test_array_todos_elementos_iguais():
    assert max_min_select([5, 5, 5, 5]) == (5, 5)

def test_array_numeros_positivos():
    assert max_min_select([10, 20, 30, 40, 50]) == (50, 10)

def test_array_numeros_negativos():
    assert max_min_select([-10, -20, -30, -40, -50]) == (-10, -50)

def test_array_misturado_positivos_negativos():
    assert max_min_select([-5, 0, 5, -10, 10]) == (10, -10)

def test_array_com_zero():
    assert max_min_select([0, 1, 2, 3, 4]) == (4, 0)

def test_array_tamanho_impar():
    assert max_min_select([1, 3, 5, 7, 9, 11, 13]) == (13, 1)

def test_array_tamanho_par():
    assert max_min_select([2, 4, 6, 8, 10, 12]) == (12, 2)

def test_array_com_repeticao():
    assert max_min_select([3, 7, 3, 1, 7, 1]) == (7, 1)

def test_array_com_elementos_muito_grandes():
    assert max_min_select([2**63 - 1, -2**63, 0]) == (2**63 - 1, -2**63)

def test_array_com_elementos_misturados_grandes_pequenos():
    assert max_min_select([2**63 - 1, -2**63, 0, 100, -100]) == (2**63 - 1, -2**63)

def test_array_com_todos_elementos_negativos_iguais():
    assert max_min_select([-5, -5, -5, -5]) == (-5, -5)

def test_array_com_todos_elementos_positivos_iguais():
    assert max_min_select([10, 10, 10, 10]) == (10, 10)

def test_array_com_elementos_alternados():
    assert max_min_select([1, -1, 2, -2, 3, -3]) == (3, -3)

def test_array_com_elementos_randomicos():
    assert max_min_select([7, 3, 9, 1, 4]) == (9, 1)

def test_array_com_um_elemento_zero():
    assert max_min_select([0]) == (0, 0)

def test_array_com_dois_elementos_zero():
    assert max_min_select([0, 0]) == (0, 0)

def test_array_com_elementos_positivos_e_zero():
    assert max_min_select([0, 10, 20, 30]) == (30, 0)

def test_array_com_elementos_negativos_e_zero():
    assert max_min_select([-10, -20, 0, -30]) == (0, -30)

def test_array_com_elementos_misturados_zero():
    assert max_min_select([0, -5, 5, -10, 10]) == (10, -10)

def test_array_com_elementos_muito_proximos():
    assert max_min_select([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]) == (10, 1)

def test_array_com_elementos_repetidos_no_fim():
    assert max_min_select([1, 2, 3, 4, 5, 5, 5]) == (5, 1)

def test_array_com_elementos_repetidos_no_inicio():
    assert max_min_select([5, 5, 5, 1, 2, 3, 4]) == (5, 1)

def test_array_com_elementos_repetidos_no_meio():
    assert max_min_select([1, 2, 5, 5, 5, 3, 4]) == (5, 1)

def test_array_com_elementos_em_ordem_decrescente():
    assert max_min_select([5, 4, 3, 2, 1]) == (5, 1)

def test_array_com_elementos_em_ordem_crescente():
    assert max_min_select([1, 2, 3, 4, 5]) == (5, 1)

def test_array_com_elementos_randomicos_grandes():
    assert max_min_select([1000, 500, 750, 250, 100]) == (1000, 100)

def test_array_com_elementos_randomicos_pequenos():
    assert max_min_select([-1000, -500, -750, -250, -100]) == (-100, -1000)
