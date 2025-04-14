import pytest
from main import hamiltonian_path

def test_caminho_simples():
    graph = [
        [0, 1, 1],
        [1, 0, 1],
        [1, 1, 0]
    ]
    path = hamiltonian_path(graph)
    assert path is not None
    assert sorted(path) == [0, 1, 2]

def test_sem_caminho():
    graph = [
        [0, 1, 0],
        [1, 0, 0],
        [0, 0, 0]
    ]
    assert hamiltonian_path(graph) is None

def test_grafo_direcionado_com_caminho():
    graph = [
        [0, 1, 0],
        [0, 0, 1],
        [0, 0, 0]
    ]
    path = hamiltonian_path(graph)
    assert path is not None
    assert sorted(path) == [0, 1, 2]

def test_grafo_com_um_vertice():
    graph = [
        [0]
    ]
    assert hamiltonian_path(graph) == [0]

def test_grafo_com_dois_vertices_conectados():
    graph = [
        [0, 1],
        [1, 0]
    ]
    path = hamiltonian_path(graph)
    assert path is not None
    assert sorted(path) == [0, 1]
