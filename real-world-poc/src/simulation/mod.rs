//! Large-scale network simulation module for Dynamic Protocols Infra Physics Generator
//! This module provides a comprehensive network simulation framework for showcasing
//! the power of dynamic protocol adaptation in realistic environments.

// Import internal modules
mod network;
mod scenarios;
mod metrics;
mod visualizer;
mod nodes;

// Re-export types needed by main
pub use network::NetworkSimulation;
pub use scenarios::ScenarioManager;
pub use visualizer::PerformanceVisualizer as SimulationVisualizer;
pub use metrics::{MetricsCollector, ErrorString};

use std::time::Duration;

/// Core simulation controller that manages the entire demonstration
pub struct LargeScaleSimulator {
    /// Network simulation
    pub network: NetworkSimulation,
    /// Scenario manager
    pub scenarios: ScenarioManager,
    /// Performance visualizer
    pub visualizer: SimulationVisualizer,
    /// Metrics collector
    pub metrics: MetricsCollector,
}

/// Configuration for the simulation
pub struct SimulationConfig {
    /// Number of nodes in the network
    pub node_count: usize,
    /// Connection density (0.0-1.0)
    pub connection_density: f64,
    /// Duration in seconds
    pub duration_secs: u64,
    /// Enable live visualization
    pub enable_live_visualization: bool,
}

impl LargeScaleSimulator {
    /// Create a new simulator
    pub fn new() -> Self {
        Self {
            network: NetworkSimulation::new(),
            scenarios: ScenarioManager::new(),
            visualizer: SimulationVisualizer::new(),
            metrics: MetricsCollector::new(),
        }
    }
    
    /// Initialize the simulator with the given configuration
    pub fn initialize(&mut self, config: &SimulationConfig) -> Result<(), ErrorString> {
        println!("Initializing large-scale network simulation with {} nodes", config.node_count);
        
        // Initialize network
        self.network.initialize(config.node_count, config.connection_density)?;
        
        // Initialize metrics
        self.metrics.initialize(Duration::from_secs(config.duration_secs));
        
        // Load predefined scenarios
        self.scenarios.load_predefined_scenarios();
        
        println!("Initialization complete");
        println!("Network topology: {} nodes with {} connections", 
                self.network.node_count(), self.network.connection_count());
        
        Ok(())
    }
    
    /// Run the simulation
    pub fn run(&mut self, duration_secs: u64) -> Result<(), ErrorString> {
        println!("Starting large-scale network simulation for {} seconds", duration_secs);
        println!("Network topology: {} nodes with {} connections", 
                self.network.node_count(), self.network.connection_count());
        
        // First run baseline without protocol adaptation
        self.run_baseline(duration_secs / 2)?;
        
        // Then run with protocol adaptation
        self.run_with_adaptation(duration_secs / 2)?;
        
        // Generate reports
        self.generate_reports()?;
        
        println!("Simulation completed successfully!");
        
        Ok(())
    }
    
    /// Run baseline without protocol adaptation
    fn run_baseline(&mut self, duration_secs: u64) -> Result<(), ErrorString> {
        self.network.set_adaptation_enabled(false);
        
        // Run through each scenario for the baseline
        for scenario in self.scenarios.get_all_scenarios() {
            println!("Running baseline with scenario: {}", scenario.name);
            self.network.apply_scenario(&scenario);
            self.network.run(Duration::from_secs(duration_secs / 8))?;
            self.metrics.collect_baseline_metrics(&self.network);
        }
        
        Ok(())
    }
    
    /// Run with dynamic protocol adaptation
    fn run_with_adaptation(&mut self, duration_secs: u64) -> Result<(), ErrorString> {
        self.network.set_adaptation_enabled(true);
        
        // Run through each scenario with adaptation
        for scenario in self.scenarios.get_all_scenarios() {
            println!("Running with adaptation in scenario: {}", scenario.name);
            self.network.apply_scenario(&scenario);
            self.network.run(Duration::from_secs(duration_secs / 8))?;
            self.metrics.collect_adaptation_metrics(&self.network);
            // Protocol usage tracking is handled internally by metrics collector
            
            // Generate visualization for this scenario
            self.visualizer.generate_final_visualizations(&self.network, &self.metrics)?;
        }
        
        // Print summary statistics
        println!("\nSimulation Statistics:");
        println!("----------------------");
        println!("Average protocol adaptation time: {:.2} ms", self.metrics.avg_adaptation_time());
        println!("Total protocol switches: {}", self.metrics.protocol_switch_count());
        println!("Most used physics model: {}", self.metrics.most_used_model());
        
        Ok(())
    }
    
    /// Generate final reports and visualizations
    fn generate_reports(&self) -> Result<(), ErrorString> {
        // Generate performance reports
        let result = self.metrics.generate_summary_report();
        if let Err(err) = result {
            return Err(err);
        }
        
        // Generate visualizations
        let result = self.visualizer.generate_final_visualizations(&self.network, &self.metrics);
        if let Err(err) = result {
            return Err(err);
        }
        
        Ok(())
    }
}
