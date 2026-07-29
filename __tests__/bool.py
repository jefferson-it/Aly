idade = 18
adulto = idade >= 18

print(f"Tem {idade} e adulto = {str(adulto).lower()}")

print(str(idade > 10).lower())
print(str(idade == 10).lower())
print(str(idade == 18).lower())
print(str(idade < 18).lower())
print(str(idade <= 19).lower())
print(str(idade <= 10).lower())
print(str(idade >= 20).lower())
print(str(idade >= int("18")).lower())
print(str(idade >= 18 and idade > 60).lower())
print(str(idade >= 18 and idade <= 60).lower())
