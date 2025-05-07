//! Network nodes for the simulation
//! Provides different types of network nodes with specialized characteristics
//! to create a realistic network topology.

use std::collections::HashSet;

/// Types of network nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// Datacenter server (high bandwidth, low latency, stable)
    Datacenter,
    /// Edge server (medium bandwidth, low latency, fairly stable)
    EdgeServer,
    /// Mobile device (variable bandwidth, higher latency, unstable)
    MobileDevice,
    /// Client device (medium bandwidth, medium latency, mostly stable)
    ClientDevice,
}

/// Network simulation node
#[derive(Debug, Clone)]
pub struct SimulationNode {
    /// Node ID
    id: usize,
    /// Node name
    name: String,
    /// Node type
    node_type: NodeType,
    /// Connected node IDs
    connected_nodes: HashSet<usize>,
    /// Is this node mobile?
    is_mobile: bool,
    /// Location coordinates (x, y) - arbitrary units
    location: (f64, f64),
}

impl SimulationNode {
    /// Create a new simulation node
    pub fn new(id: usize, name: String, node_type: NodeType) -> Self {
        // Determine if node is mobile based on type
        let is_mobile = match node_type {
            NodeType::MobileDevice => true,
            _ => false,
        };
        
        // Assign random location
        let location = (
            rand::random::<f64>() * 1000.0,
            rand::random::<f64>() * 1000.0,
        );
        
        Self {
            id,
            name,
            node_type,
            connected_nodes: HashSet::new(),
            is_mobile,
            location,
        }
    }
    
    /// Get node ID
    pub fn id(&self) -> usize {
        self.id
    }
    
    /// Get node name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get node type
    pub fn node_type(&self) -> NodeType {
        self.node_type
    }
    
    /// Get node location
    pub fn location(&self) -> (f64, f64) {
        self.location
    }
    
    /// Set node location
    pub fn set_location(&mut self, x: f64, y: f64) {
        self.location = (x, y);
    }
    
    /// Is this node mobile?
    pub fn is_mobile(&self) -> bool {
        self.is_mobile
    }
    
    /// Add a connection to another node
    pub fn add_connection(&mut self, node_id: usize) {
        self.connected_nodes.insert(node_id);
    }
    
    /// Check if connected to another node
    pub fn is_connected_to(&self, node_id: usize) -> bool {
        self.connected_nodes.contains(&node_id)
    }
    
    /// Get all connected node IDs
    pub fn connected_nodes(&self) -> &HashSet<usize> {
        &self.connected_nodes
    }
    
    /// Update position for mobile nodes
    pub fn update_position(&mut self, delta_time: f64) {
        if !self.is_mobile {
            return;
        }
        
        // Simple random movement for mobile nodes
        let speed = 10.0; // units per second
        let distance = speed * delta_time;
        
        // Random direction
        let angle = rand::random::<f64>() * 2.0 * std::f64::consts::PI;
        let dx = distance * angle.cos();
        let dy = distance * angle.sin();
        
        // Update position, keeping within bounds (0-1000)
        self.location.0 = (self.location.0 + dx).max(0.0).min(1000.0);
        self.location.1 = (self.location.1 + dy).max(0.0).min(1000.0);
    }
    
    /// Calculate distance to another node
    pub fn distance_to(&self, other: &SimulationNode) -> f64 {
        let dx = self.location.0 - other.location.0;
        let dy = self.location.1 - other.location.1;
        (dx * dx + dy * dy).sqrt()
    }
}
