# Getting Started with MODL

Welcome to MODL (Model-Oriented Description Language)! This guide will help you start using MODL in 10 minutes.

---

## What is MODL?

MODL is a formal modeling language that bridges human understanding and AI processing. It combines:
- **Formal language theory** for precise semantics
- **Cognitive science** for ease of learning (7 core constructs)
- **LLM architecture** for AI integration
- **Type theory** for safety and correctness

---

## Installation

Currently, MODL is a specification-only language. You can start writing MODL files immediately:

1. Create a file with `.modl` extension
2. Use any text editor
3. Follow the syntax described below

---

## Your First MODL Program

Let's create a simple model of a library system.

### Step 1: Define Entities

```modl
Entity Book: Publication {
  isbn: String = "978-0-123456-78-9"
  title: String = "Introduction to MODL"
  author: String = "Jane Doe"
  year: Integer = 2026
  available: Boolean = true
}

Entity Member: Person {
  id: Integer = 1001
  name: String = "John Smith"
  email: String = "john@example.com"
  joinDate: Date = "2026-01-15"
}
```

**What you learned:**
- Entities represent objects in your model
- Each attribute has a type and a value
- Syntax: `Entity Name: Type { attributes }`

### Step 2: Add Relations

```modl
Relation Borrows: Member -> Book {
  type: Loan
  properties: {
    borrowDate: "2026-02-01"
    dueDate: "2026-02-15"
    returned: false
  }
}
```

**What you learned:**
- Relations connect entities
- Syntax: `Relation Name: Source -> Target`
- Relations can have properties

### Step 3: Add Constraints

```modl
Constraint ValidYear: Book {
  condition: year >= 1450 && year <= 2100
  message: "Publication year must be between 1450 and 2100"
}

Constraint ValidEmail: Member {
  condition: email.matches("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
  message: "Invalid email format"
}
```

**What you learned:**
- Constraints validate data
- Conditions must be Boolean expressions
- Constraints provide error messages

### Step 4: Add Transformations

```modl
Transform ReturnBook: Book => Book {
  operation: {
    available = true
    lastReturned = currentDate()
  }
}

Transform RenewMembership: Member => Member {
  operation: {
    renewalDate = currentDate()
    active = true
  }
}
```

**What you learned:**
- Transforms modify entities
- They are pure functions (input → output)
- Syntax: `Transform Name: InputType => OutputType`

### Step 5: Add Queries

```modl
Query AvailableBooks: List<Book> {
  from: Book
  where: available == true
  select: { title, author, year }
}

Query OverdueLoans: List<Loan> {
  from: Borrows
  where: dueDate < currentDate() && returned == false
  select: {
    memberName: Member.name
    bookTitle: Book.title
    daysOverdue: currentDate() - dueDate
  }
}
```

**What you learned:**
- Queries retrieve data declaratively
- Use SQL-like syntax (from, where, select)
- Can compute derived values

---

## Complete Example

Putting it all together in `library.modl`:

```modl
# Entities
Entity Book: Publication {
  isbn: String = "978-0-123456-78-9"
  title: String = "Introduction to MODL"
  author: String = "Jane Doe"
  year: Integer = 2026
  available: Boolean = true
}

Entity Member: Person {
  id: Integer = 1001
  name: String = "John Smith"
  email: String = "john@example.com"
  joinDate: Date = "2026-01-15"
}

# Relations
Relation Borrows: Member -> Book {
  type: Loan
  properties: {
    borrowDate: "2026-02-01"
    dueDate: "2026-02-15"
    returned: false
  }
}

# Constraints
Constraint ValidYear: Book {
  condition: year >= 1450 && year <= 2100
  message: "Publication year must be between 1450 and 2100"
}

Constraint ValidEmail: Member {
  condition: email.matches("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
  message: "Invalid email format"
}

# Transforms
Transform ReturnBook: Book => Book {
  operation: {
    available = true
    lastReturned = currentDate()
  }
}

# Queries
Query AvailableBooks: List<Book> {
  from: Book
  where: available == true
  select: { title, author, year }
}

Query OverdueLoans: List<Loan> {
  from: Borrows
  where: dueDate < currentDate() && returned == false
  select: {
    memberName: Member.name
    bookTitle: Book.title
    daysOverdue: currentDate() - dueDate
  }
}
```

---

## The 7 Constructs at a Glance

You've now used 5 of the 7 core constructs! Here's the complete set:

1. ✓ **Entity** - Define objects
2. ✓ **Relation** - Connect objects
3. ✓ **Constraint** - Validate data
4. ✓ **Transform** - Modify data
5. **Pattern** - Create reusable templates
6. **Composition** - Build complex systems
7. ✓ **Query** - Retrieve data

### Using Pattern

```modl
Pattern Hierarchy {
  structure: {
    parent: Entity
    children: List<Entity>
  }
  matches: Relation(parent -> children)
}
```

Patterns let you define reusable structural templates.

### Using Composition

```modl
Composition LibrarySystem {
  components: [
    Entity(Book: Publication)
    Entity(Member: Person)
    Relation(Borrows: Member -> Book)
    Constraint(ValidYear: Book)
    Transform(ReturnBook: Book => Book)
    Query(AvailableBooks: List<Book>)
  ]
  interface: {
    borrowBook: (Member, Book) => Borrows
    returnBook: (Book) => Book
    searchBooks: (String) => List<Book>
    getOverdueLoans: () => List<Loan>
  }
}
```

Compositions let you build complete systems from smaller components.

---

## Next Steps

### 1. Explore Examples
Check out the [examples/](examples/) directory for more comprehensive examples:
- `01_entity.modl` through `07_query.modl` - Individual construct examples
- `complete_example.modl` - Full employee management system

### 2. Read Documentation
- [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Syntax cheat sheet
- [SPECIFICATION.md](SPECIFICATION.md) - Complete language spec
- [ARCHITECTURE.md](ARCHITECTURE.md) - Design rationale
- [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md) - How principles map to constructs

### 3. Try These Exercises

**Exercise 1: E-commerce System**
Model an e-commerce system with:
- Entities: Product, Customer, Order
- Relations: Places (Customer -> Order), Contains (Order -> Product)
- Constraints: Positive prices, valid emails
- Transforms: Checkout, Cancel order
- Queries: Find orders by customer

**Exercise 2: Social Network**
Model a social network with:
- Entities: User, Post, Comment
- Relations: Follows, Likes, Comments
- Constraints: Valid usernames, content length
- Transforms: Create post, Edit profile
- Queries: Feed generation, Popular posts

**Exercise 3: Task Management**
Model a task management system with:
- Entities: Task, User, Project
- Relations: AssignedTo, BelongsTo
- Constraints: Due dates, priorities
- Transforms: Complete task, Update status
- Queries: Overdue tasks, User workload

---

## Common Pitfalls

### 1. Forgetting Type Annotations
```modl
# ✗ Wrong
Entity User {
  name = "Alice"  # Missing type
}

# ✓ Correct
Entity User: Person {
  name: String = "Alice"
}
```

### 2. Non-Boolean Constraints
```modl
# ✗ Wrong
Constraint Check: User {
  condition: age  # age is Integer, not Boolean
}

# ✓ Correct
Constraint Check: User {
  condition: age > 0  # Boolean expression
}
```

### 3. Undefined Relations
```modl
# ✗ Wrong
Relation Works: User -> Company  # Company not defined

# ✓ Correct
Entity Company: Organization { ... }
Relation Works: User -> Company { ... }
```

---

## Tips for Success

1. **Start Small** - Begin with entities and relations
2. **Add Constraints Early** - Validate data from the start
3. **Use Descriptive Names** - Make your model self-documenting
4. **Leverage Patterns** - DRY principle applies
5. **Think Compositionally** - Build complex from simple
6. **Type Everything** - Let the type system help you

---

## Getting Help

- Check [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for syntax
- See [examples/](examples/) for practical code
- Read [SPECIFICATION.md](SPECIFICATION.md) for details
- Review [COMPARISON.md](COMPARISON.md) if coming from other languages

---

## What's Next?

You now know enough MODL to:
- Model your domain
- Validate data
- Transform entities
- Query information

As tools develop, you'll be able to:
- Generate code from MODL
- Visualize models
- Validate models
- Generate embeddings
- Integrate with LLMs

---

## Congratulations!

You've learned MODL! 🎉

Remember:
- **7 constructs** cover everything
- **Type-safe** by design
- **Compositional** semantics
- **AI-friendly** structure

Now go build something amazing with MODL!
