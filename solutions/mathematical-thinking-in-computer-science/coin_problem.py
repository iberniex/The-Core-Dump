def change(amount):
  assert(amount > 5)
  if amount % 5 == 0:
    return [5 for _ in range(amount // 5)]
  if amount % 7 == 0:
    return [7 for _ in range(amount // 7)]
  if amount == 12:
    return [5, 7]

  solution = change(amount - 5)
  solution.append(5)

  return solution

def forbenius_number(a, b):
    return a *b - a - b

print(change(11))

print(
    forbenius_number(5, 7)
)
