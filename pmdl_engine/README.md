# PMDL Engine (Rust)

Base robusta en **Rust** para el motor PMDL/MODL.

## Qué incluye

- Pipeline por etapas: dictado/transcripción → prompt → generación PMDL → validación
- Contratos de datos normalizados entre etapas
- Ciclo de corrección iterativa con límite seguro de iteraciones
- Modo human-in-the-loop con historial de revisiones
- Fuente única de verdad textual (PMDL) y proyección de grafo en tiempo real
- Primitivas de gobernanza (autorización y redacción) y telemetría de etapas

## Estructura

- `src/contracts.rs`: contratos compartidos del pipeline
- `src/errors.rs`: errores normalizados del motor
- `src/pipeline.rs`: orquestación, feedback loop, telemetría y seguridad
- `src/live.rs`: sesión en vivo, revisiones y sincronización texto↔grafo
- `src/lib.rs`: exports públicos

## Ejecutar validación local

```bash
cargo fmt --all
cargo test
cargo check
```

## Próximos pasos recomendados

1. Implementar `Transcriber` real (STT en inglés).
2. Integrar proveedor LLM real en `PmdlGenerator`.
3. Implementar validador MODL/PMDL con reglas semánticas y sintácticas.
4. Exponer API (REST/WebSocket) para edición y feedback en vivo.
