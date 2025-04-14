
def hamiltonian_path(graph):
    n = len(graph)
    path = []

    def backtrack(current, visited):
        path.append(current)
        visited[current] = True

        if len(path) == n:
            return True 

        for neighbor in range(n):
            if graph[current][neighbor] and not visited[neighbor]:
                if backtrack(neighbor, visited):
                    return True

        # backtrack
        path.pop()
        visited[current] = False
        return False

    for start_vertex in range(n):
        visited = [False] * n
        path.clear()
        if backtrack(start_vertex, visited):
            return path

    return None 


def main():
    graph = [
        [0, 1, 1, 0], 
        [1, 0, 1, 1], 
        [1, 1, 0, 1], 
        [0, 1, 1, 0]  
    ]

    result = hamiltonian_path(graph)

    if result:
        print("Caminho Hamiltoniano encontrado:", result)
    else:
        print("Nenhum Caminho Hamiltoniano encontrado.")


if __name__ == "__main__":
    main()
