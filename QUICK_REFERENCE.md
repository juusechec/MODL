# MODL Quick Reference

Quick syntax reference for MODL (Model-Oriented Description Language).

---

## Core Constructs

### 1. Entidades (Entities)

```modl
@<nombre>[<tipo>]
  .<atributo>: <tipo> = <valor>
```

**Example:**
```modl
@usuario[persona]
  .nombre: texto
  .edad: número
  .activo: bool
```

---

### 2. Relaciones (Relations)

```modl
@<origen> --<verbo>--> @<destino>[<cardinalidad>]
```

**Cardinalities:**
- `1` - exactly one
- `*` - zero or more
- `1..*` - one or more
- `0..1` - zero or one
- `n..m` - between n and m

**Example:**
```modl
@usuario --crea--> @pedido
@pedido --contiene--> @producto[1..*]
```

---

### 3. Comportamientos (Behaviors)

```modl
cuando @<entidad>.<acción>(@<params>):
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

---

### 4. Restricciones (Constraints)

```modl
siempre: <condición>
nunca: <condición>
eventualmente: <condición>
```

**Example:**
```modl
siempre: @pedido.total == suma(@pedido.productos.precio)
nunca: @usuario[edad < 18] --compra--> @producto[categoria="alcohol"]
```

---

### 5. Composiciones (Compositions)

```modl
proceso[<Nombre>]:
  entrada: <parámetros>
  salida: <tipo> | error
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

## Types

### Primitive Types
- `texto` - strings
- `número` - numbers (int/float)
- `bool` - boolean
- `fecha` - dates

### Composite Types
- `lista[<tipo>]` - ordered list
- `conjunto[<tipo>]` - set
- `mapa[<K>, <V>]` - map/dictionary
- `opcional[<tipo>]` - nullable

### Type Refinement
```modl
@producto[categoria="alcohol"]  # refinement
@usuario[edad >= 18]            # dependent type
```

---

## Operators

### Logical
```modl
AND, &&     # conjunction
OR, ||      # disjunction
NOT, !      # negation
```

### Comparison
```modl
==          # equality
!=          # inequality
<, >, <=, >=  # comparisons
```

### Arithmetic
```modl
+, -, *, /, %
```

### Assignment
```modl
:=          # assignment (not =)
```

### Collection
```modl
suma(<collection>)              # sum
<collection>.contiene(<item>)   # contains
<collection>.mapea(<fn>)        # map
<collection>.filtra(<fn>)       # filter
todos(<collection>, <fn>)       # all/every
alguno(<collection>, <fn>)      # some/any
```

---

## Control Flow

### Conditional
```modl
si <condición>: <acción>
si <condición>: <acción>
sino: <otra_acción>
```

### Iteration
```modl
para cada @item en @collection:
  <operación>
```

### Pattern Matching
```modl
cuando @entidad.método():
  caso @entidad.estado == "pendiente": <acción>
  caso @entidad.estado == "procesando": <acción>
  caso otro: <acción_default>
```

---

## Temporal Operators

```modl
siempre: <expr>       # invariant (always true)
nunca: <expr>         # forbidden (never true)
eventualmente: <expr> # eventually true
```

---

## Comments

```modl
# Single line comment

@usuario[persona]  # Inline comment
  .nombre: texto
```

---

## Functions

### Built-in Functions

```modl
ahora()                    # current timestamp
suma(<collection>)         # sum of numbers
promedio(<collection>)     # average
cuenta(<collection>)       # count items
max(<collection>)          # maximum
min(<collection>)          # minimum
concatenar(<strings>)      # join strings
```

---

## Complete Example

```modl
# Entities
@usuario[persona]
  .nombre: texto
  .email: texto
  .edad: número
  .activo: bool

@producto[item]
  .nombre: texto
  .precio: número
  .stock: número
  .categoria: texto

@pedido[transaccion]
  .id: número
  .fecha: fecha
  .estado: texto
  .total: número

# Relations
@usuario --crea--> @pedido
@pedido --contiene--> @producto[1..*]

# Constraints
siempre: @pedido.total == suma(@pedido.productos.precio)
siempre: @producto.stock >= 0
nunca: @usuario[edad < 18] --compra--> @producto[categoria="alcohol"]

# Behavior
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

# Process
proceso[CompletarCompra]:
  entrada: @usuario, @pedido, @metodoPago
  salida: @transaccion | error
  pasos:
    1. validar(@usuario.activo) -> ok | error["usuario_inactivo"]
    2. validar(@pedido.productos.stock) -> ok | error["sin_stock"]
    3. si ok: procesar_pago() -> @transaccion
    4. si fallo: lanzar(error["pago_rechazado"])
```

---

## Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Entities | Singular noun | `@usuario`, `@pedido` |
| Relations | Third-person verb | `--crea-->`, `--contiene-->` |
| Processes | Infinitive verb | `[ProcesarPago]` |
| Attributes | camelCase | `.nombreCompleto` |
| Types | lowercase | `texto`, `número` |

---

## Style Guide

### Indentation
```modl
@usuario[persona]
  .nombre: texto        # 2 spaces
  .edad: número
```

### Line Length
- Maximum 100 characters per line

### Spacing
```modl
# Good
@usuario --crea--> @pedido
siempre: @pedido.total > 0

# Avoid
@usuario--crea-->@pedido
siempre:@pedido.total>0
```

---

## Common Patterns

### Validation
```modl
Constraint Valid<Thing>: <Type> {
  condición: <boolean_expr>
  mensaje: "<error_message>"
}
```

### State Machine
```modl
@entity[type]
  .estado: texto

cuando @entity.transition():
  requiere: @entity.estado == "state1"
  ejecuta: @entity.estado := "state2"
  garantiza: @entity.estado == "state2"
```

### Builder
```modl
proceso[Build<Thing>]:
  entrada: <params>
  salida: @thing
  pasos:
    1. crear @thing
    2. configurar @thing con <params>
    3. validar @thing
    4. retornar @thing
```

### Repository
```modl
proceso[Buscar<Entity>]:
  entrada: criterios
  salida: lista[@entity]
  pasos:
    1. @entities := obtener_todos()
    2. @filtered := @entities.filtra(e -> cumple(e, criterios))
    3. retornar @filtered
```

---

## Cheat Sheet

| Want to... | Use... |
|-----------|--------|
| Define object | `@nombre[tipo] .attr: tipo` |
| Connect objects | `@a --verbo--> @b` |
| Add rule | `siempre:` / `nunca:` |
| Define operation | `cuando @obj.método():` |
| Build process | `proceso[Nombre]:` |
| Assign value | `:=` |
| Compare | `==`, `!=`, `<`, `>` |
| Logic | `AND`, `OR`, `NOT` |
| Comment | `#` |
| Type refinement | `@obj[condition]` |

---

## Error Messages

### Common Errors

```modl
# Type mismatch
@pedido.total := "cincuenta"
# Error: tipo incompatible (texto vs número)

# Cardinality violation
@pedido --contiene--> []
# Error: cardinalidad 1..* requiere al menos un elemento

# Precondition violation
cuando @usuario[activo=false].crea(@pedido)
# Error: precondición no cumplida (requiere: @usuario.activo == true)

# Constraint violation
@usuario[edad=15] --compra--> @producto[categoria="alcohol"]
# Error: viola restricción "nunca: @usuario[edad < 18] --compra--> @producto[categoria='alcohol']"
```

---

## Meta-information (LLM Extension)

```modl
cuando @usuario.crea(@pedido):
  explicación: "<natural language description>"
  caso_borde: [<condition>] -> <action>
  relacionado: [<related_rules>]
  ejemplo: { <concrete instance> }
```

---

## See Also

- [SPECIFICATION.md](SPECIFICATION.md) - Complete language specification
- [ARCHITECTURE.md](ARCHITECTURE.md) - Design rationale
- [GETTING_STARTED.md](GETTING_STARTED.md) - Tutorial
- [examples/](examples/) - Code examples
