# Iterative approach

def factorial(n: int)-> int:
    assert(n > 0)
    result = 1
    for i in range (1, n+1):
        result *= i

    return result

# Recursive Approach

def recursive_factorial(n: int) -> int:
    assert (n > 0)
    if n <= 1:
        return n
    
    return n * recursive_factorial(n-1)


print("Iterative factorial: {}".format(factorial(10)))
print("Recursive factorial: {}".format(recursive_factorial(10)))
