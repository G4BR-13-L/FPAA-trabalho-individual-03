fn hamiltonian_path(graph: &Vec<Vec<u8>>) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut path = Vec::new();

    fn backtrack(
        graph: &Vec<Vec<u8>>,
        current: usize,
        visited: &mut Vec<bool>,
        path: &mut Vec<usize>,
        n: usize,
    ) -> bool {
        path.push(current);
        visited[current] = true;

        if path.len() == n {
            return true;
        }

        for neighbor in 0..n {
            if graph[current][neighbor] == 1 && !visited[neighbor] {
                if backtrack(graph, neighbor, visited, path, n) {
                    return true;
                }
            }
        }

        path.pop();
        visited[current] = false;
        false
    }

    for start_vertex in 0..n {
        let mut visited = vec![false; n];
        path.clear();
        if backtrack(graph, start_vertex, &mut visited, &mut path, n) {
            return Some(path.clone());
        }
    }

    None
}

fn main() {
    let graph = vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 1],
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 0],
    ];

    match hamiltonian_path(&graph) {
        Some(path) => println!("Caminho Hamiltoniano encontrado: {:?}", path),
        None => println!("Nenhum Caminho Hamiltoniano encontrado."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grafo_vazio() {
        let graph = vec![];
        assert_eq!(hamiltonian_path(&graph), None);
    }

    #[test]
    fn test_um_vertice() {
        let graph = vec![vec![0]];
        assert_eq!(hamiltonian_path(&graph), Some(vec![0]));
    }

    #[test]
    fn test_dois_vertices_conectados() {
        let graph = vec![
            vec![0, 1],
            vec![1, 0],
        ];
        let result = hamiltonian_path(&graph);
        assert!(result == Some(vec![0, 1]) || result == Some(vec![1, 0]));
    }

    #[test]
    fn test_sem_caminho() {
        let graph = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(hamiltonian_path(&graph), None);
    }

    #[test]
    fn test_caminho_existente() {
        let graph = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 0],
        ];
        let result = hamiltonian_path(&graph).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result.iter().copied().collect::<std::collections::HashSet<_>>().len(), 3);
    }

    #[test]
    fn test_direcionado_com_caminho() {
        let graph = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ];
        let result = hamiltonian_path(&graph).unwrap();
        assert_eq!(result, vec![0, 1, 2]);
    }
}
