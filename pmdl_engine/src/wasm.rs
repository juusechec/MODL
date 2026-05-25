use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use crate::live::graph_from_pmdl;

// ── request / response types ─────────────────────────────────────────────────

#[derive(Deserialize)]
struct WasmRequest {
    content: String,
    #[serde(default = "default_schema_version")]
    schema_version: String,
}

fn default_schema_version() -> String {
    "pmdl.v1".to_string()
}

#[derive(Serialize)]
struct WasmNode {
    id: String,
    type_name: Option<String>,
}

#[derive(Serialize)]
struct WasmEdge {
    from: String,
    relation: String,
    to: String,
}

#[derive(Serialize)]
struct WasmGraph {
    nodes: Vec<WasmNode>,
    edges: Vec<WasmEdge>,
}

#[derive(Serialize)]
struct WasmValidationError {
    code: String,
    message: String,
    line: Option<usize>,
    column: Option<usize>,
    hint: Option<String>,
}

#[derive(Serialize)]
struct WasmResponse {
    ok: bool,
    schema_version: String,
    content: String,
    graph: WasmGraph,
    errors: Vec<WasmValidationError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// ── public API ────────────────────────────────────────────────────────────────

/// Parse and validate a MODL document from JavaScript.
///
/// Accepts a JSON string: `{"content": "…", "schema_version": "pmdl.v1"}`
/// Returns a JSON string with the parsed graph and any validation errors.
#[wasm_bindgen]
pub fn validate_modl(request_json: &str) -> String {
    let response = match serde_json::from_str::<WasmRequest>(request_json) {
        Err(e) => WasmResponse {
            ok: false,
            schema_version: default_schema_version(),
            content: String::new(),
            graph: WasmGraph {
                nodes: vec![],
                edges: vec![],
            },
            errors: vec![],
            error: Some(format!("invalid request JSON: {e}")),
        },
        Ok(req) => {
            let graph = graph_from_pmdl(&req.content);
            let errors = validate_content(&req.content);
            let ok = errors.is_empty();
            WasmResponse {
                ok,
                schema_version: req.schema_version,
                content: req.content,
                graph: WasmGraph {
                    nodes: graph
                        .nodes
                        .into_iter()
                        .map(|n| WasmNode {
                            id: n.id,
                            type_name: n.type_name,
                        })
                        .collect(),
                    edges: graph
                        .edges
                        .into_iter()
                        .map(|e| WasmEdge {
                            from: e.from,
                            relation: e.relation,
                            to: e.to,
                        })
                        .collect(),
                },
                errors,
                error: None,
            }
        }
    };

    serde_json::to_string(&response)
        .unwrap_or_else(|_| r#"{"ok":false,"error":"serialization error"}"#.to_string())
}

// ── basic MODL syntax validation ──────────────────────────────────────────────

fn validate_content(content: &str) -> Vec<WasmValidationError> {
    let mut errors = Vec::new();

    for (idx, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if !is_valid_modl_line(line) {
            errors.push(WasmValidationError {
                code: "UNKNOWN_CONSTRUCT".to_string(),
                message: format!("Construcción MODL no reconocida: `{line}`"),
                line: Some(idx + 1),
                column: Some(1),
                hint: Some(
                    "Las líneas deben comenzar con @, cuando, siempre:, nunca:, proceso[, \
                     requiere:, ejecuta:, garantiza:, entrada:, salida:, pasos:, \
                     explicación:, caso_borde:, relacionado:, un número o un punto (.)"
                        .to_string(),
                ),
            });
        }
    }

    errors
}

fn is_valid_modl_line(line: &str) -> bool {
    const KEYWORD_PREFIXES: &[&str] = &[
        "@",
        "cuando ",
        "siempre:",
        "nunca:",
        "proceso[",
        "requiere:",
        "ejecuta:",
        "garantiza:",
        "entrada:",
        "salida:",
        "pasos:",
        "explicación:",
        "caso_borde:",
        "relacionado:",
        ".",
    ];

    KEYWORD_PREFIXES
        .iter()
        .any(|prefix| line.starts_with(prefix))
        || line
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
}
