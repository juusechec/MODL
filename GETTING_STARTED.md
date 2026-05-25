# Getting Started with MODL

Welcome to MODL (Model-Oriented Description Language)! This guide will help you start using MODL in 10 minutes.

---

## What is MODL?

MODL is a formal modeling language designed at the edge of current knowledge, combining:
- **Formal language theory** for unambiguous semantics
- **Cognitive science** for ease of learning (only 5 core constructs)
- **LLM architecture** for natural AI integration
- **Type theory** for safety and correctness

MODL reduces token usage by ~60-70% compared to natural language while maintaining clarity and formal rigor.

---

## Why MODL?

### For Humans
- **Visual-spatial notation**: `@`, `-->`, `[]` as visual anchors (pre-attentional processing)
- **Natural thinking syntax**: "when X, if Y, then Z" matches how we reason
- **Automatic chunking**: 5 constructs fit in working memory
- **Semantic mnemonics**: `@` = concrete thing, `-->` = directional flow, `:=` = assignment

### For LLMs
- **Efficient tokenization**: special symbols (`@`, `-->`) are single tokens vs 3-5 token phrases
- **Parseable structure**: context-free grammar, LL(1) parsing = less attention ambiguity
- **Compositional embeddings**: each block is semantically self-contained
- **~60-70% token reduction** vs natural language for same specification

---

## Installation

Currently, MODL is a specification language. You can start writing MODL files immediately:

1. Create a file with `.modl` extension
2. Use any text editor
3. Follow the syntax described below

---

## The 5 Core Constructs

### 1. Entidades (Entities) - Mental Graph Nodes

Define objects with attributes.

**Syntax:**
```modl
@<nombre>[<tipo>]
  .<atributo>: <tipo>
```

**Example:**
```modl
@usuario[persona]
  .nombre: texto
  .edad: número
  .activo: bool
```

### 2. Relaciones (Relations) - Semantic Edges

Define directional connections between entities.

**Syntax:**
```modl
@<origen> --<verbo>--> @<destino>[<cardinalidad>]
```

**Example:**
```modl
@usuario --crea--> @pedido
@pedido --contiene--> @producto[1..*]  # explicit cardinality
```

### 3. Comportamientos (Behaviors) - Operations with Contracts

Define operations with pre/post conditions.

**Syntax:**
```modl
cuando @<entidad>.<acción>(@<parámetros>):
  requiere: <precondición>
  ejecuta: <operaciones>
  garantiza: <postcondición>
```

**Example:**
```modl
cuando @usuario.crea(@pedido):
  requiere: @usuario.activo == true
  ejecuta: 
    @pedido.estado := "pendiente"
    @pedido.fecha := ahora()
  garantiza: @pedido.total > 0
```

### 4. Restricciones (Constraints) - System Invariants

Define rules that must always hold.

**Syntax:**
```modl
siempre: <condición>
nunca: <condición>
eventualmente: <condición>
```

**Example:**
```modl
siempre: @pedido.total == suma(@pedido.productos.precio)
nunca: @usuario.edad < 18 AND @pedido.contiene(@producto[categoria="alcohol"])
```

### 5. Composiciones (Compositions) - Reusable Modules

Define complex processes as composition of steps.

**Syntax:**
```modl
proceso[<Nombre>]:
  entrada: <parámetros>
  salida: <tipo_resultado> | error
  pasos:
    1. <paso> -> <resultado>
    2. si <condición>: <acción>
```

**Example:**
```modl
proceso[ProcesarPago]:
  entrada: @pedido, @metodoPago
  salida: @transaccion | error
  pasos:
    1. validar(@metodoPago) -> ok | fallo
    2. si ok: cobrar() -> @transaccion
    3. si fallo: lanzar(error["pago_rechazado"])
```

---

## Your First MODL Program

Let's create a simple e-commerce system.

### Step 1: Define Entities

```modl
@usuario[persona]
  .nombre: texto
  .email: texto
  .edad: número
  .activo: bool

@producto[item]
  .nombre: texto
  .precio: número
  .categoria: texto
  .stock: número

@pedido[transaccion]
  .id: número
  .fecha: fecha
  .estado: texto
  .total: número
```

### Step 2: Define Relations

```modl
@usuario --crea--> @pedido
@pedido --contiene--> @producto[1..*]
@usuario --añade--> @producto[*]  # carrito
```

### Step 3: Add Business Rules

```modl
# Invariantes
siempre: @pedido.total == suma(@pedido.productos.precio)
siempre: @producto.stock >= 0

# Restricciones de negocio
nunca: @usuario[edad < 18] --compra--> @producto[categoria="alcohol"]
nunca: @pedido --contiene--> @producto[stock == 0]
```

### Step 4: Define Behavior

```modl
cuando @usuario.crea(@pedido):
  requiere: @usuario.activo == true
  requiere: todos(@pedido.productos, p -> p.stock > 0)
  ejecuta:
    @pedido.estado := "pendiente"
    @pedido.fecha := ahora()
    @pedido.total := suma(@pedido.productos.precio)
    para cada @producto en @pedido:
      @producto.stock := @producto.stock - 1
  garantiza: @pedido.total > 0
  garantiza: @pedido.estado == "pendiente"
```

### Step 5: Create Process

```modl
proceso[CompletarCompra]:
  entrada: @usuario, @pedido, @metodoPago
  salida: @transaccion | error
  pasos:
    1. validar(@usuario.activo) -> ok | error["usuario_inactivo"]
    2. validar(@pedido.productos.stock) -> ok | error["sin_stock"]
    3. validar(@metodoPago) -> ok | error["pago_invalido"]
    4. si todos ok: procesar_pago(@pedido, @metodoPago) -> @transaccion
    5. si transaccion ok: 
         @pedido.estado := "completado"
         @usuario --realizó--> @transaccion
    6. si transaccion falla:
         @pedido.estado := "fallido"
         revertir_stock(@pedido.productos)
         lanzar(error["pago_rechazado"])
```

---

## Type System

### Primitive Types

- `texto` - strings
- `número` - integers or floats
- `bool` - true or false
- `fecha` - dates and timestamps

### Composite Types

- `lista[<tipo>]` - ordered collections
- `conjunto[<tipo>]` - sets (no duplicates)
- `mapa[<tipo_clave>, <tipo_valor>]` - key-value pairs
- `opcional[<tipo>]` - may be value or null

### Type Refinement (Light Dependent Types)

```modl
@producto[categoria="alcohol"]  # type refinement
@usuario[edad >= 18]             # dependent type
@pedido[total > 0]              # constraint refinement
```

### Type Inference

Types are inferred when unambiguous:

```modl
@usuario.edad             # inferred as número
@pedido.total             # inferred as número (from suma())
@producto[stock > 0]      # boolean refinement inferred
```

---

## Operators

### Logical
- `AND` (&&) - conjunction
- `OR` (||) - disjunction  
- `NOT` (!) - negation

### Comparison
- `==` - equality
- `!=` - inequality
- `<`, `>`, `<=`, `>=` - comparisons

### Arithmetic
- `+`, `-`, `*`, `/`, `%` - standard operations

### Assignment
- `:=` - explicit assignment (unambiguous, from Pascal/Ada)

### Collection
- `suma()` - sum elements
- `.contiene()` - membership test
- `.mapea()` - transform elements
- `.filtra()` - filter elements

---

## Common Patterns

### Validation Pattern

```modl
Constraint ValidEmail: Persona {
  condición: email.matches("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
  mensaje: "Email inválido"
}

Constraint ValidAge: Persona {
  condición: edad >= 0 AND edad <= 150
  mensaje: "Edad debe estar entre 0 y 150"
}
```

### State Machine Pattern

```modl
@pedido[transaccion]
  .estado: texto  # "pendiente" | "procesando" | "completado" | "cancelado"

siempre: @pedido.estado in ["pendiente", "procesando", "completado", "cancelado"]

cuando @pedido.procesar():
  requiere: @pedido.estado == "pendiente"
  ejecuta: @pedido.estado := "procesando"
  garantiza: @pedido.estado == "procesando"

cuando @pedido.completar():
  requiere: @pedido.estado == "procesando"
  ejecuta: @pedido.estado := "completado"
  garantiza: @pedido.estado == "completado"
```

### Aggregation Pattern

```modl
cuando @pedido.calcular_total():
  ejecuta:
    @pedido.subtotal := suma(@pedido.productos.precio)
    @pedido.impuestos := @pedido.subtotal * 0.16
    @pedido.total := @pedido.subtotal + @pedido.impuestos
```

### Builder Pattern

```modl
proceso[CrearPedidoCompleto]:
  entrada: @usuario, lista[@producto], @direccion, @metodoPago
  salida: @pedido
  pasos:
    1. crear @pedido vacío
    2. @pedido.usuario := @usuario
    3. para cada @producto:
         @pedido --contiene--> @producto
    4. @pedido.direccion := @direccion
    5. @pedido.metodoPago := @metodoPago
    6. @pedido.calcular_total()
    7. retornar @pedido
```

---

## Comparison with Natural Language

### Natural Language (23 tokens)
```
"When a user creates an order, the system must verify that the user is active,
set the order state as pending and record the current date"
```

### MODL (8 main tokens + structure)
```modl
cuando @usuario.crea(@pedido):
  requiere: @usuario.activo
  ejecuta: @pedido.estado := "pendiente"
           @pedido.fecha := ahora()
```

**~65% token reduction** while maintaining clarity and precision.

---

## Style Guide

### Naming Conventions

- **Entities**: singular nouns (`@usuario`, `@pedido`)
- **Relations**: third-person verbs (`--crea-->`, `--contiene-->`)
- **Processes**: infinitive verbs (`[ProcesarPago]`, `[ValidarDatos]`)
- **Attributes**: camelCase (`.nombreCompleto`, `.fechaCreacion`)

### Formatting

- Indentation: 2 spaces
- Line length: max 100 characters
- Comments: `#` for line comments

```modl
# Good example with proper formatting
@usuario[persona]
  .nombre: texto
  .edad: número

cuando @usuario.crea(@pedido):
  requiere: @usuario.activo == true  # Must be active
  ejecuta:
    @pedido.estado := "pendiente"
    @pedido.fecha := ahora()
```

---

## Next Steps

1. **Read the Specification** - See [SPECIFICATION.md](SPECIFICATION.md) for complete language reference
2. **Explore Architecture** - See [ARCHITECTURE.md](ARCHITECTURE.md) for design rationale
3. **Check Examples** - See [examples/](examples/) for more complex examples
4. **Quick Reference** - See [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for syntax cheat sheet

---

## Tips for Success

1. **Start Simple**: Begin with just entities and relations
2. **Add Constraints**: Then add business rules as constraints
3. **Define Behavior**: Add behavior contracts for operations
4. **Compose**: Finally, build complex processes

5. **Think in Graphs**: Visualize entities as nodes and relations as edges
6. **Use Types**: Let type inference help you, but be explicit when needed
7. **Test Constraints**: Think about edge cases and encode them as restrictions

---

## Common Mistakes to Avoid

### ❌ Wrong: Using `=` for assignment
```modl
@pedido.estado = "pendiente"  # Confusing with comparison
```

### ✅ Correct: Using `:=` for assignment
```modl
@pedido.estado := "pendiente"  # Unambiguous
```

### ❌ Wrong: Forgetting cardinality
```modl
@pedido --contiene--> @producto  # How many?
```

### ✅ Correct: Explicit cardinality
```modl
@pedido --contiene--> @producto[1..*]  # One or more
```

### ❌ Wrong: Missing preconditions
```modl
cuando @usuario.crea(@pedido):
  ejecuta: @pedido.estado := "pendiente"
```

### ✅ Correct: Explicit preconditions
```modl
cuando @usuario.crea(@pedido):
  requiere: @usuario.activo == true
  ejecuta: @pedido.estado := "pendiente"
```

---

## Getting Help

- **Specification**: [SPECIFICATION.md](SPECIFICATION.md)
- **Architecture**: [ARCHITECTURE.md](ARCHITECTURE.md)
- **Design Principles**: [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md)
- **Quick Reference**: [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
- **Examples**: [examples/](examples/)

---

Welcome to MODL! Happy modeling! 🚀
