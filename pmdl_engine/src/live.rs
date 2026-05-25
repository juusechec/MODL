use crate::contracts::{PmdlDocument, UserFeedback};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphNode {
    pub id: String,
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: String,
    pub relation: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphView {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRevision {
    pub author_id: String,
    pub reason: String,
    pub document: PmdlDocument,
    pub graph: GraphView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveSession {
    current: ModelRevision,
    history: Vec<ModelRevision>,
    max_history: usize,
}

impl LiveSession {
    pub fn new(initial_document: PmdlDocument, author_id: impl Into<String>) -> Self {
        let graph = graph_from_pmdl(&initial_document.content);
        let current = ModelRevision {
            author_id: author_id.into(),
            reason: "session_start".to_string(),
            document: initial_document,
            graph,
        };

        Self {
            current,
            history: Vec::new(),
            max_history: 50,
        }
    }

    pub fn current(&self) -> &ModelRevision {
        &self.current
    }

    pub fn apply_ai_update(
        &mut self,
        document: PmdlDocument,
        reason: impl Into<String>,
        ai_actor: impl Into<String>,
    ) {
        self.push_revision(document, reason.into(), ai_actor.into());
    }

    pub fn apply_user_feedback(&mut self, feedback: UserFeedback, updated_document: PmdlDocument) {
        self.push_revision(updated_document, feedback.instruction, feedback.author_id);
    }

    fn push_revision(&mut self, document: PmdlDocument, reason: String, author_id: String) {
        let next_graph = graph_from_pmdl(&document.content);
        let prev = std::mem::replace(
            &mut self.current,
            ModelRevision {
                author_id,
                reason,
                document,
                graph: next_graph,
            },
        );
        self.history.push(prev);

        if self.history.len() > self.max_history {
            let overflow = self.history.len() - self.max_history;
            self.history.drain(0..overflow);
        }
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

pub fn graph_from_pmdl(content: &str) -> GraphView {
    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();

    for line in content.lines().map(|line| line.trim()).filter(|line| !line.is_empty()) {
        if let Some((left, right)) = line.split_once("--") {
            if let Some((relation, to_side)) = right.split_once("-->") {
                let from = left.trim().trim_start_matches('@').to_string();
                let relation = relation.trim().to_string();
                let to = to_side
                    .trim()
                    .trim_start_matches('@')
                    .split('[')
                    .next()
                    .unwrap_or_default()
                    .to_string();

                if !from.is_empty() {
                    ensure_node(&mut nodes, &from, None);
                }
                if !to.is_empty() {
                    ensure_node(&mut nodes, &to, None);
                }
                if !from.is_empty() && !to.is_empty() && !relation.is_empty() {
                    edges.push(GraphEdge { from, relation, to });
                }
                continue;
            }
        }

        if line.starts_with('@') {
            let token = line
                .trim_start_matches('@')
                .split(['[', ' '])
                .next()
                .unwrap_or_default()
                .to_string();
            let type_name = line
                .split('[')
                .nth(1)
                .and_then(|rest| rest.split(']').next())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            if !token.is_empty() {
                ensure_node(&mut nodes, &token, type_name);
            }
        }
    }

    GraphView { nodes, edges }
}

fn ensure_node(nodes: &mut Vec<GraphNode>, id: &str, type_name: Option<String>) {
    if let Some(node) = nodes.iter_mut().find(|node| node.id == id) {
        if node.type_name.is_none() {
            node.type_name = type_name;
        }
        return;
    }

    nodes.push(GraphNode {
        id: id.to_string(),
        type_name,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_text_and_graph_in_sync() {
        let base = PmdlDocument {
            schema_version: "pmdl.v1".to_string(),
            content: "@usuario[persona]\n@pedido[transaccion]\n@usuario --crea--> @pedido".to_string(),
        };

        let mut session = LiveSession::new(base, "user-1");
        assert_eq!(session.current().graph.nodes.len(), 2);
        assert_eq!(session.current().graph.edges.len(), 1);

        session.apply_ai_update(
            PmdlDocument {
                schema_version: "pmdl.v1".to_string(),
                content: "@usuario[persona]\n@pedido[transaccion]\n@factura[doc]\n@pedido --genera--> @factura"
                    .to_string(),
            },
            "validator_fix",
            "ai-agent",
        );

        assert_eq!(session.current().graph.nodes.len(), 3);
        assert_eq!(session.current().graph.edges.len(), 1);
        assert_eq!(session.history_len(), 1);
    }
}
