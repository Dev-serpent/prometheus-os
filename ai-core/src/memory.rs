use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryGraph {
    nodes: Arc<DashMap<Uuid, MemoryNode>>,
    edges: Arc<DashMap<(Uuid, Uuid), MemoryEdge>>,
    index: Arc<DashMap<String, Vec<Uuid>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
    pub importance: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    Concept,
    Entity,
    Event,
    File,
    Application,
    Command,
    Workflow,
    User,
    Message,
    Knowledge,
    Observation,
    Automation,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEdge {
    source: Uuid,
    target: Uuid,
    relation: RelationType,
    weight: f32,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RelationType {
    RelatedTo,
    PartOf,
    FollowedBy,
    Causes,
    UsedBy,
    CreatedBy,
    ReferencedBy,
    SimilarTo,
    OppositeOf,
    InstanceOf,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(DashMap::new()),
            edges: Arc::new(DashMap::new()),
            index: Arc::new(DashMap::new()),
        }
    }

    pub fn initialize(&self) {
        tracing::info!("Memory graph initialized");
    }

    pub fn add_node(&self, node_type: NodeType, content: String) -> Uuid {
        let id = Uuid::new_v4();
        let node = MemoryNode {
            id,
            node_type,
            content: content.clone(),
            embedding: None,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            importance: 0.5,
            metadata: HashMap::new(),
        };

        self.nodes.insert(id, node);

        // Index important terms
        for word in content.split_whitespace() {
            let key = word.to_lowercase();
            self.index.entry(key).or_default().push(id);
        }

        id
    }

    pub fn add_edge(&self, source: Uuid, target: Uuid, relation: RelationType) {
        let edge = MemoryEdge {
            source,
            target,
            relation,
            weight: 1.0,
            created_at: Utc::now(),
        };
        self.edges.insert((source, target), edge);
    }

    pub fn query(&self, query: &str) -> Vec<MemoryNode> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        // Direct index lookup
        if let Some(ids) = self.index.get(&query_lower) {
            for id in ids.iter() {
                if let Some(node) = self.nodes.get(id) {
                    results.push(node.clone());
                }
            }
        }

        // Semantic search would go here with embeddings
        results.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
        results.truncate(20);

        results
    }

    pub fn get_related(&self, id: Uuid) -> Vec<MemoryNode> {
        let mut related = Vec::new();

        for entry in self.edges.iter() {
            let ((source, target), _) = entry.pair();
            if *source == id {
                if let Some(node) = self.nodes.get(target) {
                    related.push(node.clone());
                }
            }
            if *target == id {
                if let Some(node) = self.nodes.get(source) {
                    related.push(node.clone());
                }
            }
        }

        related
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
