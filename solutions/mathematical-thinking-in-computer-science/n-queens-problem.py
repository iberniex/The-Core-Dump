
import itertools as it  

def is_solution(perm):
    for (i1, i2) in it.permutations(range(len(perm)), 2):
        if abs(i1 - i2) == abs(perm[i1] - perm[i2]):
            return False
    return True


permy = [1, 5, 0, 6, 3, 7, 2, 4]

print(is_solution(permy))

for perm in it.permutations(range(8)):
    if is_solution(perm):
        print(perm)
        exit()

