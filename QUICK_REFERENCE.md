# MODL Quick Reference Guide

A concise reference for the Model-Oriented Description Language.

---

## The 7 Core Constructs

### 1. Entity - Define Objects

```modl
Entity <name>: <type> {
  <attribute>: <type> = <value>
  ...
}
```

**Example:**
```modl
Entity User: Person {
  id: Integer = 1001
  name: String = "Alice"
  email: String = "alice@example.com"
  age: Integer = 30
}
```

---

### 2. Relation - Connect Objects

```modl
Relation <name>: <source> -> <target> {
  type: <relation_type>
  properties: {
    <key>: <value>
    ...
  }
}
```

**Example:**
```modl
Relation WorksAt: User -> Company {
  type: Employment
  properties: {
    since: "2020-01-01"
    position: "Engineer"
  }
}
```

---

### 3. Constraint - Validate Data

```modl
Constraint <name>: <entity_type> {
  condition: <boolean_expression>
  message: <error_message>
}
```

**Example:**
```modl
Constraint ValidAge: Person {
  condition: age >= 0 && age <= 150
  message: "Age must be between 0 and 150"
}
```

---

### 4. Transform - Modify Data

```modl
Transform <name>: <input_type> => <output_type> {
  operation: {
    <statements>
  }
}
```

**Example:**
```modl
Transform Promote: Employee => Employee {
  operation: {
    salary = salary * 1.15
    level = level + 1
  }
}
```

---

### 5. Pattern - Reusable Templates

```modl
Pattern <name> {
  structure: {
    <component_definitions>
  }
  matches: <matching_condition>
}
```

**Example:**
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

### 6. Composition - Build Systems

```modl
Composition <name> {
  components: [
    <component_list>
  ]
  interface: {
    <function_signatures>
  }
}
```

**Example:**
```modl
Composition Department {
  components: [
    Entity(Manager: Person)
    List<Entity(Employee: Person)>
    Relation(ReportsTo: Employee -> Manager)
  ]
  interface: {
    getMembers: () => List<Employee>
    addEmployee: (Employee) => Department
  }
}
```

---

### 7. Query - Retrieve Data

```modl
Query <name>: <result_type> {
  from: <source>
  where: <condition>
  select: <projection>
}
```

**Example:**
```modl
Query SeniorEngineers: List<Employee> {
  from: Employee
  where: level >= 5 && department == "Engineering"
  select: { id, name, level }
}
```

---

## Type System

### Primitive Types
- `Integer` - Whole numbers
- `Float` - Decimal numbers
- `String` - Text
- `Boolean` - true/false
- `Date` - Dates and timestamps

### Composite Types
- `List<T>` - Ordered collection
- `Set<T>` - Unordered unique collection
- `Map<K,V>` - Key-value pairs
- `Option<T>` - Optional value (may be null)

### Type Definition
```modl
Type <name> = <definition>
```

**Example:**
```modl
Type Email = String where matches("^[a-z]+@[a-z]+\\.[a-z]+$")
Type PositiveInt = Integer where value > 0
```

---

## Common Patterns

### Entity with Constraints
```modl
Entity Product: Item {
  id: Integer = 1
  name: String = "Laptop"
  price: Float = 999.99
}

Constraint PositivePrice: Product {
  condition: price > 0.0
  message: "Price must be positive"
}
```

### Related Entities
```modl
Entity Author: Person { name: String }
Entity Book: Publication { title: String }

Relation Wrote: Author -> Book {
  type: Authorship
  properties: { year: 2026 }
}
```

### Transformation Pipeline
```modl
Transform Step1: A => B { operation: {...} }
Transform Step2: B => C { operation: {...} }

# Compose: Step2(Step1(input))
```

### Query with Aggregation
```modl
Query Statistics: List<Stats> {
  from: Employee
  groupBy: department
  select: {
    dept: department
    count: count()
    avgSalary: avg(salary)
  }
}
```

---

## Operators

### Comparison
- `==` - Equals
- `!=` - Not equals
- `<` - Less than
- `<=` - Less or equal
- `>` - Greater than
- `>=` - Greater or equal

### Logical
- `&&` - AND
- `||` - OR
- `!` - NOT

### Arithmetic
- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `%` - Modulo

### String
- `.matches(pattern)` - Regex match
- `.contains(substring)` - Contains check
- `.trim()` - Remove whitespace
- `.toLowerCase()` - Convert to lowercase
- `.toUpperCase()` - Convert to uppercase

### Collection
- `.count()` - Number of elements
- `.all(predicate)` - All match condition
- `.any(predicate)` - Any match condition
- `.exists()` - Entity exists

---

## Functions

### Date/Time
- `currentDate()` - Current date/time
- `dateSubtract(date, unit)` - Subtract time

### Aggregation
- `sum(field)` - Sum values
- `avg(field)` - Average
- `min(field)` - Minimum
- `max(field)` - Maximum
- `count()` - Count items

### Embedding
- `cosineSimilarity(v1, v2)` - Vector similarity
- `euclideanDistance(v1, v2)` - Vector distance

---

## Best Practices

### 1. Use Descriptive Names
```modl
# Good
Entity Employee: Person { ... }

# Avoid
Entity E: P { ... }
```

### 2. Always Add Constraints
```modl
Entity Account: BankAccount {
  balance: Float = 0.0
}

Constraint NonNegativeBalance: Account {
  condition: balance >= 0.0
  message: "Balance cannot be negative"
}
```

### 3. Keep Transformations Pure
```modl
# Good - Pure function
Transform CalculateTotal: Order => Float {
  operation: { result = subtotal + tax }
}

# Avoid - Side effects
Transform SendEmail: Order => Order {
  operation: { sendNotification() }  # Not pure!
}
```

### 4. Use Composition for Complex Systems
```modl
# Build large systems from smaller components
Composition LargeSystem {
  components: [
    Composition(SubSystemA)
    Composition(SubSystemB)
    ...
  ]
  interface: { ... }
}
```

### 5. Leverage Patterns for Reuse
```modl
Pattern StandardWorkflow {
  structure: { ... }
  matches: ...
}

# Reuse pattern in multiple places
```

---

## Common Errors

### Type Mismatch
```modl
# ✗ Error
Entity User { age: Integer = "thirty" }

# ✓ Correct
Entity User { age: Integer = 30 }
```

### Invalid Relation
```modl
# ✗ Error - UndefinedType doesn't exist
Relation Works: User -> UndefinedType { ... }

# ✓ Correct - Both types defined
Relation Works: User -> Company { ... }
```

### Non-Boolean Constraint
```modl
# ✗ Error - condition must be Boolean
Constraint Bad: Person {
  condition: age  # Integer, not Boolean
}

# ✓ Correct
Constraint Good: Person {
  condition: age > 0  # Boolean expression
}
```

---

## Cheat Sheet

| Need to... | Use... |
|-----------|--------|
| Define an object | Entity |
| Connect objects | Relation |
| Validate data | Constraint |
| Modify data | Transform |
| Create template | Pattern |
| Build system | Composition |
| Retrieve data | Query |

---

## Resources

- [SPECIFICATION.md](SPECIFICATION.md) - Complete language specification
- [ARCHITECTURE.md](ARCHITECTURE.md) - Design rationale and theory
- [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md) - How principles are implemented
- [examples/](examples/) - Practical examples

---

## Quick Start

1. Define your entities
2. Add relations between them
3. Specify constraints
4. Create transformations
5. Build compositions
6. Write queries

That's it! You now know MODL.
