# Traits in Aly

Traits define interfaces — a set of method signatures that a schema can implement. They enable polymorphism and code contracts.

---

## 1. Defining a Trait

A trait declares method names and their parameter lists:

```aly
trait Drawable {
    fun draw(canvas)
    fun resize(w, h)
}
```

---

## 2. Implementing a Trait

A schema implements a trait by defining all required methods:

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

The runtime trait system (`TraitDef`) checks that all trait methods exist on the schema (including inherited ones).

---

## 3. Trait Verification

At runtime, `is_implemented_by` checks whether a schema satisfies a trait by verifying that every method declared in the trait exists on the schema or its parent chain.
