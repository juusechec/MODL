# MODL
**Model-Oriented Description Language** - An AI-Human model exchange language

## Overview

MODL is a formal language designed at the intersection of:
- **Formal Language Theory** (semantic compositionality)
- **Cognitive Science** (cognitive load optimization)
- **LLM Architecture** (embedding-friendly structures)
- **Type Theory** (static verification)

## Design Principles

### 1. Composicionalidad Semántica
Each construction has unambiguous, compositional meaning. The meaning of complex expressions is derived from the meaning of their components.

### 2. Economía Cognitiva
Exactly **7 core constructs** - optimal for human working memory (Miller's 7±2 rule):
1. **Entity** - Define objects with typed attributes
2. **Relation** - Express connections between entities
3. **Constraint** - Specify validation rules
4. **Transform** - Define pure transformations
5. **Pattern** - Create reusable structural templates
6. **Composition** - Build complex systems
7. **Query** - Declarative data retrieval

### 3. Isomorfismo con Embeddings
Every construct maps naturally to vector spaces:
- Entities → vectors in embedding space
- Relations → linear/non-linear transformations
- Compositions → vector operations
- Queries → similarity searches

### 4. Verificabilidad por Tipos
Strong static type system with:
- Type inference
- Compile-time verification
- Informative error messages

## Quick Start

See the [SPECIFICATION.md](SPECIFICATION.md) for complete language reference.

Explore [examples/](examples/) for practical demonstrations of each construct.

## Repository Structure

```
MODL/
├── SPECIFICATION.md      # Complete language specification
├── ARCHITECTURE.md       # Design rationale and theory
├── examples/            # Example MODL programs
│   ├── 01_entity.modl
│   ├── 02_relation.modl
│   ├── 03_constraint.modl
│   ├── 04_transform.modl
│   ├── 05_pattern.modl
│   ├── 06_composition.modl
│   └── 07_query.modl
└── README.md           # This file
```

## Key Features

- ✅ **7 core constructs** for cognitive efficiency
- ✅ **Type-safe** with static verification
- ✅ **Embedding-friendly** syntax for AI/ML integration
- ✅ **Compositional** semantics
- ✅ **Declarative** query language
- ✅ **Pattern-based** abstraction

## Theoretical Foundation

MODL combines insights from:
- Chomsky (1956) - Formal language structure
- Miller (1956) - Cognitive load limits (7±2)
- Vaswani et al. (2017) - Transformer architecture
- Martin-Löf (1975) - Type theory

## License

See [LICENSE](LICENSE) for details.
