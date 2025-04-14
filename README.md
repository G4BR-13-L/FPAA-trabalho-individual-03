# 1. FPAA-trabalho-individual-03 - Hamiltonian Path Finder

O **Hamiltonian Path Finder** é um algoritmo que identifica um caminho hamiltoniano em um grafo representado por uma matriz de adjacência. Um caminho hamiltoniano é aquele que visita cada vértice exatamente uma vez. Este projeto demonstra uma implementação em Python utilizando backtracking para explorar possibilidades de caminhos no grafo.

## Descrição do Algoritmo

A função `hamiltonian_path` opera da seguinte forma:

1. **Inicialização**:
   - Define o número de vértices `n`
   - Cria uma lista `path` para armazenar o caminho atual

2. **Função de backtracking interna**:
   - Marca o vértice atual como visitado e o adiciona ao caminho
   - Se o caminho contém `n` vértices, um caminho hamiltoniano foi encontrado
   - Itera sobre os vértices vizinhos e chama recursivamente o backtracking
   - Se não encontrar caminho, desfaz as escolhas (backtrack)

3. **Itera sobre todos os vértices como ponto de partida**:
   - Para cada vértice de origem, inicializa `visited` e limpa `path`
   - Se um caminho for encontrado, retorna `path`

4. **Retorno final**:
   - Se nenhum caminho for encontrado a partir de nenhum vértice, retorna `None`

## Como Executar o Projeto

### Executar os testes com Docker:
Executa os testes unitários em **containeres de Rust e Python**
```sh
sudo docker compose up --build -d
```

### Executar os testes localmente:
```sh
./test.sh
```

### 1. Acessar o Diretório do Projeto  
```sh
cd caminho_hamiltoniano_python
```

### 2. Criar e Ativar um Ambiente Virtual Python

Crie um ambiente virtual:  
```sh
python3 -m venv venv
```  
Ative o ambiente virtual:  

```sh
source venv/bin/activate
```  

### 3. Instalar as Dependências  
```sh
pip install -r requirements.txt
```

### 4. Executar o Projeto  
```sh
python3 main.py
```

O programa executará a função `main()`

## Relatório Técnico


---

### Aplicação do Teorema Mestre

**Não é possivel aplicar o Teorema Mestre.**  
O **Teorema Mestre** é aplicável a **recorrências** do tipo:

\[
T(n) = a \cdot T\left(\frac{n}{b}\right) + f(n)
\]

Ou seja, ele serve para resolver **algoritmos recursivos com divisão do problema em subproblemas de tamanho menor** (ex: mergesort, quicksort, etc.).

O algoritmo de **Hamiltonian Path por backtracking**:
- **Não divide o problema em subproblemas menores**
- Ele **gera todas as permutações possíveis** de vértices para tentar encontrar um caminho válido
- A recorrência, se fosse formalizada, se assemelharia mais a:
  
  \[
  T(n) = (n - 1) \cdot T(n - 1) + O(1)
  \]

Essa recorrência **não se encaixa na forma canônica** do Teorema Mestre.

### Conclusão:

> O **Teorema Mestre não é aplicável** ao algoritmo Hamiltonian Path por backtracking porque:
> - O problema **não é dividido** em subproblemas de tamanho reduzido
> - A recorrência resultante **não possui a forma adequada** exigida pelo teorema

---

### Análise dos Casos de Complexidade

#### Diferença entre os casos de complexidade

| Caso         | Descrição                                                                 |
|--------------|--------------------------------------------------------------------------|
| **Pior caso** | O algoritmo percorre **todas as permutações possíveis** e **não encontra** um caminho válido. Complexidade: **O(n!)** |
| **Caso médio** | O algoritmo encontra um caminho **em algum ponto intermediário** depois de explorar parte das possibilidades. Complexidade média estimada: **entre O(n) e O(n!)**, mas geralmente próximo de **O(n!)** |
| **Melhor caso** | O algoritmo encontra um caminho Hamiltoniano **na primeira tentativa**. Complexidade: **O(n)** |

#### Impacto no desempenho do algoritmo

- O algoritmo é extremamente **sensível ao número de vértices (n)**. Pequenos aumentos em `n` causam crescimento exponencial no tempo de execução.
- **No pior caso**, é **inviável para grafos com mais de ~15 vértices**, pois há até `n!` caminhos a serem testados.
- **No melhor caso**, pode ser muito eficiente — mas isso é raro e depende da sorte do vértice inicial e da estrutura do grafo.
- O **caso médio** geralmente tende a ser próximo do pior caso, pois **a maioria das tentativas falha** antes de encontrar um caminho válido.


## Exemplo de Uso

```python
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
```

## Grafo:

![imagem algoritmo python](/img/grafo_python.png)

### Tabela de Mapeamento:

| Vértice | Linha do Código | Instrução/Chamada           |
|---------|------------------|-----------------------------|
| 0       | def main():      | Início de main              |
| 1       | result = ...     | Chamada hamiltonian_path    |
| 2       | def hamiltonian_path | Início da função         |
| 3       | n = len(graph)   | Tamanho do grafo            |
| 4       | path = []        | Inicializa o caminho        |
| 5       | def backtrack(...) | Função recursiva interna |
| 6       | path.append(...) | Adiciona ao caminho         |
| 7       | visited[...] = True | Marca como visitado      |
| 8       | if len(path) == n | Verifica fim do caminho   |
| 9       | return True      | Caminho encontrado          |
| 10      | for neighbor in ... | Loop pelos vizinhos     |
| 11      | if graph[...] and not visited[...] | Checa adjacência e visita |
| 12      | if backtrack(...) | Chamada recursiva          |
| 13      | path.pop()       | Desfaz escolha (backtrack)  |
| 14      | visited[...] = False | Marca como não visitado |
| 15      | for start_vertex in ... | Testa todos os inícios |
| 16      | visited = [...]  | Inicializa vetor de visita  |
| 17      | path.clear()     | Limpa o caminho anterior    |
| 18      | return path      | Retorna caminho encontrado  |
| 19      | return None      | Nenhum caminho encontrado   |
| 20      | print(...)       | Exibe resultado final       |
| 21      | fim              | Fim da execução             |