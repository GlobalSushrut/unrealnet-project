//! Network scenarios for the simulation
//! Provides predefined network scenarios with different conditions to showcase
//! the adaptive capabilities of the Dynamic Protocols Infra Physics Generator.

/// Network scenario with specific conditions
#[derive(Debug, Clone)]
pub struct NetworkScenario {
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Base latency in ms
    pub base_latency: f64,
    /// Base bandwidth in Kbps
    pub base_bandwidth: f64,
    /// Base packet loss (0.0-1.0)
    pub base_packet_loss: f64,
    /// Base jitter in ms
    pub base_jitter: f64,
    /// Latency variation range
    pub latency_variation: f64,
    /// Bandwidth variation range
    pub bandwidth_variation: f64,
    /// Packet loss variation range
    pub packet_loss_variation: f64,
    /// Jitter variation range
    pub jitter_variation: f64,
}

impl NetworkScenario {
    /// Create a new network scenario
    pub fn new(
        name: &str,
        description: &str,
        base_latency: f64,
        base_bandwidth: f64,
        base_packet_loss: f64,
        base_jitter: f64,
        latency_variation: f64,
        bandwidth_variation: f64,
        packet_loss_variation: f64,
        jitter_variation: f64,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            base_latency,
            base_bandwidth,
            base_packet_loss,
            base_jitter,
            latency_variation,
            bandwidth_variation,
            packet_loss_variation,
            jitter_variation,
        }
    }
}

/// Manager for network scenarios
pub struct ScenarioManager {
    /// Available scenarios
    scenarios: std::collections::HashMap<String, NetworkScenario>,
}

impl ScenarioManager {
    /// Create a new scenario manager
    pub fn new() -> Self {
        Self {
            scenarios: std::collections::HashMap::new(),
        }
    }
    
    /// Load predefined scenarios
    pub fn load_predefined_scenarios(&mut self) {
        // Clear existing scenarios
        self.scenarios.clear();
        
        // Add idealized network scenario
        self.add_scenario(NetworkScenario::new(
            "ideal",
            "Ideal network conditions with low latency, high bandwidth, and minimal packet loss",
            20.0,
            10000.0,
            0.001,
            1.0,
            5.0,
            1000.0,
            0.002,
            0.5,
        ));
        
        // Add congestion scenario
        self.add_scenario(NetworkScenario::new(
            "congestion",
            "Network congestion with high latency and reduced bandwidth",
            120.0,
            2000.0,
            0.02,
            15.0,
            50.0,
            1000.0,
            0.03,
            10.0,
        ));
        
        // Add long-distance international scenario
        self.add_scenario(NetworkScenario::new(
            "international",
            "International connections with high latency and moderate bandwidth",
            200.0,
            5000.0,
            0.01,
            8.0,
            30.0,
            1000.0,
            0.01,
            5.0,
        ));
        
        // Add wireless interference scenario
        self.add_scenario(NetworkScenario::new(
            "wireless_interference",
            "Wireless networks with interference causing packet loss and jitter",
            50.0,
            3000.0,
            0.05,
            20.0,
            20.0,
            1500.0,
            0.1,
            15.0,
        ));
        
        // Add mobile handover scenario
        self.add_scenario(NetworkScenario::new(
            "mobile_handover",
            "Mobile devices during cell tower handover with unstable connections",
            80.0,
            2000.0,
            0.1,
            25.0,
            40.0,
            1000.0,
            0.15,
            20.0,
        ));
        
        // Add asymmetric bandwidth scenario
        self.add_scenario(NetworkScenario::new(
            "asymmetric",
            "Asymmetric connections with high download but low upload speeds",
            40.0,
            8000.0,
            0.01,
            5.0,
            10.0,
            2000.0,
            0.02,
            3.0,
        ));
        
        // Add satellite connection scenario
        self.add_scenario(NetworkScenario::new(
            "satellite",
            "Satellite connections with very high latency but decent bandwidth",
            500.0,
            5000.0,
            0.02,
            10.0,
            100.0,
            1000.0,
            0.03,
            8.0,
        ));
        
        // Add extreme conditions scenario
        self.add_scenario(NetworkScenario::new(
            "extreme",
            "Extreme network conditions with high latency, low bandwidth, and high packet loss",
            300.0,
            500.0,
            0.2,
            50.0,
            100.0,
            300.0,
            0.2,
            30.0,
        ));
        
        println!("Loaded {} predefined network scenarios", self.scenarios.len());
    }
    
    /// Add a scenario
    pub fn add_scenario(&mut self, scenario: NetworkScenario) {
        self.scenarios.insert(scenario.name.clone(), scenario);
    }
    
    /// Get all scenarios as a vector
    pub fn get_all_scenarios(&self) -> Vec<NetworkScenario> {
        self.scenarios.values().cloned().collect()
    }
    
    /// Get a specific scenario by name
    pub fn get_scenario(&self, name: &str) -> Option<NetworkScenario> {
        self.scenarios.get(name).cloned()
    }
    
    /// Get number of scenarios
    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }
}
