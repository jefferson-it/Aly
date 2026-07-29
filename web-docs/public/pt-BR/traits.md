# Traits no Aly

As traits definem interfaces — um conjunto de assinaturas de métodos que um schema pode implementar. Elas habilitam polimorfismo e contratos de código.

---

## 1. Definindo uma Trait

Uma trait declara nomes de métodos e suas listas de parâmetros:

```aly
trait Drawable {
    fun draw(canvas)
    fun resize(w, h)
}
```

---

## 2. Implementando uma Trait

Um schema implementa uma trait definindo todos os métodos exigidos:

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

O sistema de traits em tempo de execução (`TraitDef`) verifica se todos os métodos da trait existem no schema (incluindo os herdados).

---

## 3. Verificação de Trait

Em tempo de execução, `is_implemented_by` verifica se um schema atende a uma trait ao confirmar que cada método declarado na trait existe no schema ou em sua cadeia de pais (parent chain).
