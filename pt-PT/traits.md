# Traits em Aly

Traits definem interfaces — um conjunto de assinaturas de métodos que um schema pode implementar. Permitem polimorfismo e contratos de código.

---

## 1. Definição de um Trait

Um trait declara nomes de métodos e as suas listas de parâmetros:

```aly
trait Drawable {
    fun draw(canvas)
    fun resize(w, h)
}
```

---

## 2. Implementação de um Trait

Um schema implementa um trait definindo todos os métodos obrigatórios:

```aly
schema Circle {
    radius: 10,
    
    fun draw(canvas) {
        print("Drawing circle at radius " + @radius)
    }
    
    fun resize(w, h) {
        @radius = w
    }
}
```

O sistema de traits de tempo de execução (`TraitDef`) verifica que todos os métodos do trait existem no schema (incluindo os herdados).

---

## 3. Verificação de Trait

Em tempo de execução, `is_implemented_by` verifica se um schema satisfaz um trait verificando que cada método declarado no trait existe no schema ou na cadeia de progenitores.
