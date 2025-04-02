
def max_min_select(array):
    n = len(array)
    max = 0
    min = 0
    if n == 0:
        return None
    
    if n == 1:
        max = array[0]
        min = array[0]
        return (max, min)
    
    if array[0] > array[1]:
        max = array[0]
        min = array[1]
    else:
        max = array[1]
        min = array[0]
    
    i = 2
    while i < (n-1):
        if i == n - 1:
            if array[i] > max:
                max = array[i]
            elif array[i] < min:
                min = array[i]
        else:
            if array[i] > array[i+1]:
                if array[i] > max:
                    max = array[i]
                if array[i+1] < min:
                    min = array[i+1]
            else:
                if array[i+1] > max:
                    max = array[i+1]
                if array[i] < min:
                    min = array[i]
        i += 1

    return (max, min)


def main():
    print(max_min_select([0,1,2,3,4,5,6,7,8,9]))

if __name__ == "__main__":
    main()