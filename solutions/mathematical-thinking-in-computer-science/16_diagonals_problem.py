def solve_grid():
    SIZE = 5
    #Track used intersections (6x6 grid)
    intersections = [[False] * (SIZE +1) for _ in range(SIZE +1)]
    solutions = []
    current_grid = [[' ' for _ in range(SIZE)] for _ in range(SIZE)]
    max_diagonals = 0

    def is_valid(i, j, diagonal_type):
        if diagonal_type == '/':
            return not intersections[i][j+1] and not intersections[i+1][j]
        else: # '\'
            return not intersections[i][j] and not intersections[i+1][j+1]

    def mark_diagonal(i, j, diagonal_type, state):
        if diagonal_type == '/':
            intersections[i][j + 1] = state
            intersections[i + 1][j] = state
        else:  #'\'
            intersections[i][j] = state
            intersections[i + 1][j + 1] = state

    def backtrack(cell, count):
        nonlocal max_diagonals
        row, col = cell // SIZE, cell % SIZE

        if cell == SIZE * SIZE:
            if count == 16:
                solutions.append([row.copy() for row in current_grid])
                max_diagonals = max(max_diagonals, count)
            return

        # Try no diagonal
        backtrack(cell + 1,count)

        #Try '/' diagonal if valid
        if is_valid(row, col, '/'):
            mark_diagonal(row, col, '/', True)
            current_grid[row][col] = '/'
            backtrack(cell +1, count + 1)
            mark_diagonal(row, col, '/', False)
            current_grid[row][col] = ' '
            
        # Try '\' diagonal if valid
        if is_valid(row, col, '\\'):
            mark_diagonal(row, col, '\\', True)
            current_grid[row][col] = '\\'
            backtrack(cell + 1, count +1)
            mark_diagonal(row, col, '\\', False)
            current_grid[row][col] = ' '

    backtrack(0,0)
    return solutions


# Usage
solutions = solve_grid()
print(f"Found {len(solutions)} optimal solutions:")
for idx, solution in enumerate(solutions, 1):
    print(f"\nSolution {idx}:")
    for row in solution:
        print(" ".join(row))
     
