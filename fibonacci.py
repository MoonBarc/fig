iterations = 1_000_000

for _ in range(iterations):
    n = 20
    a, b = 0, 1
    for _ in range(n - 1):
        a, b = b, a + b
