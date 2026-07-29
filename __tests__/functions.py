def add(a, b):
    return a + b

def multiply(a, b):
    return a * b

def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

sum_val = add(10, 20)
prod = multiply(5, 6)
fact = factorial(5)

print(f"sum: {sum_val}")
print(f"prod: {prod}")
print(f"fact: {fact}")
