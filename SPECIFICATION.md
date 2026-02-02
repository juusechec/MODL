# MODL: Model-Oriented Description Language

## Specification v1.0

### Design Principles

1. **Composicionalidad Semántica**: Cada construcción tiene significado unívoco y componible
2. **Economía Cognitiva**: Exactamente 7 construcciones principales (dentro del límite 7±2 de memoria de trabajo)
3. **Isomorfismo con Embeddings**: Estructuras que mapean naturalmente a espacios vectoriales
4. **Verificabilidad por Tipos**: Errores detectables sintácticamente

---

## Las 7 Construcciones Fundamentales

### 1. **Entity** (Entidad)
Define una entidad con tipo y atributos.

**Sintaxis:**
```modl
Entity <name>: <type> {
  <attribute>: <type> = <value>
}
```

**Semántica:** Representa un objeto del modelo con identidad única.

**Mapping a embeddings:** Cada entidad mapea a un vector en el espacio de embeddings, donde atributos son dimensiones.

**Ejemplo:**
```modl
Entity User: Person {
  name: String = "Alice"
  age: Integer = 30
  role: String = "Engineer"
}
```

---

### 2. **Relation** (Relación)
Define una relación entre entidades.

**Sintaxis:**
```modl
Relation <name>: <source> -> <target> {
  type: <relation_type>
  properties: { <key>: <value> }
}
```

**Semántica:** Expresa conexiones direccionales entre entidades.

**Mapping a embeddings:** Relaciones son transformaciones lineales entre espacios vectoriales.

**Ejemplo:**
```modl
Relation WorksAt: User -> Company {
  type: Employment
  properties: {
    since: "2020-01-01"
    position: "Senior Engineer"
  }
}
```

---

### 3. **Constraint** (Restricción)
Define reglas que deben cumplirse.

**Sintaxis:**
```modl
Constraint <name>: <entity_type> {
  condition: <boolean_expression>
  message: <string>
}
```

**Semántica:** Especifica invariantes del sistema verificables en tiempo de compilación o ejecución.

**Mapping a embeddings:** Restricciones definen regiones válidas en el espacio vectorial.

**Ejemplo:**
```modl
Constraint AgeLimit: Person {
  condition: age >= 0 && age <= 150
  message: "Age must be between 0 and 150"
}
```

---

### 4. **Transform** (Transformación)
Define operaciones sobre entidades.

**Sintaxis:**
```modl
Transform <name>: <input_type> => <output_type> {
  operation: <transformation_logic>
}
```

**Semántica:** Funciones puras que transforman datos preservando propiedades de tipo.

**Mapping a embeddings:** Funciones lineales o no-lineales en espacio de embeddings (como capas de red neuronal).

**Ejemplo:**
```modl
Transform Promote: Employee => Employee {
  operation: {
    salary = salary * 1.15
    level = level + 1
  }
}
```

---

### 5. **Pattern** (Patrón)
Define patrones estructurales reutilizables.

**Sintaxis:**
```modl
Pattern <name> {
  structure: <template>
  matches: <condition>
}
```

**Semántica:** Abstracción reutilizable que encapsula estructuras comunes.

**Mapping a embeddings:** Prototipos en el espacio vectorial que definen clusters semánticos.

**Ejemplo:**
```modl
Pattern Hierarchy {
  structure: {
    parent: Entity
    children: List<Entity>
  }
  matches: Relation(parent -> children)
}
```

---

### 6. **Composition** (Composición)
Combina construcciones en estructuras complejas.

**Sintaxis:**
```modl
Composition <name> {
  components: [<component_list>]
  interface: <public_interface>
}
```

**Semántica:** Permite construir sistemas complejos preservando composicionalidad semántica.

**Mapping a embeddings:** Concatenación o suma de vectores de componentes.

**Ejemplo:**
```modl
Composition Department {
  components: [
    Entity(Manager)
    List<Entity(Employee)>
    Relation(ReportsTo)
  ]
  interface: {
    getMembers: () => List<Employee>
    getManager: () => Manager
  }
}
```

---

### 7. **Query** (Consulta)
Define consultas declarativas sobre el modelo.

**Sintaxis:**
```modl
Query <name>: <result_type> {
  from: <source>
  where: <condition>
  select: <projection>
}
```

**Semántica:** Expresiones declarativas para extraer información del modelo.

**Mapping a embeddings:** Búsquedas por similaridad en espacio vectorial (e.g., nearest neighbors).

**Ejemplo:**
```modl
Query SeniorEngineers: List<Employee> {
  from: Employee
  where: level >= 5 && department == "Engineering"
  select: *
}
```

---

## Sistema de Tipos

### Tipos Primitivos
- `Integer`: Números enteros
- `Float`: Números de punto flotante
- `String`: Cadenas de texto
- `Boolean`: Valores verdadero/falso
- `Date`: Fechas y tiempos

### Tipos Compuestos
- `List<T>`: Lista de elementos de tipo T
- `Set<T>`: Conjunto de elementos únicos de tipo T
- `Map<K,V>`: Mapa de claves K a valores V
- `Option<T>`: Valor opcional de tipo T

### Tipos Definidos por Usuario
```modl
Type <name> = <definition>
```

**Ejemplo:**
```modl
Type Email = String where matches("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
Type PositiveInt = Integer where value > 0
```

---

## Reglas de Verificación de Tipos

1. **Unicidad de nombres**: No puede haber dos construcciones con el mismo nombre en el mismo ámbito.
2. **Compatibilidad de tipos**: Operaciones solo pueden aplicarse entre tipos compatibles.
3. **Completitud**: Todas las referencias deben resolverse.
4. **Consistencia de restricciones**: Las restricciones no pueden ser contradictorias.

---

## Gramática Formal (EBNF)

```ebnf
program        ::= construction*

construction   ::= entity | relation | constraint | transform | pattern | composition | query

entity         ::= "Entity" identifier ":" type "{" attribute* "}"
attribute      ::= identifier ":" type "=" value

relation       ::= "Relation" identifier ":" type "->" type "{" relation_body "}"
relation_body  ::= "type:" identifier properties?

constraint     ::= "Constraint" identifier ":" type "{" constraint_body "}"
constraint_body::= "condition:" expression "message:" string

transform      ::= "Transform" identifier ":" type "=>" type "{" transform_body "}"
transform_body ::= "operation:" "{" statement* "}"

pattern        ::= "Pattern" identifier "{" pattern_body "}"
pattern_body   ::= "structure:" structure "matches:" expression

composition    ::= "Composition" identifier "{" composition_body "}"
composition_body::= "components:" "[" component_list "]" "interface:" interface

query          ::= "Query" identifier ":" type "{" query_body "}"
query_body     ::= "from:" identifier "where:" expression "select:" projection

type           ::= primitive_type | composite_type | user_type
primitive_type ::= "Integer" | "Float" | "String" | "Boolean" | "Date"
composite_type ::= "List" "<" type ">" | "Set" "<" type ">" | "Map" "<" type "," type ">" | "Option" "<" type ">"

identifier     ::= [a-zA-Z_][a-zA-Z0-9_]*
value          ::= number | string | boolean | date
expression     ::= <boolean expression>
```

---

## Propiedades del Lenguaje

### Composicionalidad
- El significado de una construcción compuesta es función del significado de sus componentes
- No hay efectos secundarios implícitos
- Las transformaciones son funciones puras

### Carga Cognitiva
- 7 construcciones principales = límite óptimo de memoria de trabajo (Miller, 1956)
- Sintaxis uniforme reduce carga cognitiva
- Nombres descriptivos facilitan comprensión

### Embedding-Friendly
- Cada construcción tiene representación vectorial natural
- Relaciones son transformaciones lineales
- Composiciones son operaciones vectoriales
- Queries son búsquedas por similaridad

### Type Safety
- Sistema de tipos estático
- Inferencia de tipos cuando es posible
- Verificación en tiempo de compilación
- Mensajes de error informativos

---

## Referencias Teóricas

1. **Teoría de Lenguajes Formales**: Chomsky (1956), estructura BNF
2. **Ciencia Cognitiva**: Miller (1956), "The Magical Number Seven, Plus or Minus Two"
3. **Arquitectura de LLMs**: Vaswani et al. (2017), "Attention Is All You Need"
4. **Teoría de Tipos**: Martin-Löf (1975), Type Theory
