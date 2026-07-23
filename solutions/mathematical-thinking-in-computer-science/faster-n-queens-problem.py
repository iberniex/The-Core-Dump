def can_be_extended_to_solution(perm: list) -> bool:
    i = len(perm) - 1
    for j in range(i):
        if i - j == abs( perm[i] - perm[j]):
            return False
    return True

def extend(perm,total_perm, n):
    if len(perm) == n:
        total_perm.append(perm)
        print(perm)
        print(len(total_perm))
        return

    for k in range(n):
        if k not in perm:
            perm.append(k)
            if can_be_extended_to_solution(perm):
                extend(perm, total_perm, n)
            perm.pop()


extend(perm = [], total_perm=[] , n=8)


