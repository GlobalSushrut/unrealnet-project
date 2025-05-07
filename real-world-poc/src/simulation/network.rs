//! Network simulation module for large-scale Dynamic Protocol demonstrations
//! Provides a comprehensive network topology simulation with multiple nodes
//! and connections to demonstrate protocol adaptation benefits.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

use unrealnet_core::dynphys::{
    DynamicProtocolEngine, PhysicsModel, GeneratedProtocol, NetworkCondition
};

use crate::simulation::metrics::{ScenarioMetrics, PerformanceMetrics};
use super::nodes::{SimulationNode, NodeType};
use super::scenarios::NetworkScenario;

/// Represents a connection between two nodes
#[derive(Debug, Clone)]
pub struct NodeConnection {
    /// Source node ID
    pub source_id: usize,
    /// Destination node ID
    pub dest_id: usize,
    /// Current latency in ms
    pub latency: f64,
    /// Current bandwidth in Kbps
    pub bandwidth: f64,
    /// Current packet loss rate (0.0-1.0)
    pub packet_loss: f64,
    /// Current jitter in ms
    pub jitter: f64,
    /// Whether this connection uses dynamic protocol adaptation
    pub uses_adaptation: bool,
    /// Currently active protocol on this connection
    pub active_protocol: Option<String>,
    /// Current network conditions
    pub current_conditions: Vec<NetworkCondition>,
}

/// Large-scale network simulation
pub struct NetworkSimulation {
    /// Network nodes
    pub nodes: HashMap<usize, SimulationNode>,
    /// Network connections
    pub connections: Vec<NodeConnection>,
    /// Protocol adaptation enabled
    pub adaptation_enabled: bool,
    /// Protocol engines for each connection
    pub protocol_engines: HashMap<(usize, usize), DynamicProtocolEngine>,
    /// Connection metrics over time
    pub connection_metrics: HashMap<(usize, usize), ConnectionMetrics>,
    /// Current simulation time in seconds
    pub current_time: u64,
    /// Current network scenario
    pub current_scenario: Option<NetworkScenario>,
    /// Network scenarios
    pub scenarios: HashMap<String, NetworkScenario>,
    /// Random number generator
    pub rng: rand::rngs::ThreadRng,
    /// Number of simulation iterations to run
    pub simulation_iterations: usize,
}

/// Metrics for a single connection
#[derive(Debug, Clone)]
pub struct ConnectionMetrics {
    /// Latency measurements over time (ms)
    pub latency_history: Vec<f64>,
    /// Bandwidth measurements over time (Kbps)
    pub bandwidth_history: Vec<f64>,
    /// Packet loss measurements over time (percentage)
    pub packet_loss_history: Vec<f64>,
    /// Jitter measurements over time (ms)
    pub jitter_history: Vec<f64>,
    /// Transfer time measurements over time (ms)
    pub transfer_time_history: Vec<f64>,
    /// Active protocol history
    pub protocol_history: Vec<Option<String>>,
    /// Timestamp of measurements
    pub timestamps: Vec<u64>,
    /// Source node ID
    pub source_id: usize,
    /// Destination node ID
    pub dest_id: usize,
    /// Latency
    pub latency: f64,
    /// Bandwidth
    pub bandwidth: f64,
    /// Packet loss
    pub packet_loss: f64,
    /// Jitter
    pub jitter: f64,
    /// Transfer time
    pub transfer_time: f64,
    /// Protocol
    pub protocol: Option<GeneratedProtocol>,
    /// Resilience score
    pub resilience_score: f64,
    /// Efficiency score
    pub efficiency_score: f64,
}

impl ConnectionMetrics {
    /// Create new connection metrics
    pub fn new() -> Self {
        Self {
            latency_history: Vec::new(),
            bandwidth_history: Vec::new(),
            packet_loss_history: Vec::new(),
            jitter_history: Vec::new(),
            transfer_time_history: Vec::new(),
            protocol_history: Vec::new(),
            timestamps: Vec::new(),
            source_id: 0,
            dest_id: 0,
            latency: 0.0,
            bandwidth: 0.0,
            packet_loss: 0.0,
            jitter: 0.0,
            transfer_time: 0.0,
            protocol: None,
            resilience_score: 0.0,
            efficiency_score: 0.0,
        }
    }
    
    /// Add a new measurement
    pub fn add_measurement(&mut self, 
        timestamp: u64, 
        latency: f64, 
        bandwidth: f64, 
        packet_loss: f64,
        jitter: f64,
        transfer_time: f64,
        protocol_id: Option<String>) {
        
        self.timestamps.push(timestamp);
        self.latency_history.push(latency);
        self.bandwidth_history.push(bandwidth);
        self.packet_loss_history.push(packet_loss);
        self.jitter_history.push(jitter);
        self.transfer_time_history.push(transfer_time);
        self.protocol_history.push(protocol_id);
    }
    
    /// Get average metrics
    pub fn averages(&self) -> (f64, f64, f64, f64, f64) {
        if self.latency_history.is_empty() {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }
        
        let avg_latency = self.latency_history.iter().sum::<f64>() / self.latency_history.len() as f64;
        let avg_bandwidth = self.bandwidth_history.iter().sum::<f64>() / self.bandwidth_history.len() as f64;
        let avg_packet_loss = self.packet_loss_history.iter().sum::<f64>() / self.packet_loss_history.len() as f64;
        let avg_jitter = self.jitter_history.iter().sum::<f64>() / self.jitter_history.len() as f64;
        let avg_transfer = self.transfer_time_history.iter().sum::<f64>() / self.transfer_time_history.len() as f64;
        
        (avg_latency, avg_bandwidth, avg_packet_loss, avg_jitter, avg_transfer)
    }
}

impl NetworkSimulation {
    /// Create a new network simulation
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            adaptation_enabled: true,
            protocol_engines: HashMap::new(),
            connection_metrics: HashMap::new(),
            current_scenario: None,
            scenarios: HashMap::new(),
            rng: thread_rng(),
            simulation_iterations: 100,
            current_time: 0,
        }
    }
    
    /// Initialize the network simulation
    pub fn initialize(&mut self, node_count: usize, connection_density: f64) -> Result<(), String> {
        // Create nodes
        self.create_nodes(node_count)?;
        
        // Create connections based on density
        self.create_connections(connection_density)?;
        
        // Initialize protocol engines for each connection
        self.initialize_protocol_engines()?;
        
        println!("Network simulation initialized with {} nodes and {} connections", 
            self.nodes.len(), self.connections.len());
            
        Ok(())
    }
    
    /// Create network nodes
    fn create_nodes(&mut self, node_count: usize) -> Result<(), String> {
        // Clear existing nodes
        self.nodes.clear();
        
        // Create nodes with different types
        let datacenter_count = node_count / 10;
        let edge_count = node_count / 5;
        let mobile_count = node_count / 3;
        let client_count = node_count - datacenter_count - edge_count - mobile_count;
        
        let mut node_id = 0;
        
        // Create datacenter nodes
        for _ in 0..datacenter_count {
            let node = SimulationNode::new(
                node_id,
                format!("datacenter_{}", node_id),
                NodeType::Datacenter
            );
            self.nodes.insert(node_id, node);
            node_id += 1;
        }
        
        // Create edge nodes
        for _ in 0..edge_count {
            let node = SimulationNode::new(
                node_id,
                format!("edge_{}", node_id),
                NodeType::EdgeServer
            );
            self.nodes.insert(node_id, node);
            node_id += 1;
        }
        
        // Create mobile nodes
        for _ in 0..mobile_count {
            let node = SimulationNode::new(
                node_id,
                format!("mobile_{}", node_id),
                NodeType::MobileDevice
            );
            self.nodes.insert(node_id, node);
            node_id += 1;
        }
        
        // Create client nodes
        for _ in 0..client_count {
            let node = SimulationNode::new(
                node_id,
                format!("client_{}", node_id),
                NodeType::ClientDevice
            );
            self.nodes.insert(node_id, node);
            node_id += 1;
        }
        
        Ok(())
    }
    
    /// Create network connections
    fn create_connections(&mut self, density: f64) -> Result<(), String> {
        // Clear existing connections
        self.connections.clear();
        
        let node_ids: Vec<usize> = self.nodes.keys().cloned().collect();
        let node_count = node_ids.len();
        
        // Calculate maximum possible connections (fully connected graph)
        let max_connections = (node_count * (node_count - 1)) / 2;
        
        // Calculate target number of connections based on density
        let target_connections = (max_connections as f64 * density) as usize;
        
        // Create connections
        let mut connections_created = 0;
        let mut connected_pairs = HashSet::new();
        
        while connections_created < target_connections {
            // Randomly select two nodes
            let idx1 = self.rng.gen_range(0..node_count);
            let mut idx2 = self.rng.gen_range(0..node_count);
            
            // Ensure nodes are different
            while idx2 == idx1 {
                idx2 = self.rng.gen_range(0..node_count);
            }
            
            let node1 = node_ids[idx1];
            let node2 = node_ids[idx2];
            
            // Ensure order for the HashSet
            let (source, dest) = if node1 < node2 { (node1, node2) } else { (node2, node1) };
            
            // Check if connection already exists
            if connected_pairs.contains(&(source, dest)) {
                continue;
            }
            
            // Add connection
            self.connections.push(NodeConnection {
                source_id: source,
                dest_id: dest,
                latency: 50.0 + self.rng.gen_range(0.0..50.0),
                bandwidth: 5000.0 + self.rng.gen_range(0.0..5000.0),
                packet_loss: self.rng.gen_range(0.0..0.05),
                jitter: self.rng.gen_range(0.0..10.0),
                uses_adaptation: false,
                active_protocol: None,
                current_conditions: Vec::new(),
            });
            
            // Mark as connected
            connected_pairs.insert((source, dest));
            connections_created += 1;
        }
        
        // Initialize metrics for each connection
        for conn in &self.connections {
            self.connection_metrics.insert(
                (conn.source_id, conn.dest_id), 
                ConnectionMetrics::new()
            );
        }
        
        Ok(())
    }
    
    /// Initialize protocol engines for each connection
    fn initialize_protocol_engines(&mut self) -> Result<(), String> {
        for conn in &self.connections {
            let mut engine = DynamicProtocolEngine::new();
            
            // Add physics models
            self.configure_physics_models(&mut engine);
            
            self.protocol_engines.insert(
                (conn.source_id, conn.dest_id),
                engine
            );
        }
        
        Ok(())
    }
    
    /// Configure physics models for the protocol engine
    fn configure_physics_models(&self, engine: &mut DynamicProtocolEngine) {
        // Low latency model
        let mut low_latency_model = PhysicsModel::new("low_latency_model", "Low Latency Optimization Model");
        low_latency_model.add_parameter("wave_propagation", 0.9)
            .add_parameter("entropy_scaling", 0.7)
            .add_parameter("quantum_resilience", 0.8)
            .add_parameter("phase_vector_precaching", 0.9)  // Reduce startup overhead
            .add_parameter("observer_sync_interval", 0.5)   // Reduce routing drift delays
            .add_condition_weight("latency", 1.5)
            .add_condition_weight("bandwidth", 0.3)
            .add_condition_weight("packet_loss", 0.8)
            .add_condition_weight("jitter", 1.5);
        engine.register_model(low_latency_model);
            
        // High bandwidth model
        let mut high_bandwidth_model = PhysicsModel::new("high_bandwidth_model", "High Bandwidth Optimization Model");
        high_bandwidth_model.add_parameter("wave_propagation", 0.7)
            .add_parameter("entropy_scaling", 1.3)
            .add_parameter("quantum_resilience", 0.6)
            .add_parameter("thermal_echo_boosting", 1.0)    // For ideal/extreme conditions
            .add_condition_weight("latency", 0.5)
            .add_condition_weight("bandwidth", 2.0)
            .add_condition_weight("packet_loss", 0.7)
            .add_condition_weight("jitter", 0.3);
        engine.register_model(high_bandwidth_model);
            
        // Reliability model
        let mut reliability_model = PhysicsModel::new("reliability_model", "Network Reliability Optimization Model");
        reliability_model.add_parameter("wave_propagation", 0.6)
            .add_parameter("entropy_scaling", 0.9)
            .add_parameter("quantum_resilience", 1.4)
            .add_parameter("directional_bias_correction", 1.0) // Fix high packet loss in asymmetry
            .add_condition_weight("latency", 0.6)
            .add_condition_weight("bandwidth", 0.4)
            .add_condition_weight("packet_loss", 2.0)
            .add_condition_weight("jitter", 1.0);
        engine.register_model(reliability_model);
        
        // Balanced model
        let mut balanced_model = PhysicsModel::new("balanced_model", "Balanced Network Optimization Model");
        balanced_model.add_parameter("wave_propagation", 0.8)
            .add_parameter("entropy_scaling", 1.0)
            .add_parameter("quantum_resilience", 1.0)
            .add_parameter("adaptive_silence", 0.5)         // Moderate adaptive silence
            .add_condition_weight("latency", 1.0)
            .add_condition_weight("bandwidth", 1.0)
            .add_condition_weight("packet_loss", 1.0)
            .add_condition_weight("jitter", 1.0);
        engine.register_model(balanced_model);
        
        // Mobile optimization model
        let mut mobile_model = PhysicsModel::new("mobile_model", "Mobile Network Optimization Model");
        mobile_model.add_parameter("wave_propagation", 0.75)
            .add_parameter("entropy_scaling", 0.8)
            .add_parameter("quantum_resilience", 1.2)
            .add_parameter("time_weighted_phase_stabilization", 1.0) // Add time-weighted packet phase stabilization
            .add_condition_weight("latency", 0.9)
            .add_condition_weight("bandwidth", 0.8)
            .add_condition_weight("packet_loss", 1.5)
            .add_condition_weight("jitter", 1.2);
        engine.register_model(mobile_model);
    }
    
    /// Apply a network scenario
    pub fn apply_scenario(&mut self, scenario: &NetworkScenario) {
        println!("Applying network scenario: {}", scenario.name);
        
        // Store the active scenario
        self.current_scenario = Some(scenario.clone());
        
        // Apply the scenario to all connections
        for conn in self.connections.iter_mut() {
            // Clear existing conditions
            conn.current_conditions.clear();
            
            // Get base latency and bandwidth from node types
            let (source_node, dest_node) = {
                let source = &self.nodes[&conn.source_id];
                let dest = &self.nodes[&conn.dest_id];
                (source, dest)
            };
            
            // Calculate base latency and bandwidth
            let (base_latency, base_bandwidth) = match (source_node.node_type(), dest_node.node_type()) {
                (NodeType::Datacenter, NodeType::Datacenter) => (10.0, 100000.0),
                (NodeType::Datacenter, NodeType::EdgeServer) | 
                (NodeType::EdgeServer, NodeType::Datacenter) => (20.0, 50000.0),
                (NodeType::Datacenter, NodeType::MobileDevice) | 
                (NodeType::MobileDevice, NodeType::Datacenter) => (50.0, 20000.0),
                (NodeType::EdgeServer, NodeType::MobileDevice) | 
                (NodeType::MobileDevice, NodeType::EdgeServer) => (30.0, 15000.0),
                (NodeType::MobileDevice, NodeType::MobileDevice) => (40.0, 10000.0),
                _ => (25.0, 25000.0),
            };
            
            // Apply scenario-specific modifications
            let (latency_mod, bandwidth_mod, packet_loss_mod, jitter_mod) = match scenario.name.as_str() {
                "asymmetric" => (1.5, 0.8, 1.2, 1.5),
                "mobile_handover" => (1.2, 0.9, 1.1, 1.3),
                "satellite" => (2.0, 0.5, 1.5, 2.0),
                _ => (1.0, 1.0, 1.0, 1.0),
            };
            
            // Apply random variations
            let latency_variation = self.rng.gen_range(-5.0..5.0);
            let bandwidth_variation = self.rng.gen_range(-200.0..200.0);
            let packet_loss_variation = self.rng.gen_range(-0.01..0.01);
            let jitter_variation = self.rng.gen_range(-1.0..1.0);
            
            // Update connection metrics with modifiers and variations
            let latency = base_latency * latency_mod + latency_variation;
            conn.latency = if latency < 1.0 { 1.0 } else { latency };
            
            let bandwidth = base_bandwidth * bandwidth_mod + bandwidth_variation;
            conn.bandwidth = if bandwidth < 100.0 { 100.0 } else { bandwidth };
            
            conn.packet_loss = (scenario.base_packet_loss * packet_loss_mod + packet_loss_variation).max(0.0).min(1.0);
            conn.jitter = (scenario.base_jitter * jitter_mod + jitter_variation).max(0.0);
            
            // Add scenario-specific conditions to guide protocol selection
            match scenario.name.as_str() {
                "asymmetric" => {
                    // Add asymmetric flag for asymmetric scenario
                    conn.current_conditions.push(NetworkCondition {
                        name: "asymmetric".to_string(),
                        value: 1.0,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    });
                },
                "mobile_handover" => {
                    // Add handover flag for mobile scenario
                    conn.current_conditions.push(NetworkCondition {
                        name: "handover".to_string(),
                        value: 1.0,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    });
                },
                "satellite" => {
                    // Add high latency flag for satellite
                    conn.current_conditions.push(NetworkCondition {
                        name: "high_latency".to_string(),
                        value: 1.0,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    });
                },
                _ => {}
            }
        }
    }
    
    /// Enable or disable protocol adaptation
    pub fn set_adaptation_enabled(&mut self, enabled: bool) {
        self.adaptation_enabled = enabled;
        
        // Update connection adaptation flags
        for conn in &mut self.connections {
            conn.uses_adaptation = enabled;
        }
        
        println!("Protocol adaptation {}", if enabled { "enabled" } else { "disabled" });
    }
    
    /// Run the simulation for the specified duration
    pub fn run(&mut self, duration: Duration) -> Result<(), String> {
        let start_time = Instant::now();
        let mut last_update = Instant::now();
        let update_interval = Duration::from_millis(100); // Update every 100ms
        
        println!("Running simulation for {:?}...", duration);
        
        while start_time.elapsed() < duration {
            // If it's time for an update
            if last_update.elapsed() >= update_interval {
                // Update network conditions based on scenario
                self.update_network_conditions();
                
                // Update protocols if adaptation is enabled
                if self.adaptation_enabled {
                    self.update_protocols();
                }
                
                // Collect metrics
                self.collect_metrics();
                
                last_update = Instant::now();
            }
        }
        
        Ok(())
    }
    
    /// Update network conditions based on current scenario and simulation time
    fn update_network_conditions(&mut self) {
        // If no scenario is active, do nothing
        if self.current_scenario.is_none() {
            return;
        }
        
        let scenario = self.current_scenario.as_ref().unwrap().clone();
        
        // Apply dynamic effects to each connection
        for conn in &mut self.connections {
            // Get node types
            let source_type = match self.nodes.get(&conn.source_id) {
                Some(node) => node.node_type(),
                None => NodeType::ClientDevice,
            };
            
            let dest_type = match self.nodes.get(&conn.dest_id) {
                Some(node) => node.node_type(),
                None => NodeType::ClientDevice,
            };
            
            // Calculate type-specific modifications
            let (latency_mod, bandwidth_mod, packet_loss_mod, jitter_mod) = match (source_type, dest_type) {
                // Datacenter to datacenter: excellent connection
                (NodeType::Datacenter, NodeType::Datacenter) => (0.5, 2.0, 0.2, 0.5),
                
                // Datacenter to edge: good connection
                (NodeType::Datacenter, NodeType::EdgeServer) | 
                (NodeType::EdgeServer, NodeType::Datacenter) => (0.7, 1.5, 0.3, 0.7),
                
                // Datacenter to mobile/client: depends on scenario
                (NodeType::Datacenter, NodeType::MobileDevice) | 
                (NodeType::MobileDevice, NodeType::Datacenter) => {
                    match scenario.name.as_str() {
                        "congestion" => (1.5, 0.6, 1.3, 1.4),
                        "wireless_interference" => (1.3, 0.7, 1.5, 1.6),
                        _ => (1.0, 0.8, 1.1, 1.2),
                    }
                },
                
                // Edge to mobile: varies by scenario
                (NodeType::EdgeServer, NodeType::MobileDevice) | 
                (NodeType::MobileDevice, NodeType::EdgeServer) => {
                    match scenario.name.as_str() {
                        "wireless_interference" => (1.4, 0.6, 1.6, 1.8),
                        "mobile_handover" => (1.6, 0.5, 1.7, 1.9),
                        _ => (1.1, 0.7, 1.2, 1.3),
                    }
                },
                
                // Mobile to mobile: challenging
                (NodeType::MobileDevice, NodeType::MobileDevice) => {
                    match scenario.name.as_str() {
                        "wireless_interference" => (1.7, 0.4, 1.8, 2.0),
                        "mobile_handover" => (1.8, 0.3, 1.9, 2.2),
                        _ => (1.4, 0.5, 1.5, 1.7),
                    }
                },
                
                // Default case
                _ => (1.0, 1.0, 1.0, 1.0),
            };
            
            // Apply random variations
            let latency_variation = self.rng.gen_range(-5.0..5.0);
            let bandwidth_variation = self.rng.gen_range(-200.0..200.0);
            let packet_loss_variation = self.rng.gen_range(-0.01..0.01);
            let jitter_variation = self.rng.gen_range(-1.0..1.0);
            
            // Update connection metrics with modifiers and variations
            let latency = scenario.base_latency * latency_mod + latency_variation;
            conn.latency = if latency < 1.0 { 1.0 } else { latency };
            
            let bandwidth = scenario.base_bandwidth * bandwidth_mod + bandwidth_variation;
            conn.bandwidth = if bandwidth < 100.0 { 100.0 } else { bandwidth };
            
            conn.packet_loss = (scenario.base_packet_loss * packet_loss_mod + packet_loss_variation).max(0.0).min(1.0);
            conn.jitter = (scenario.base_jitter * jitter_mod + jitter_variation).max(0.0);
        }
    }
    
    /// Update protocols based on current network conditions
    fn update_protocols(&mut self) {
        // Collect all conditions first to avoid borrow checker issues
        let mut connection_data = Vec::new();
        
        for conn in self.connections.iter() {
            if !conn.uses_adaptation {
                continue;
            }
            
            // Build normalized network conditions for this connection
            let norm_latency = Self::normalize_latency_static(conn.latency);
            let norm_bandwidth = Self::normalize_bandwidth_static(conn.bandwidth);
            let norm_packet_loss = Self::normalize_packet_loss_static(conn.packet_loss);
            let norm_jitter = Self::normalize_jitter_static(conn.jitter);
            
            let conditions = vec![
                NetworkCondition {
                    name: "latency".to_string(),
                    value: norm_latency,
                    timestamp: 0, // Not important for our simulation
                },
                NetworkCondition {
                    name: "bandwidth".to_string(),
                    value: norm_bandwidth,
                    timestamp: 0,
                },
                NetworkCondition {
                    name: "packet_loss".to_string(),
                    value: norm_packet_loss,
                    timestamp: 0,
                },
                NetworkCondition {
                    name: "jitter".to_string(),
                    value: norm_jitter,
                    timestamp: 0,
                },
            ];
            
            connection_data.push((conn.source_id, conn.dest_id, conditions, conn.active_protocol.is_some()));
        }
        
        // Now update protocols without holding mutable references
        for (source_id, dest_id, conditions, _has_protocol) in connection_data {
            // Get engine for this connection
            if let Some(engine) = self.protocol_engines.get_mut(&(source_id, dest_id)) {
                // Update engine with conditions
                for condition in &conditions {
                    // DynamicProtocolEngine doesn't have add_condition, so we'll
                    // use the engine's update_conditions method after setting our conditions
                    engine.update_conditions();
                }
                
                // Generate new protocol
                if let Some(protocol) = engine.generate_protocol() {
                    // Find the connection and update it
                    for conn in &mut self.connections {
                        if conn.source_id == source_id && conn.dest_id == dest_id {
                            // Only update if protocol changed or none active
                            let should_update = match &conn.active_protocol {
                                Some(active) => active != &protocol.name,
                                None => true,
                            };
                            
                            if should_update {
                                conn.active_protocol = Some(protocol.name.clone());
                            }
                            
                            break;
                        }
                    }
                }
            }
        }
    }
    
    /// Collect metrics for each connection
    fn collect_metrics(&mut self) {
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
            
        for conn in &self.connections {
            // Calculate transfer time based on conditions and protocol
            let transfer_time = self.calculate_transfer_time(conn);
            
            // Get or create metrics for this connection
            if let Some(metrics) = self.connection_metrics.get_mut(&(conn.source_id, conn.dest_id)) {
                metrics.add_measurement(
                    now,
                    conn.latency,
                    conn.bandwidth,
                    conn.packet_loss * 100.0, // Convert to percentage
                    conn.jitter,
                    transfer_time,
                    conn.active_protocol.as_ref().map(|p| p.clone()),
                );
            }
        }
    }
    
    /// Calculate transfer time for a connection
    fn calculate_transfer_time(&self, conn: &NodeConnection) -> f64 {
        // Base file size: 10MB = 10 * 1024 * 8 Kb
        let file_size_kb = 10.0 * 1024.0 * 8.0;
        
        // Calculate base transfer time in ms
        let base_time = (file_size_kb / conn.bandwidth) * 1000.0; // Time to transfer 1MB in ms
        
        // Adjust for packet loss (each 1% increases time by ~2%)
        let loss_factor = 1.0 + (conn.packet_loss * 2.0);
        
        // Adjust for protocol optimization
        let protocol_factor = if conn.active_protocol.is_some() {
            // Different protocols have different optimization levels
            match conn.active_protocol.as_ref().unwrap().as_str() {
                "Low Latency Optimization Model" => 0.65,
                "High Bandwidth Optimization Model" => 0.7,
                "Network Reliability Optimization Model" => 0.75,
                "Balanced Network Optimization Model" => 0.8,
                "Mobile Network Optimization Model" => 0.75,
                _ => 0.85,
            }
        } else {
            1.0 // No optimization
        };
        
        // Calculate final transfer time
        base_time * loss_factor * protocol_factor
    }
    
    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
    
    /// Get all connections
    pub fn get_connections(&self) -> &[NodeConnection] {
        &self.connections
    }
    
    /// Get all nodes
    pub fn get_nodes(&self) -> &HashMap<usize, SimulationNode> {
        &self.nodes
    }
    
    /// Get metrics for all connections
    pub fn get_metrics(&self) -> &HashMap<(usize, usize), ConnectionMetrics> {
        &self.connection_metrics
    }
    
    /// Get current scenario
    pub fn get_current_scenario(&self) -> Option<&NetworkScenario> {
        self.current_scenario.as_ref()
    }
    
    /// Static version of normalize_latency to avoid borrow checker issues
    fn normalize_latency_static(latency_ms: f64) -> f64 {
        // Clamp to reasonable range
        let clamped = latency_ms.min(500.0).max(1.0);
        // Reverse and normalize (1ms → 1.0, 500ms → 0.0)
        (500.0 - clamped) / 499.0
    }
    
    /// Static version of normalize_bandwidth to avoid borrow checker issues
    fn normalize_bandwidth_static(bandwidth_kbps: f64) -> f64 {
        let log_bw = bandwidth_kbps.max(1.0).ln();
        let min_log = 1.0f64.ln(); // ln(1)
        let max_log = 100_000.0f64.ln(); // ln(100,000)
        
        // Normalize on logarithmic scale (1 Kbps → 0.0, 100,000 Kbps → 1.0)
        (log_bw - min_log) / (max_log - min_log)
    }
    
    /// Static version of normalize_packet_loss to avoid borrow checker issues
    fn normalize_packet_loss_static(packet_loss_rate: f64) -> f64 {
        // Clamp to valid range
        let clamped = packet_loss_rate.min(1.0).max(0.0);
        // Reverse and normalize (0.0 → 1.0, 1.0 → 0.0)
        1.0 - clamped
    }
    
    /// Static version of normalize_jitter to avoid borrow checker issues
    fn normalize_jitter_static(jitter_ms: f64) -> f64 {
        // Clamp to reasonable range
        let clamped = jitter_ms.min(100.0).max(0.0);
        // Reverse and normalize (0ms → 1.0, 100ms → 0.0)
        (100.0 - clamped) / 100.0
    }
    
    /// Creates an optimized protocol engine based on the current network conditions
    fn create_protocol_engine(&self, conditions: Vec<NetworkCondition>) -> DynamicProtocolEngine {
        let mut engine = DynamicProtocolEngine::new();
        
        // Create physics models for different network scenarios
        let mut low_latency_model = PhysicsModel::new("low_latency", "Low Latency Optimization");
        let mut high_bandwidth_model = PhysicsModel::new("high_bandwidth", "High Bandwidth Optimization");
        let mut reliability_model = PhysicsModel::new("reliability", "Network Reliability Optimization");
        let mut mobile_model = PhysicsModel::new("mobile", "Mobile Network Optimization");
        let mut satellite_model = PhysicsModel::new("satellite", "Satellite Network Optimization");
        let mut asymmetric_model = PhysicsModel::new("asymmetric", "Asymmetric Network Optimization");
        
        // Add condition weights to each model
        low_latency_model.add_condition_weight("latency", 1.0);
        low_latency_model.add_condition_weight("jitter", 0.8);
        low_latency_model.add_condition_weight("packet_loss", 0.5);
        
        high_bandwidth_model.add_condition_weight("bandwidth", 1.0);
        high_bandwidth_model.add_condition_weight("latency", 0.3);
        
        reliability_model.add_condition_weight("packet_loss", 1.0);
        reliability_model.add_condition_weight("jitter", 0.7);
        reliability_model.add_condition_weight("latency", 0.4);
        
        mobile_model.add_condition_weight("latency", 0.6);
        mobile_model.add_condition_weight("jitter", 1.0);
        mobile_model.add_condition_weight("bandwidth", 0.5);
        mobile_model.add_condition_weight("packet_loss", 0.7);
        
        satellite_model.add_condition_weight("latency", 1.0);
        satellite_model.add_condition_weight("jitter", 0.5);
        satellite_model.add_condition_weight("packet_loss", 0.8);
        
        // Special optimization for asymmetric networks
        asymmetric_model.add_condition_weight("latency", 0.7);
        asymmetric_model.add_condition_weight("bandwidth", 0.6);
        asymmetric_model.add_condition_weight("packet_loss", 1.0);
        asymmetric_model.add_condition_weight("jitter", 0.8);
        
        // Add advanced optimization parameters
        
        // Low latency optimizations
        low_latency_model.add_parameter("phase_vector_precaching", 0.9);
        low_latency_model.add_parameter("observer_synchronization", 0.8);
        low_latency_model.add_parameter("echo_optimization", 0.7);
        low_latency_model.add_parameter("routing_optimization", 0.85);
        
        // High bandwidth optimizations
        high_bandwidth_model.add_parameter("parallel_transfer", 0.9);
        high_bandwidth_model.add_parameter("compression_level", 0.4);
        high_bandwidth_model.add_parameter("chunk_optimization", 0.8);
        high_bandwidth_model.add_parameter("buffer_size", 0.95);
        
        // Reliability optimizations
        reliability_model.add_parameter("redundancy_level", 0.8);
        reliability_model.add_parameter("error_correction", 0.9);
        reliability_model.add_parameter("packet_verification", 0.7);
        reliability_model.add_parameter("retry_optimization", 0.6);
        
        // Mobile optimizations
        mobile_model.add_parameter("time_weighted_phase_stabilization", 1.0);
        mobile_model.add_parameter("handover_optimization", 0.9);
        mobile_model.add_parameter("variable_route_selection", 0.7);
        mobile_model.add_parameter("power_conservation", 0.5);
        
        // Satellite optimizations
        satellite_model.add_parameter("adaptive_silence", 1.0);
        satellite_model.add_parameter("predictive_routing", 0.9);
        satellite_model.add_parameter("path_diversity", 0.7);
        satellite_model.add_parameter("temporal_compression", 0.6);
        
        // Asymmetric optimizations - fixed values for better performance
        asymmetric_model.add_parameter("directional_bias_correction", 1.0);
        asymmetric_model.add_parameter("asymmetric_buffer_sizing", 0.9);
        asymmetric_model.add_parameter("downlink_optimization", 0.8);
        asymmetric_model.add_parameter("uplink_optimization", 0.7);
        asymmetric_model.add_parameter("adaptive_queue_management", 0.85);
        asymmetric_model.add_parameter("thermal_echo_boosting", 0.95);
        
        // Register all models with the engine
        engine.register_model(low_latency_model);
        engine.register_model(high_bandwidth_model);
        engine.register_model(reliability_model);
        engine.register_model(mobile_model);
        engine.register_model(satellite_model);
        engine.register_model(asymmetric_model);
        
        // Add current network conditions to engine
        for condition in conditions {
            // Create a simple model for this condition
            let mut condition_model = PhysicsModel::new(&format!("condition_{}", condition.name), &condition.name);
            condition_model.add_condition_weight(&condition.name, 1.0);
            engine.register_model(condition_model);
        }
        
        engine
    }
    
    /// Apply the generated protocol optimizations to connections
    fn apply_protocol_optimizations(&mut self, protocol: &GeneratedProtocol) {
        let protocol_type = &protocol.name;
        
        // Loop through each connection and apply appropriate optimizations
        for conn in self.connections.iter_mut() {
            if !conn.uses_adaptation {
                continue;
            }
            
            // Get base metrics before modification
            let base_latency = conn.latency;
            let base_bandwidth = conn.bandwidth;
            let base_packet_loss = conn.packet_loss;
            let base_jitter = conn.jitter;
            
            // Get optimization parameters from the protocol
            let latency_opt = protocol.parameters.get("latency_optimization").unwrap_or(&0.0);
            let bandwidth_opt = protocol.parameters.get("bandwidth_optimization").unwrap_or(&0.0);
            let packet_loss_opt = protocol.parameters.get("packet_loss_optimization").unwrap_or(&0.0);
            let jitter_opt = protocol.parameters.get("jitter_optimization").unwrap_or(&0.0);
            
            // Check for directional bias correction parameter
            let directional_bias = protocol.parameters.get("directional_bias_correction").unwrap_or(&0.0);
            let asymmetric_buffer = protocol.parameters.get("asymmetric_buffer_sizing").unwrap_or(&0.0);
            let thermal_echo = protocol.parameters.get("thermal_echo_boosting").unwrap_or(&0.0);
            
            // Calculate improvement factors based on protocol parameters and connection properties
            // Enhanced formula for asymmetric network scenarios
            let is_asymmetric = base_latency > 200.0 && base_packet_loss > 0.05;
            
            // Different improvement calculations based on network type
            let (latency_improve, bandwidth_improve, packet_loss_improve, jitter_improve) = 
                if protocol_type.contains("asymmetric") {
                    // Asymmetric network optimizations
                    let lat_imp = 0.3 * latency_opt * directional_bias;
                    let bw_imp = 0.25 * bandwidth_opt * asymmetric_buffer;
                    let pl_imp = 0.4 * packet_loss_opt * directional_bias;
                    let jit_imp = 0.3 * jitter_opt * thermal_echo;
                    (lat_imp, bw_imp, pl_imp, jit_imp)
                } else if protocol_type.contains("satellite") {
                    // Satellite optimizations
                    let lat_imp = 0.2 * latency_opt;
                    let bw_imp = 0.15 * bandwidth_opt;
                    let pl_imp = 0.35 * packet_loss_opt;
                    let jit_imp = 0.1 * jitter_opt * thermal_echo;
                    (lat_imp, bw_imp, pl_imp, jit_imp)
                } else if protocol_type.contains("mobile") {
                    // Mobile network optimizations
                    let lat_imp = 0.25 * latency_opt;
                    let bw_imp = 0.1 * bandwidth_opt;
                    let pl_imp = 0.2 * packet_loss_opt;
                    let jit_imp = 0.4 * jitter_opt;
                    (lat_imp, bw_imp, pl_imp, jit_imp)
                } else if base_latency < 5.0 && base_bandwidth > 9000.0 {
                    // Ideal/fast network - small improvements
                    let lat_imp = 0.01 * latency_opt;
                    let bw_imp = 0.01 * bandwidth_opt;
                    let pl_imp = 0.01 * packet_loss_opt;
                    let jit_imp = 0.01 * jitter_opt;
                    (lat_imp, bw_imp, pl_imp, jit_imp)
                } else {
                    // Default optimizations
                    let lat_imp = 0.15 * latency_opt;
                    let bw_imp = 0.15 * bandwidth_opt;
                    let pl_imp = 0.15 * packet_loss_opt;
                    let jit_imp = 0.15 * jitter_opt;
                    (lat_imp, bw_imp, pl_imp, jit_imp)
                };
            
            // Apply improvements
            conn.latency = base_latency * (1.0 - latency_improve).max(0.6).min(1.0);
            conn.bandwidth = base_bandwidth * (1.0 + bandwidth_improve).max(1.0).min(1.5);
            conn.packet_loss = base_packet_loss * (1.0 - packet_loss_improve).max(0.5).min(1.0);
            conn.jitter = base_jitter * (1.0 - jitter_improve).max(0.7).min(1.0);
            
            // Record that this connection uses an optimized protocol
            conn.active_protocol = Some(protocol_type.to_string());
        }
    }
    
    /// Apply protocol performance impact to network conditions
    fn apply_protocol_impact(&self, metrics: &mut ConnectionMetrics, protocol_name: &str) {
        // Apply the protocol optimizations to the metrics
        match protocol_name {
            "low_latency" => {
                // Reduce latency but slightly reduce bandwidth due to overhead
                metrics.latency *= 0.8;
                metrics.bandwidth *= 0.95;
                metrics.packet_loss *= 0.9;
                metrics.jitter *= 0.7;
            },
            "high_bandwidth" => {
                // Increase bandwidth but slightly increase latency
                metrics.bandwidth *= 1.2;
                metrics.latency *= 1.05;
                metrics.packet_loss *= 0.85;
                metrics.jitter *= 0.95;
            },
            "reliability" => {
                // Reduce packet loss but increase latency and reduce bandwidth
                metrics.packet_loss *= 0.6;
                metrics.latency *= 1.1;
                metrics.bandwidth *= 0.9;
                metrics.jitter *= 0.85;
            },
            "mobile" => {
                // Reduce jitter but increase latency and packet loss
                metrics.jitter *= 0.5;
                metrics.latency *= 1.15;
                metrics.packet_loss *= 1.1;
                metrics.bandwidth *= 0.95;
            },
            "satellite" => {
                // Optimize for high latency connections
                metrics.packet_loss *= 0.7;
                metrics.jitter *= 0.8;
            },
            "asymmetric" => {
                // Handle asymmetric network conditions
                metrics.bandwidth *= 1.15;
                metrics.packet_loss *= 0.7;
            },
            _ => {
                // Default improvement for custom protocols
                metrics.latency *= 0.9;
                metrics.bandwidth *= 1.05;
                metrics.packet_loss *= 0.85;
                metrics.jitter *= 0.9;
            }
        }
        
        // Recalculate transfer time
        metrics.transfer_time = metrics.latency * (1.0 + metrics.packet_loss * 10.0) / (metrics.bandwidth / 1000.0);
    }
    
    /// Gather protocol statistics across connections
    pub fn protocol_distribution(&self) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();
        
        for conn in &self.connections {
            if let Some(protocol) = &conn.active_protocol {
                *distribution.entry(protocol.clone()).or_insert(0) += 1;
            }
        }
        
        distribution
    }
    
    /// Get the protocol name for a connection
    fn get_protocol_name(&self, conn_idx: usize) -> Option<String> {
        if conn_idx < self.connections.len() {
            self.connections[conn_idx].active_protocol.clone()
        } else {
            None
        }
    }
    
    /// Check for active protocol on a connection
    fn has_active_protocol(&self, conn: &NodeConnection) -> bool {
        conn.active_protocol.is_some()
    }
    
    /// Calculate transfer time based on network conditions
    fn calculate_transfer_time_from_metrics(&self, latency: f64, bandwidth: f64, packet_loss: f64, jitter: f64) -> f64 {
        // Base transfer time calculation (simplified model)
        let base_transfer = 1000.0 * 1000.0 / bandwidth; // Time to transfer 1MB in ms
        
        // Apply latency impact
        let latency_factor = 1.0 + (latency / 100.0);
        
        // Apply packet loss impact (retransmissions)
        let packet_loss_factor = 1.0 + (packet_loss * 10.0);
        
        // Apply jitter impact
        let jitter_factor = 1.0 + (jitter / 100.0);
        
        // Calculate final transfer time
        base_transfer * latency_factor * packet_loss_factor * jitter_factor
    }
    
    /// Calculate resilience score based on network metrics
    pub fn calculate_resilience_score(&self, latency: f64, bandwidth: f64, packet_loss: f64, jitter: f64) -> f64 {
        // Normalize all metrics to 0-1 range
        let norm_latency = (1000.0 - latency.min(1000.0)) / 1000.0;
        let norm_bandwidth = bandwidth.min(10000.0) / 10000.0;
        let norm_packet_loss = 1.0 - packet_loss;
        let norm_jitter = (100.0 - jitter.min(100.0)) / 100.0;
        
        // Calculate resilience as a weighted average
        let resilience = norm_latency * 0.3 + 
                         norm_bandwidth * 0.2 + 
                         norm_packet_loss * 0.3 + 
                         norm_jitter * 0.2;
                         
        // Scale to 0-100 range
        resilience * 100.0
    }
    
    /// Calculate efficiency score based on network metrics
    pub fn calculate_efficiency_score(&self, packet_count: f64, dropped_packets: f64, bandwidth: f64) -> f64 {
        // Calculate packet delivery ratio (avoid division by zero)
        let delivery_ratio = if packet_count > 0.0 {
            (packet_count - dropped_packets) / packet_count
        } else {
            1.0 // Perfect delivery if no packets
        };
        
        // Calculate bandwidth utilization (normalized to 0-1)
        let bandwidth_utilization = bandwidth / 10000.0;
        
        // Calculate efficiency as a weighted average
        let efficiency = delivery_ratio * 0.6 + bandwidth_utilization * 0.4;
        
        // Scale to 0-100 range
        efficiency * 100.0
    }
    
    /// Measure network performance with the current configuration
    fn measure_performance(&mut self, scenario_name: &str, adaptation_enabled: bool) -> ScenarioMetrics {
        let mut metrics = ScenarioMetrics::new(scenario_name.to_string());
        
        // First, create a copy of connections for baseline measurement
        let original_connections = self.connections.clone();
        
        // Turn off adaptation for baseline measurement
        self.adaptation_enabled = false;
        
        // Reset all connections to remove any active protocol
        for conn in &mut self.connections {
            conn.active_protocol = None;
        }
        
        // Variables for metrics calculation
        let mut total_latency = 0.0;
        let mut total_bandwidth = 0.0;
        let mut total_packet_loss = 0.0;
        let mut total_jitter = 0.0;
        let mut total_transfer_time = 0.0;
        let mut resilience_score = 0.0;
        let mut efficiency_score = 0.0;
        let connection_count = self.connections.len();
        
        // Collect metrics from all connections
        for conn in &self.connections {
            // Add base metrics
            total_latency += conn.latency;
            total_bandwidth += conn.bandwidth;
            total_packet_loss += conn.packet_loss;
            total_jitter += conn.jitter;
            
            // Calculate file transfer time
            let transfer_time = self.calculate_transfer_time(conn);
            total_transfer_time += transfer_time;
            
            // Calculate resilience based on connection properties
            let conn_resilience = self.calculate_resilience_score(conn.latency, conn.bandwidth, conn.packet_loss, conn.jitter);
            resilience_score += conn_resilience;
            
            // Calculate efficiency based on bandwidth utilization and overhead
            let conn_efficiency = self.calculate_efficiency_score(
                (conn.bandwidth / 1000.0).max(1.0),  // packet count estimation
                conn.packet_loss * (conn.bandwidth / 1000.0).max(1.0),  // dropped packet estimation
                conn.bandwidth
            );
            efficiency_score += conn_efficiency;
        }
        
        // Calculate averages
        if connection_count > 0 {
            metrics.avg_latency = total_latency / connection_count as f64;
            metrics.avg_bandwidth = total_bandwidth / connection_count as f64;
            metrics.avg_packet_loss = total_packet_loss / connection_count as f64;
            metrics.avg_jitter = total_jitter / connection_count as f64;
            metrics.avg_transfer_time = total_transfer_time / connection_count as f64;
            metrics.resilience_score = resilience_score / connection_count as f64;
            metrics.efficiency_score = efficiency_score / connection_count as f64;
        }
        
        metrics
    }
    
    /// Measure performance for a scenario
    fn measure_scenario_performance(&mut self, scenario_name: &str) -> PerformanceMetrics {
        let mut metrics = PerformanceMetrics::new();
        
        // Apply the scenario
        if let Some(scenario) = self.scenarios.get(scenario_name).cloned() {
            self.apply_scenario(&scenario);
            
            // Run the simulation with adaptation disabled
            self.set_adaptation_enabled(false);
            self.run(Duration::from_secs(10)).unwrap_or_else(|e| {
                eprintln!("Error running baseline simulation: {}", e);
            });
            
            // Collect baseline metrics
            metrics.baseline = self.measure_performance(scenario_name, false);
            
            // Run the simulation with adaptation enabled
            self.set_adaptation_enabled(true);
            self.run(Duration::from_secs(10)).unwrap_or_else(|e| {
                eprintln!("Error running adaptation simulation: {}", e);
            });
            
            // Collect adaptation metrics
            metrics.adaptation = self.measure_performance(scenario_name, true);
            
            // Set adaptation improvement metrics
            metrics.improvement = self.calculate_performance_improvement(
                &metrics.baseline,
                &metrics.adaptation
            );
        }
        
        metrics
    }
    
    /// Calculate performance improvement between baseline and adapted metrics
    fn calculate_performance_improvement(
        &self,
        baseline: &crate::simulation::metrics::ScenarioMetrics,
        adapted: &crate::simulation::metrics::ScenarioMetrics
    ) -> crate::simulation::metrics::PerformanceImprovement {
        use crate::simulation::metrics::PerformanceImprovement;
        
        // Create empty improvement metrics
        let mut improvement = PerformanceImprovement::default();
        
        // Latency (lower is better)
        if baseline.avg_latency > 0.0 {
            improvement.latency = ((baseline.avg_latency - adapted.avg_latency) / baseline.avg_latency) * 100.0;
        }
        
        // Bandwidth (higher is better)
        if baseline.avg_bandwidth > 0.0 {
            improvement.bandwidth = ((adapted.avg_bandwidth - baseline.avg_bandwidth) / baseline.avg_bandwidth) * 100.0;
        }
        
        // Packet loss (lower is better)
        if baseline.avg_packet_loss > 0.0 {
            improvement.packet_loss = ((baseline.avg_packet_loss - adapted.avg_packet_loss) / baseline.avg_packet_loss) * 100.0;
        }
        
        // Transfer time (lower is better)
        if baseline.avg_transfer_time > 0.0 {
            improvement.transfer_time = ((baseline.avg_transfer_time - adapted.avg_transfer_time) / baseline.avg_transfer_time) * 100.0;
        }
        
        // Resilience (higher is better)
        if baseline.resilience_score > 0.0 {
            improvement.resilience = ((adapted.resilience_score - baseline.resilience_score) / baseline.resilience_score) * 100.0;
        }
        
        // Overall improvement (weighted average)
        improvement.overall = improvement.latency * 0.3 +
            improvement.bandwidth * 0.2 +
            improvement.packet_loss * 0.2 +
            improvement.transfer_time * 0.2 +
            improvement.resilience * 0.1;
        
        improvement
    }
    
    /// Handle protocol adaptation for all connections
    fn handle_protocol_adaptation(&mut self) {
        // Skip if adaptation is disabled
        if !self.adaptation_enabled {
            return;
        }
        
        // Collect all connections that need adaptation
        let mut adaptations = Vec::new();
        
        // First pass: identify connections needing adaptation and collect their data
        for (idx, conn) in self.connections.iter().enumerate() {
            if !self.is_adaptation_needed(conn) {
                continue;
            }
            
            // Collect connection data for adaptation
            let conditions = self.get_connection_conditions(conn);
            adaptations.push((idx, conditions));
        }
        
        // Second pass: generate and apply protocols without holding mutable references
        for (conn_idx, conditions) in adaptations {
            // Generate protocol for this connection
            let protocol_engine = self.create_protocol_engine(conditions);
            
            if let Some(protocol) = protocol_engine.generate_protocol() {
                // Apply the protocol
                let source_id = self.connections[conn_idx].source_id;
                let dest_id = self.connections[conn_idx].dest_id;
                
                // Store protocol name and update connection
                self.connections[conn_idx].active_protocol = Some(protocol.name.clone());
                
                // Apply optimizations based on the protocol type
                let protocol_type = protocol.name.clone();
                self.apply_optimizations_to_connection(conn_idx, &protocol_type);
                
                // Store engine for later
                self.protocol_engines.insert(
                    (source_id, dest_id),
                    protocol_engine
                );
            }
        }
    }
    
    /// Apply optimizations to a specific connection based on protocol type
    fn apply_optimizations_to_connection(&mut self, conn_idx: usize, protocol_type: &str) {
        if conn_idx >= self.connections.len() {
            return;
        }
        
        let conn = &mut self.connections[conn_idx];
        
        // Get base metrics before modification
        let base_latency = conn.latency;
        let base_bandwidth = conn.bandwidth;
        let base_packet_loss = conn.packet_loss;
        let base_jitter = conn.jitter;
        
        // Define optimization parameters with appropriate initial values
        let latency_opt;
        let bandwidth_opt;
        let packet_loss_opt;
        let jitter_opt;
        let mut directional_bias = 0.0;
        let mut asymmetric_buffer = 0.0;
        let mut thermal_echo = 0.0;
        
        // Set optimization parameters based on protocol type
        match protocol_type {
            "low_latency" => {
                latency_opt = 0.9;
                bandwidth_opt = 0.3;
                packet_loss_opt = 0.5;
                jitter_opt = 0.7;
                directional_bias = 0.3;
            },
            "high_bandwidth" => {
                latency_opt = 0.2;
                bandwidth_opt = 0.9;
                packet_loss_opt = 0.4;
                jitter_opt = 0.2;
                asymmetric_buffer = 0.5;
            },
            "reliability" => {
                latency_opt = 0.5;
                bandwidth_opt = 0.4;
                packet_loss_opt = 0.9;
                jitter_opt = 0.6;
                directional_bias = 0.7;
            },
            "mobile" => {
                latency_opt = 0.6;
                bandwidth_opt = 0.3;
                packet_loss_opt = 0.4;
                jitter_opt = 0.9;
                thermal_echo = 0.4;
            },
            "satellite" => {
                latency_opt = 0.8;
                bandwidth_opt = 0.2;
                packet_loss_opt = 0.6;
                jitter_opt = 0.4;
                thermal_echo = 0.8;
            },
            "asymmetric" => {
                latency_opt = 0.5;
                bandwidth_opt = 0.7;
                packet_loss_opt = 0.8;
                jitter_opt = 0.6;
                directional_bias = 0.9;
                asymmetric_buffer = 0.8;
                thermal_echo = 0.7;
            },
            _ => {
                // Default optimizations
                latency_opt = 0.3;
                bandwidth_opt = 0.3;
                packet_loss_opt = 0.3;
                jitter_opt = 0.3;
            }
        };
        
        // Calculate improvement factors based on protocol parameters
        let (latency_improve, bandwidth_improve, packet_loss_improve, jitter_improve) = 
            if protocol_type.contains("asymmetric") {
                // Asymmetric network optimizations
                let lat_imp = 0.3 * latency_opt * directional_bias;
                let bw_imp = 0.25 * bandwidth_opt * asymmetric_buffer;
                let pl_imp = 0.4 * packet_loss_opt * directional_bias;
                let jit_imp = 0.3 * jitter_opt * thermal_echo;
                (lat_imp, bw_imp, pl_imp, jit_imp)
            } else if protocol_type.contains("satellite") {
                // Satellite optimizations
                let lat_imp = 0.2 * latency_opt;
                let bw_imp = 0.15 * bandwidth_opt;
                let pl_imp = 0.35 * packet_loss_opt;
                let jit_imp = 0.1 * jitter_opt * thermal_echo;
                (lat_imp, bw_imp, pl_imp, jit_imp)
            } else if protocol_type.contains("mobile") {
                // Mobile network optimizations
                let lat_imp = 0.25 * latency_opt;
                let bw_imp = 0.1 * bandwidth_opt;
                let pl_imp = 0.2 * packet_loss_opt;
                let jit_imp = 0.4 * jitter_opt;
                (lat_imp, bw_imp, pl_imp, jit_imp)
            } else if base_latency < 5.0 && base_bandwidth > 9000.0 {
                // Ideal/fast network - small improvements
                let lat_imp = 0.01 * latency_opt;
                let bw_imp = 0.01 * bandwidth_opt;
                let pl_imp = 0.01 * packet_loss_opt;
                let jit_imp = 0.01 * jitter_opt;
                (lat_imp, bw_imp, pl_imp, jit_imp)
            } else {
                // Default optimizations
                let lat_imp = 0.15 * latency_opt;
                let bw_imp = 0.15 * bandwidth_opt;
                let pl_imp = 0.15 * packet_loss_opt;
                let jit_imp = 0.15 * jitter_opt;
                (lat_imp, bw_imp, pl_imp, jit_imp)
            };
        
        // Apply improvements with proper type handling - using f64 methods to avoid ambiguity
        conn.latency = base_latency * f64::max(0.6, f64::min(1.0, 1.0 - latency_improve));
        conn.bandwidth = base_bandwidth * f64::max(1.0, f64::min(1.5, 1.0 + bandwidth_improve));
        conn.packet_loss = base_packet_loss * f64::max(0.5, f64::min(1.0, 1.0 - packet_loss_improve));
        conn.jitter = base_jitter * f64::max(0.7, f64::min(1.0, 1.0 - jitter_improve));
    }
    
    /// Get node name by ID
    fn get_node_name(&self, node_id: usize) -> Option<String> {
        if let Some(node) = self.nodes.get(&node_id) {
            return Some(node.name().to_string());
        }
        None
    }
    
    /// Check if adaptation is needed based on current conditions
    fn is_adaptation_needed(&self, conn: &NodeConnection) -> bool {
        // If no adaptation has been applied yet, always return true
        if conn.active_protocol.is_none() {
            return true;
        }
        
        // For now, always return true for testing purposes
        // TODO: implement actual condition checks based on thresholds
        true
    }
    
    /// Get network conditions for a connection
    fn get_connection_conditions(&self, conn: &NodeConnection) -> Vec<NetworkCondition> {
        // Create conditions based on connection metrics
        let mut conditions = vec![
            NetworkCondition {
                name: "latency".to_string(),
                value: conn.latency,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            },
            NetworkCondition {
                name: "bandwidth".to_string(),
                value: conn.bandwidth,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            },
            NetworkCondition {
                name: "packet_loss".to_string(),
                value: conn.packet_loss,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            },
            NetworkCondition {
                name: "jitter".to_string(),
                value: conn.jitter,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            },
        ];
        
        // Add special conditions based on connection properties
        if conn.latency > 200.0 {
            conditions.push(NetworkCondition {
                name: "high_latency".to_string(),
                value: 1.0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            });
        }
        
        if conn.packet_loss > 0.1 {
            conditions.push(NetworkCondition {
                name: "high_packet_loss".to_string(),
                value: 1.0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            });
        }
        
        if conn.bandwidth < 1000.0 {
            conditions.push(NetworkCondition {
                name: "low_bandwidth".to_string(),
                value: 1.0,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            });
        }
        
        conditions
    }
}
