# __tests__/comprehensive_showcase.py
print("=== 1. Basic Variables and Arithmetic ===")
name = "Jefferson"
version = 2.0
age = 30
is_developer = True

print(f"Developer: {name}")
print(f"Age: {age}")
print(f"Interpreter Version: {version}")

result = 10 + 20 * 5 / 2 - 5
print(f"Math result (10 + 20 * 5 / 2 - 5): {int(result)}")

print("\n=== 2. Comparisons & Control Flow ===")
if age >= 18:
    print(f"{name} is an adult.")
else:
    print(f"{name} is a minor.")

count = 1
sum_val = 0
while count <= 5:
    sum_val = sum_val + count
    count = count + 1
print(f"Sum from 1 to 5: {sum_val}")

print("\n=== 3. Functions & Recursion ===")
def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

fact = factorial(5)
print(f"Factorial of 5: {fact}")

print("\n=== 4. Objects and Lists ===")
list_val = [1, 2, 3, 4]
print(f"List value: {list_val}")

print("\n=== 5. GUI Window Setup ===")
print("GUI Window is fully initialized. Opening loop...")
