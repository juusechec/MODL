# MODL: Model-Oriented Description Language

## Specification v1.0

### Design Principles

1. **Composicionalidad Semántica**: Cada construcción tiene significado unívoco y componible
2. **Economía Cognitiva**: Máximo 7±2 construcciones principales (solo 5 en la versión base)
3. **Isomorfismo con Embeddings**: Estructuras que mapean naturalmente a espacios vectoriales
4. **Verificabilidad por Tipos**: Errores detectables sintácticamente

---

## Las 5 Construcciones Fundamentales

### 1. **Entidades** - Nodos en el grafo mental

Define una entidad con tipo y atributos.

**Sintaxis:**
```modl
@<nombre>[<tipo>]
  .<atributo>: <tipo>
```

**Semántica:** Representa un objeto del modelo con identidad única, funcionando como nodo en el grafo mental.

**Mapping a embeddings:** Cada entidad mapea a un vector en el espacio de embeddings, donde los atributos son dimensiones.

**Ejemplo:**
```modl
@usuario[persona]
  .nombre: texto
  .edad: número
  .activo: bool
```

---

### 2. **Relaciones** - Aristas con semántica

Define una relación direccional entre entidades.

**Sintaxis:**
```modl
@<entidad_origen> --<verbo>--> @<entidad_destino>[<cardinalidad>]
```

**Semántica:** Expresa conexiones direccionales entre entidades con cardinalidad explícita.

**Mapping a embeddings:** Relaciones son transformaciones lineales o matrices de atención entre espacios vectoriales.

**Ejemplo:**
```modl
@usuario --crea--> @pedido
@pedido --contiene--> @producto[1..*]  # cardinalidad explícita
```

**Cardinalidad:**
- `1` - exactamente uno
- `*` - cero o más
- `1..*` - uno o más
- `0..1` - cero o uno

---

### 3. **Comportamientos** - Verbos con pre/post condiciones

Define operaciones sobre entidades con contratos formales.

**Sintaxis:**
```modl
cuando @<entidad>.<acción>(@<parámetros>):
  requiere: <precondición>
  ejecuta: 
    <operaciones>
  garantiza: <postcondición>
```

**Semántica:** Especifica comportamientos con contratos de pre y post condiciones verificables.

**Mapping a embeddings:** Comportamientos son funciones diferenciables en el espacio vectorial.

**Ejemplo:**
```modl
cuando @usuario.crea(@pedido):
  requiere: @usuario.activo == true
  ejecuta: 
    @pedido.estado := "pendiente"
    @pedido.fecha := ahora()
  garantiza: @pedido.total > 0
```

---

### 4. **Restricciones** - Invariantes del sistema

Define reglas que deben cumplirse en todo momento.

**Sintaxis:**
```modl
siempre: <condición>
nunca: <condición>
eventualmente: <condición>
```

**Semántica:** Especifica invariantes del sistema con operadores temporales simples.

**Mapping a embeddings:** Restricciones definen regiones válidas en el espacio vectorial.

**Ejemplo:**
```modl
siempre: @pedido.total == suma(@pedido.productos.precio)
nunca: @usuario.edad < 18 AND @pedido.contiene(@producto[categoria="alcohol"])
```

---

### 5. **Composiciones** - Módulos reutilizables

Define procesos complejos como composición de pasos.

**Sintaxis:**
```modl
proceso[<Nombre>]:
  entrada: <parámetros>
  salida: <tipo_resultado> | error
  pasos:
    1. <paso> -> <resultado>
    2. si <condición>: <acción>
    3. si <condición>: <acción_alternativa>
```

**Semántica:** Permite definir flujos de trabajo complejos de manera modular y reutilizable.

**Mapping a embeddings:** Composiciones son agregaciones jerárquicas de embeddings.

**Ejemplo:**
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

## Sistema de Tipos

### Tipos Primitivos

- `texto` (String) - cadenas de texto
- `número` (Number) - enteros o flotantes
- `bool` (Boolean) - verdadero o falso
- `fecha` (Date) - fechas y timestamps

### Tipos Compuestos

- `lista[<tipo>]` - colecciones ordenadas
- `conjunto[<tipo>]` - colecciones sin duplicados
- `mapa[<tipo_clave>, <tipo_valor>]` - pares clave-valor
- `opcional[<tipo>]` - puede ser valor o nulo

### Tipos Dependientes Light

Refinamiento de tipos con condiciones:

```modl
@producto[categoria="alcohol"]  # refinamiento de tipo
@usuario[edad >= 18]             # tipo dependiente
```

### Inferencia de Tipos

El sistema infiere tipos automáticamente cuando es posible:

```modl
@usuario.edad      # inferido como número
@pedido.total      # inferido como número (por operación suma())
@producto[stock > 0]  # refinamiento booleano inferido
```

---

## Gramática Formal

### EBNF Specification

```ebnf
(* MODL Grammar *)

program = { declaration } ;

declaration = entity_decl 
            | relation_decl 
            | behavior_decl 
            | constraint_decl 
            | process_decl ;

(* Entidades *)
entity_decl = "@", identifier, "[", type_spec, "]", 
              { attribute } ;

attribute = ".", identifier, ":", type_spec ;

(* Relaciones *)
relation_decl = entity_ref, "--", verb, "-->", entity_ref,
                [ cardinality ] ;

entity_ref = "@", identifier ;
verb = identifier ;
cardinality = "[", card_expr, "]" ;
card_expr = number, [ "..", ( number | "*" ) ] ;

(* Comportamientos *)
behavior_decl = "cuando", entity_ref, ".", identifier, 
                "(", [ params ], ")", ":",
                [ requires_clause ],
                executes_clause,
                [ ensures_clause ] ;

requires_clause = "requiere:", expression ;
executes_clause = "ejecuta:", { statement } ;
ensures_clause = "garantiza:", expression ;

(* Restricciones *)
constraint_decl = temporal_operator, ":", expression ;
temporal_operator = "siempre" | "nunca" | "eventualmente" ;

(* Procesos *)
process_decl = "proceso", "[", identifier, "]", ":",
               "entrada:", param_list,
               "salida:", type_spec, [ "|", "error" ],
               "pasos:", { step } ;

step = number, ".", statement, [ "->", result ] ;

(* Expresiones y tipos *)
expression = identifier 
           | literal
           | binary_expr
           | function_call
           | entity_ref, ".", identifier ;

type_spec = primitive_type | composite_type | user_type ;
primitive_type = "texto" | "número" | "bool" | "fecha" ;
composite_type = "lista" | "conjunto" | "mapa" | "opcional" ;

identifier = letter, { letter | digit | "_" } ;
number = digit, { digit } ;
literal = string | number | boolean ;
```

---

## Operadores

### Operadores Lógicos

- `AND` (&&) - conjunción lógica
- `OR` (||) - disyunción lógica
- `NOT` (!) - negación lógica

### Operadores de Comparación

- `==` - igualdad
- `!=` - desigualdad
- `<` - menor que
- `>` - mayor que
- `<=` - menor o igual
- `>=` - mayor o igual

### Operadores Aritméticos

- `+` - suma
- `-` - resta
- `*` - multiplicación
- `/` - división
- `%` - módulo

### Operador de Asignación

- `:=` - asignación (inequívoca, inspirada en Pascal/Ada)

### Operadores de Colección

- `suma()` - suma de elementos
- `.contiene()` - verificación de pertenencia
- `.mapea()` - transformación de elementos
- `.filtra()` - filtrado de elementos

---

## Semántica Formal

### Notación Visual-Espacial

El lenguaje utiliza anclajes visuales para facilitar el procesamiento:

- `@` - indica cosa concreta (entidad)
- `-->` - indica flujo direccional (relación)
- `[]` - indica refinamiento o parametrización
- `:=` - indica asignación explícita

### Semántica de Grafos

MODL expresa modelos como grafos:

```
@entidad --> @entidad  ≅  (sujeto, predicado, objeto)
```

Esta estructura mapea directamente a:
- Tripletas RDF en Knowledge Graphs
- Grafos de atención en Transformers
- Grafos de dependencias en análisis sintáctico

### Verificación Estática

El lenguaje permite verificar en tiempo de compilación:

- **Consistencia de tipos**: todos los tipos son compatibles
- **Validez de relaciones**: las entidades relacionadas existen
- **Cumplimiento de restricciones**: las invariantes se mantienen
- **Cardinalidades**: las relaciones respetan sus cardinalidades
- **Ciclos imposibles**: no hay dependencias circulares inválidas
- **Estados inalcanzables**: todos los estados son alcanzables

---

## Extensión para LLMs (Meta-capa)

MODL permite que los LLMs generen metadatos y explicaciones:

```modl
cuando @usuario.crea(@pedido):
  explicación: "Esta regla de negocio asegura que solo usuarios 
                autenticados puedan realizar pedidos"
  caso_borde: [@usuario.activo == false] -> mostrar("Por favor active su cuenta")
  relacionado: [regla_activacion_usuarios, proceso_verificacion]
```

**Campos de meta-información:**
- `explicación` - descripción en lenguaje natural
- `caso_borde` - situaciones límite y su manejo
- `relacionado` - enlaces a otras reglas o procesos
- `ejemplo` - instancias concretas

---

## Vector Space Mapping

### Entidades → Vectores Densos

```modl
@usuario[persona]
  .nombre: texto = "Alice"
  .edad: número = 30
  .rol: texto = "Engineer"

Embedding: [e_nombre, e_edad, e_rol, ...] ∈ ℝⁿ
```

### Relaciones → Transformaciones

```modl
@usuario --crea--> @pedido

Transformación: T: ℝⁿᵤ → ℝⁿₚ
donde T puede ser atención o transformación lineal
```

### Restricciones → Regiones Válidas

```modl
siempre: @usuario.edad >= 18

Define: V = {v ∈ ℝⁿ | edad(v) ≥ 18}
```

### Composiciones → Agregación Jerárquica

```modl
proceso[ProcesarPago]:
  entrada: @pedido, @metodoPago
  
Embedding: [e_pedido; e_metodoPago; e_contexto]
```

---

## Compatibilidad con Arquitecturas de IA

### Transformers

- Entidades como tokens con embeddings
- Relaciones como mecanismos de atención
- Restricciones como máscaras de atención
- Composiciones como capas jerárquicas

### Graph Neural Networks (GNN)

- Grafo MODL → grafo computacional directo
- Entidades → nodos
- Relaciones → aristas
- Propagación de mensajes natural

### Embeddings Semánticos

- Similitud coseno para búsqueda
- Clustering para patrones
- t-SNE/UMAP para visualización

---

## Convenciones de Estilo

### Nomenclatura

- **Entidades**: sustantivos en singular (`@usuario`, `@pedido`)
- **Relaciones**: verbos en tercera persona (`--crea-->`, `--contiene-->`)
- **Procesos**: verbos en infinitivo (`[ProcesarPago]`, `[ValidarDatos]`)
- **Atributos**: minúsculas con camelCase (`.nombreCompleto`, `.fechaCreacion`)

### Formato

- Indentación: 2 espacios
- Líneas: máximo 100 caracteres
- Comentarios: `#` para comentarios de línea

### Organización de Archivos

```
proyecto.modl
├── entidades/
│   ├── usuario.modl
│   └── pedido.modl
├── relaciones/
│   └── negocio.modl
├── procesos/
│   └── pagos.modl
└── restricciones/
    └── validaciones.modl
```

---

## Conclusión

MODL v1.0 proporciona una especificación completa para modelado formal que:

1. Es cognitivamente eficiente (5 construcciones principales)
2. Mapea naturalmente a espacios vectoriales
3. Permite verificación formal
4. Es compatible con arquitecturas de IA modernas
5. Mantiene legibilidad para humanos

La próxima versión incluirá herramientas de parsing, validación y conversión a formatos estándar.
