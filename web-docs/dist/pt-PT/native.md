# Ligações Nativas e Extensibilidade em Aly

Aly suporta extensibilidade em tempo de execução e ligações de módulos nativos Rust através de traits de registo nativo e bibliotecas dinâmicas.

---

## 1. Integração com Rust

Para associar métodos Rust como funções nativas dentro de Aly, mapeia tipos Rust usando a representação `ValueData` em módulos Rust (como `src/native/`):

* **`ValueData::Int(i64)`**: Corresponde a `int`.
* **`ValueData::Float(f64)`**: Corresponde a `float`.
* **`ValueData::Str(String)`**: Corresponde a `string`.
* **`ValueData::Bool(bool)`**: Corresponde a `bool`.
* **`ValueData::Nil`**: Corresponde a `None`.

---

## 2. Plugins de Biblioteca Dinâmica (`plugin import`)

Plugins de biblioteca externos `.so` / `.dylib` / `.dll` podem ser carregados dinamicamente no interpretador usando:

```aly
plugin import "path/to/plugin"
```

Isto regista símbolos, métodos e estruturas externos no espaço de nomes de tempo de execução activo de Aly.
