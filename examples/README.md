# MODL Examples

This directory contains comprehensive examples demonstrating the 5 core constructs of MODL.

## Files

1. **01_entidades.modl** - Entity construction examples
2. **02_relaciones.modl** - Relation construction examples
3. **03_restricciones.modl** - Constraint construction examples
4. **04_comportamientos.modl** - Behavior construction examples
5. **05_composiciones.modl** - Composition/Process construction examples
6. **complete_example.modl** - Complete integrated system example

Each file demonstrates the syntax, semantics, and practical usage of its respective construct.

## Learning Path

If you're new to MODL, we recommend studying the examples in this order:

1. Start with **01_entidades.modl** to understand basic object definition
2. Move to **02_relaciones.modl** to see how entities connect
3. Study **03_restricciones.modl** to learn about business rules
4. Review **04_comportamientos.modl** for operation contracts
5. Explore **05_composiciones.modl** for complex processes
6. Finally, see **complete_example.modl** for a full system

## Example Domain

Most examples use an e-commerce domain with:
- Users (usuarios)
- Products (productos)
- Orders (pedidos)
- Payments (pagos)
- Transactions (transacciones)

This domain is familiar and demonstrates real-world modeling scenarios.

## Running Examples

Currently, MODL is a specification language. These examples are meant to be:
- Read and studied for learning
- Used as templates for your own models
- Validated against the grammar in SPECIFICATION.md

Future tooling will include:
- Parser and validator
- Type checker
- Constraint verifier
- Code generators

## Contributing Examples

If you create interesting MODL examples, please consider contributing them! Examples should:
- Be clear and well-commented
- Demonstrate specific features
- Use realistic domains
- Follow the style guide in SPECIFICATION.md
