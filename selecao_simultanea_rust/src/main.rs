fn max_min_select(array: Vec<i64>) -> Option<(i64, i64)> {
    let n = array.len();
    let mut min: i64;
    let mut max: i64;

    if n == 0 {
        return None;
    }

    if n == 1 {
        min = array[0];
        max = array[0];
        return Some((max, min));
    }

    if array[0] > array[1] {
        max = array[0];
        min = array[1];
    } else {
        max = array[1];
        min = array[0]
    }

    for i in 2..(n - 1) {
        if i == (n - 1) {
            if array[i] > max {
                max = array[i];
            } else if array[i] < min {
                min = array[i];
            }
        } else {
            if array[i] > array[i + 1] {
                if array[i] > max {
                    max = array[i];
                }
                if array[i + 1] < min {
                    min = array[i + 1];
                }
            } else {
                if array[i+1] > max {
                    max = array[i+1];
                }
                if array[i] < min {
                    min = array[i];
                }
            }
        }
    }

    return Some((max, min));

}

fn main() {
    let array: Vec<i64> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    match max_min_select(array) {
        Some((max, min)) => println!("Máximo: {}, Mínimo: {}", max, min),
        None => println!("Erro: vetor vazio ou outra condição de erro"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_vazio() {
        let array: Vec<i64> = vec![];
        assert_eq!(max_min_select(array), None);
    }

    #[test]
    fn test_array_um_elemento() {
        let array = vec![42];
        assert_eq!(max_min_select(array), Some((42, 42)));
    }

    #[test]
    fn test_array_dois_elementos_crescente() {
        let array = vec![1, 2];
        assert_eq!(max_min_select(array), Some((2, 1)));
    }

    #[test]
    fn test_array_dois_elementos_decrescente() {
        let array = vec![2, 1];
        assert_eq!(max_min_select(array), Some((2, 1)));
    }

    #[test]
    fn test_array_todos_elementos_iguais() {
        let array = vec![5, 5, 5, 5];
        assert_eq!(max_min_select(array), Some((5, 5)));
    }

    #[test]
    fn test_array_numeros_positivos() {
        let array = vec![10, 20, 30, 40, 50];
        assert_eq!(max_min_select(array), Some((50, 10)));
    }

    #[test]
    fn test_array_numeros_negativos() {
        let array = vec![-10, -20, -30, -40, -50];
        assert_eq!(max_min_select(array), Some((-10, -50)));
    }

    #[test]
    fn test_array_misturado_positivos_negativos() {
        let array = vec![-5, 0, 5, -10, 10];
        assert_eq!(max_min_select(array), Some((10, -10)));
    }

    #[test]
    fn test_array_com_zero() {
        let array = vec![0, 1, 2, 3, 4];
        assert_eq!(max_min_select(array), Some((4, 0)));
    }

    #[test]
    fn test_array_tamanho_impar() {
        let array = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(max_min_select(array), Some((13, 1)));
    }

    #[test]
    fn test_array_tamanho_par() {
        let array = vec![2, 4, 6, 8, 10, 12];
        assert_eq!(max_min_select(array), Some((12, 2)));
    }

    #[test]
    fn test_array_com_repeticao() {
        let array = vec![3, 7, 3, 1, 7, 1];
        assert_eq!(max_min_select(array), Some((7, 1)));
    }

    #[test]
    fn test_array_com_um_unico_elemento_negativo() {
        let array = vec![-42];
        assert_eq!(max_min_select(array), Some((-42, -42)));
    }

    #[test]
    fn test_array_com_dois_elementos_negativos() {
        let array = vec![-10, -20];
        assert_eq!(max_min_select(array), Some((-10, -20)));
    }

    #[test]
    fn test_array_com_tres_elementos_negativos() {
        let array = vec![-30, -10, -20];
        assert_eq!(max_min_select(array), Some((-10, -30)));
    }

    #[test]
    fn test_array_com_elementos_muito_grandes() {
        let array = vec![i64::MAX, i64::MIN, 0];
        assert_eq!(max_min_select(array), Some((i64::MAX, i64::MIN)));
    }

    #[test]
    fn test_array_com_elementos_misturados_grandes_pequenos() {
        let array = vec![i64::MAX, i64::MIN, 0, 100, -100];
        assert_eq!(max_min_select(array), Some((i64::MAX, i64::MIN)));
    }

    #[test]
    fn test_array_com_todos_elementos_negativos_iguais() {
        let array = vec![-5, -5, -5, -5];
        assert_eq!(max_min_select(array), Some((-5, -5)));
    }

    #[test]
    fn test_array_com_todos_elementos_positivos_iguais() {
        let array = vec![10, 10, 10, 10];
        assert_eq!(max_min_select(array), Some((10, 10)));
    }

    #[test]
    fn test_array_com_elementos_alternados() {
        let array = vec![1, -1, 2, -2, 3, -3];
        assert_eq!(max_min_select(array), Some((3, -3)));
    }

    #[test]
    fn test_array_com_elementos_decrescentes() {
        let array = vec![5, 4, 3, 2, 1];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_crescentes() {
        let array = vec![1, 2, 3, 4, 5];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_randomicos() {
        let array = vec![7, 3, 9, 1, 4];
        assert_eq!(max_min_select(array), Some((9, 1)));
    }

    #[test]
    fn test_array_com_um_elemento_zero() {
        let array = vec![0];
        assert_eq!(max_min_select(array), Some((0, 0)));
    }

    #[test]
    fn test_array_com_dois_elementos_zero() {
        let array = vec![0, 0];
        assert_eq!(max_min_select(array), Some((0, 0)));
    }

    #[test]
    fn test_array_com_tres_elementos_zero() {
        let array = vec![0, 0, 0];
        assert_eq!(max_min_select(array), Some((0, 0)));
    }

    #[test]
    fn test_array_com_elementos_positivos_e_zero() {
        let array = vec![0, 10, 20, 30];
        assert_eq!(max_min_select(array), Some((30, 0)));
    }

    #[test]
    fn test_array_com_elementos_negativos_e_zero() {
        let array = vec![-10, -20, 0, -30];
        assert_eq!(max_min_select(array), Some((0, -30)));
    }

    #[test]
    fn test_array_com_elementos_misturados_zero() {
        let array = vec![0, -5, 5, -10, 10];
        assert_eq!(max_min_select(array), Some((10, -10)));
    }

    #[test]
    fn test_array_com_elementos_muito_proximos() {
        let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(max_min_select(array), Some((10, 1)));
    }

    #[test]
    fn test_array_com_elementos_repetidos_no_fim() {
        let array = vec![1, 2, 3, 4, 5, 5, 5];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_repetidos_no_inicio() {
        let array = vec![5, 5, 5, 1, 2, 3, 4];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_repetidos_no_meio() {
        let array = vec![1, 2, 5, 5, 5, 3, 4];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_em_ordem_decrescente() {
        let array = vec![5, 4, 3, 2, 1];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_em_ordem_crescente() {
        let array = vec![1, 2, 3, 4, 5];
        assert_eq!(max_min_select(array), Some((5, 1)));
    }

    #[test]
    fn test_array_com_elementos_randomicos_grandes() {
        let array = vec![1000, 500, 750, 250, 100];
        assert_eq!(max_min_select(array), Some((1000, 100)));
    }

    #[test]
    fn test_array_com_elementos_randomicos_pequenos() {
        let array = vec![-1000, -500, -750, -250, -100];
        assert_eq!(max_min_select(array), Some((-100, -1000)));
    }
}