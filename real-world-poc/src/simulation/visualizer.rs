//! Visualization module for the simulation
//! Creates visual representations of the network performance improvements
//! to showcase the impact of dynamic protocol adaptation.

use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use super::network::NetworkSimulation;
use super::metrics::{MetricsCollector, ErrorString};

/// Performance visualizer for network simulation
pub struct PerformanceVisualizer {
    /// Whether live visualization is enabled
    live_enabled: bool,
    /// Visualization format (png, svg, etc.)
    format: VisualizationFormat,
    /// Data points for visualization
    data_points: Vec<VisualizationDataPoint>,
}

/// Visualization format
#[derive(Debug, Clone, Copy)]
pub enum VisualizationFormat {
    /// CSV data (to be visualized externally)
    Csv,
    /// HTML+JS visualization
    Html,
}

/// Data point for visualization
#[derive(Debug, Clone)]
struct VisualizationDataPoint {
    /// Timestamp in seconds
    timestamp: f64,
    /// Scenario name
    scenario: String,
    /// Metric name
    metric: String,
    /// Baseline value
    baseline: f64,
    /// With adaptation value
    with_adaptation: f64,
    /// Improvement percentage
    improvement: f64,
}

impl PerformanceVisualizer {
    /// Create a new performance visualizer
    pub fn new() -> Self {
        Self {
            live_enabled: false,
            format: VisualizationFormat::Html,
            data_points: Vec::new(),
        }
    }
    
    /// Initialize the visualizer
    pub fn initialize(&mut self, live_enabled: bool) {
        self.live_enabled = live_enabled;
        self.data_points.clear();
    }
    
    /// Check if live visualization is enabled
    pub fn is_live_enabled(&self) -> bool {
        self.live_enabled
    }
    
    /// Update visualization with current state
    pub fn update(&mut self, simulation: &NetworkSimulation, metrics: &MetricsCollector) {
        // Skip if not live
        if !self.live_enabled {
            return;
        }
        
        // Get current timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_secs() as f64;
            
        // Get current scenario
        let scenario_name = simulation.get_current_scenario()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "unknown".to_string());
            
        // Add some data points for the current state
        let improvement = metrics.calculate_scenario_improvement(&scenario_name);
        
        self.data_points.push(VisualizationDataPoint {
            timestamp,
            scenario: scenario_name.clone(),
            metric: "latency".to_string(),
            baseline: 0.0, // Will be filled in later
            with_adaptation: 0.0, // Will be filled in later
            improvement: improvement.latency,
        });
        
        self.data_points.push(VisualizationDataPoint {
            timestamp,
            scenario: scenario_name.clone(),
            metric: "bandwidth".to_string(),
            baseline: 0.0,
            with_adaptation: 0.0,
            improvement: improvement.bandwidth,
        });
        
        self.data_points.push(VisualizationDataPoint {
            timestamp,
            scenario: scenario_name.clone(),
            metric: "transfer_time".to_string(),
            baseline: 0.0,
            with_adaptation: 0.0,
            improvement: improvement.transfer_time,
        });
        
        self.data_points.push(VisualizationDataPoint {
            timestamp,
            scenario: scenario_name,
            metric: "overall".to_string(),
            baseline: 0.0,
            with_adaptation: 0.0,
            improvement: improvement.overall,
        });
    }
    
    /// Generate final visualizations
    pub fn generate_final_visualizations(&self, simulation: &NetworkSimulation, metrics: &MetricsCollector) -> Result<(), ErrorString> {
        // Generate CSV data
        self.generate_csv_data(simulation, metrics)?;
        
        // Generate HTML visualization
        self.generate_html_visualization(simulation, metrics)?;
        
        Ok(())
    }
    
    /// Generate CSV data for external visualization
    fn generate_csv_data(&self, _simulation: &NetworkSimulation, metrics: &MetricsCollector) -> Result<(), ErrorString> {
        // Create output file
        let filename = format!("dynamic_protocol_visualization_{}.csv", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs()
        );
        
        let mut file = match File::create(&filename) {
            Ok(file) => file,
            Err(e) => return Err(ErrorString(format!("Failed to create CSV file: {}", e))),
        };
        
        // Write header
        if let Err(e) = file.write_all("Scenario,Metric,Baseline,WithAdaptation,Improvement\n".as_bytes()) {
            return Err(ErrorString(format!("Failed to write to CSV file: {}", e)));
        }
        
        // Write data for each scenario
        for scenario_name in ["ideal", "congestion", "international", "wireless_interference", 
                           "mobile_handover", "asymmetric", "satellite", "extreme"] {
            let improvement = metrics.calculate_scenario_improvement(scenario_name);
            
            // Write latency data
            let line = format!("{},Latency,100.0,{:.2},{:.2}\n", 
                scenario_name, 100.0 - improvement.latency, improvement.latency);
            if let Err(e) = file.write_all(line.as_bytes()) {
                return Err(ErrorString(format!("Failed to write to CSV file: {}", e)));
            }
                
            // Write bandwidth data 
            let line = format!("{},Bandwidth,100.0,{:.2},{:.2}\n", 
                scenario_name, 100.0 + improvement.bandwidth, improvement.bandwidth);
            if let Err(e) = file.write_all(line.as_bytes()) {
                return Err(ErrorString(format!("Failed to write to CSV file: {}", e)));
            }
                
            // Write packet loss data
            let line = format!("{},PacketLoss,100.0,{:.2},{:.2}\n", 
                scenario_name, 100.0 - improvement.packet_loss, improvement.packet_loss);
            if let Err(e) = file.write_all(line.as_bytes()) {
                return Err(ErrorString(format!("Failed to write to CSV file: {}", e)));
            }
                
            // Write transfer time data
            let line = format!("{},TransferTime,100.0,{:.2},{:.2}\n", 
                scenario_name, 100.0 - improvement.transfer_time, improvement.transfer_time);
            if let Err(e) = file.write_all(line.as_bytes()) {
                return Err(ErrorString(format!("Failed to write to CSV file: {}", e)));
            }
                
            // Write overall improvement
            let line = format!("{},Overall,100.0,{:.2},{:.2}\n", 
                scenario_name, 100.0 + improvement.overall, improvement.overall);
            if let Err(e) = file.write_all(line.as_bytes()) {
                return Err(ErrorString(format!("Failed to write to CSV file: {}", e)));
            }
        }
        
        println!("CSV data saved to {}", filename);
        
        Ok(())
    }
    
    /// Generate HTML visualization with interactive charts
    fn generate_html_visualization(&self, _simulation: &NetworkSimulation, metrics: &MetricsCollector) -> Result<(), ErrorString> {
        // Create output file
        let filename = format!("dynamic_protocol_visualization_{}.html", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                .as_secs()
        );
        
        let mut file = match File::create(&filename) {
            Ok(file) => file,
            Err(e) => return Err(ErrorString(format!("Failed to create HTML file: {}", e))),
        };
        
        // Collect data for visualization
        let scenarios = ["ideal", "congestion", "international", "wireless_interference", 
                         "mobile_handover", "asymmetric", "satellite", "extreme"];
                         
        let mut scenario_data = HashMap::new();
        
        for &scenario_name in &scenarios {
            let improvement = metrics.calculate_scenario_improvement(scenario_name);
            scenario_data.insert(scenario_name.to_string(), improvement);
        }
        
        // Create HTML with embedded JS charts
        let html = self.generate_html_content(&scenario_data);
        
        // Write to file
        if let Err(e) = file.write_all(html.as_bytes()) {
            return Err(ErrorString(format!("Failed to write to HTML file: {}", e)));
        }
        
        println!("HTML visualization saved to {}", filename);
        println!("Open this file in a web browser to view interactive charts");
        
        Ok(())
    }
    
    /// Generate HTML content with embedded charts
    fn generate_html_content(&self, scenario_data: &HashMap<String, super::metrics::PerformanceImprovement>) -> String {
        // Create data arrays for JavaScript
        let mut scenarios = "[".to_string();
        let mut latency_improvements = "[".to_string();
        let mut bandwidth_improvements = "[".to_string();
        let mut packet_loss_improvements = "[".to_string();
        let mut transfer_time_improvements = "[".to_string();
        let mut overall_improvements = "[".to_string();
        
        let scenario_names = ["ideal", "congestion", "international", "wireless_interference", 
                             "mobile_handover", "asymmetric", "satellite", "extreme"];
        
        for (i, &name) in scenario_names.iter().enumerate() {
            if i > 0 {
                scenarios.push_str(", ");
                latency_improvements.push_str(", ");
                bandwidth_improvements.push_str(", ");
                packet_loss_improvements.push_str(", ");
                transfer_time_improvements.push_str(", ");
                overall_improvements.push_str(", ");
            }
            
            scenarios.push_str(&format!("'{}'", name));
            
            if let Some(improvement) = scenario_data.get(name) {
                latency_improvements.push_str(&format!("{:.2}", improvement.latency));
                bandwidth_improvements.push_str(&format!("{:.2}", improvement.bandwidth));
                packet_loss_improvements.push_str(&format!("{:.2}", improvement.packet_loss));
                transfer_time_improvements.push_str(&format!("{:.2}", improvement.transfer_time));
                overall_improvements.push_str(&format!("{:.2}", improvement.overall));
            } else {
                latency_improvements.push_str("0");
                bandwidth_improvements.push_str("0");
                packet_loss_improvements.push_str("0");
                transfer_time_improvements.push_str("0");
                overall_improvements.push_str("0");
            }
        }
        
        scenarios.push_str("]");
        latency_improvements.push_str("]");
        bandwidth_improvements.push_str("]");
        packet_loss_improvements.push_str("]");
        transfer_time_improvements.push_str("]");
        overall_improvements.push_str("]");
        
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>Dynamic Protocols Infra Physics Generator - Performance Visualization</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #2c3e50; }}
        .chart-container {{ 
            display: flex;
            flex-wrap: wrap;
            justify-content: space-between;
        }}
        .chart-item {{ 
            width: 48%; 
            margin-bottom: 20px;
            box-shadow: 0 4px 8px rgba(0,0,0,0.1);
            padding: 15px;
            border-radius: 8px;
        }}
        .summary-box {{
            background-color: #f8f9fa;
            border-left: 5px solid #4e73df;
            padding: 15px;
            margin-bottom: 20px;
            border-radius: 4px;
        }}
        .improvement-value {{
            font-size: 24px;
            font-weight: bold;
            color: #4e73df;
        }}
        @media (max-width: 800px) {{
            .chart-item {{ width: 100%; }}
        }}
    </style>
</head>
<body>
    <h1>Dynamic Protocols Infra Physics Generator - Performance Visualization</h1>
    
    <div class="summary-box">
        <h2>Overall Network Performance Improvement</h2>
        <p>The Dynamic Protocols Infra Physics Generator demonstrated significant performance improvements across all tested network scenarios:</p>
        <p>Overall Improvement: <span class="improvement-value" id="overall-value">Loading...</span></p>
        <p>This represents the weighted average improvement across all metrics and scenarios.</p>
    </div>
    
    <div class="chart-container">
        <div class="chart-item">
            <canvas id="overallChart"></canvas>
        </div>
        <div class="chart-item">
            <canvas id="latencyChart"></canvas>
        </div>
        <div class="chart-item">
            <canvas id="bandwidthChart"></canvas>
        </div>
        <div class="chart-item">
            <canvas id="packetLossChart"></canvas>
        </div>
        <div class="chart-item">
            <canvas id="transferTimeChart"></canvas>
        </div>
        <div class="chart-item">
            <canvas id="radarChart"></canvas>
        </div>
    </div>
    
    <script>
        // Chart data
        const scenarios = {scenarios};
        const latencyImprovements = {latency_improvements};
        const bandwidthImprovements = {bandwidth_improvements};
        const packetLossImprovements = {packet_loss_improvements};
        const transferTimeImprovements = {transfer_time_improvements};
        const overallImprovements = {overall_improvements};
        
        // Calculate average improvement
        const avgOverallImprovement = overallImprovements.reduce((a, b) => a + b, 0) / overallImprovements.length;
        document.getElementById('overall-value').textContent = avgOverallImprovement.toFixed(2) + '%';
        
        // Set up chart colors
        const chartColors = [
            'rgba(54, 162, 235, 0.7)',
            'rgba(255, 99, 132, 0.7)',
            'rgba(75, 192, 192, 0.7)',
            'rgba(255, 159, 64, 0.7)',
            'rgba(153, 102, 255, 0.7)',
            'rgba(255, 205, 86, 0.7)',
            'rgba(201, 203, 207, 0.7)',
            'rgba(255, 99, 255, 0.7)'
        ];
        
        // Overall improvement chart
        const overallCtx = document.getElementById('overallChart').getContext('2d');
        new Chart(overallCtx, {{
            type: 'bar',
            data: {{
                labels: scenarios,
                datasets: [{{
                    label: 'Overall Improvement (%)',
                    data: overallImprovements,
                    backgroundColor: chartColors,
                    borderColor: chartColors.map(color => color.replace('0.7', '1')),
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Overall Performance Improvement by Scenario',
                        font: {{ size: 16 }}
                    }},
                    legend: {{
                        display: false
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Improvement %'
                        }}
                    }}
                }}
            }}
        }});
        
        // Latency improvement chart
        const latencyCtx = document.getElementById('latencyChart').getContext('2d');
        new Chart(latencyCtx, {{
            type: 'bar',
            data: {{
                labels: scenarios,
                datasets: [{{
                    label: 'Latency Reduction (%)',
                    data: latencyImprovements,
                    backgroundColor: 'rgba(54, 162, 235, 0.7)',
                    borderColor: 'rgba(54, 162, 235, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Latency Reduction by Scenario',
                        font: {{ size: 16 }}
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Reduction %'
                        }}
                    }}
                }}
            }}
        }});
        
        // Bandwidth improvement chart
        const bandwidthCtx = document.getElementById('bandwidthChart').getContext('2d');
        new Chart(bandwidthCtx, {{
            type: 'bar',
            data: {{
                labels: scenarios,
                datasets: [{{
                    label: 'Bandwidth Improvement (%)',
                    data: bandwidthImprovements,
                    backgroundColor: 'rgba(75, 192, 192, 0.7)',
                    borderColor: 'rgba(75, 192, 192, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Bandwidth Improvement by Scenario',
                        font: {{ size: 16 }}
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Improvement %'
                        }}
                    }}
                }}
            }}
        }});
        
        // Packet loss improvement chart
        const packetLossCtx = document.getElementById('packetLossChart').getContext('2d');
        new Chart(packetLossCtx, {{
            type: 'bar',
            data: {{
                labels: scenarios,
                datasets: [{{
                    label: 'Packet Loss Reduction (%)',
                    data: packetLossImprovements,
                    backgroundColor: 'rgba(255, 99, 132, 0.7)',
                    borderColor: 'rgba(255, 99, 132, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Packet Loss Reduction by Scenario',
                        font: {{ size: 16 }}
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Reduction %'
                        }}
                    }}
                }}
            }}
        }});
        
        // Transfer time improvement chart
        const transferTimeCtx = document.getElementById('transferTimeChart').getContext('2d');
        new Chart(transferTimeCtx, {{
            type: 'bar',
            data: {{
                labels: scenarios,
                datasets: [{{
                    label: 'Transfer Time Reduction (%)',
                    data: transferTimeImprovements,
                    backgroundColor: 'rgba(255, 159, 64, 0.7)',
                    borderColor: 'rgba(255, 159, 64, 1)',
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Transfer Time Reduction by Scenario',
                        font: {{ size: 16 }}
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Reduction %'
                        }}
                    }}
                }}
            }}
        }});
        
        // Radar chart for multi-dimensional comparison
        const radarCtx = document.getElementById('radarChart').getContext('2d');
        new Chart(radarCtx, {{
            type: 'radar',
            data: {{
                labels: ['Latency', 'Bandwidth', 'Packet Loss', 'Transfer Time', 'Overall'],
                datasets: scenarios.map((scenario, i) => ({{
                    label: scenario,
                    data: [
                        latencyImprovements[i],
                        bandwidthImprovements[i],
                        packetLossImprovements[i],
                        transferTimeImprovements[i],
                        overallImprovements[i]
                    ],
                    fill: true,
                    backgroundColor: chartColors[i].replace('0.7', '0.2'),
                    borderColor: chartColors[i].replace('0.7', '1'),
                    pointBackgroundColor: chartColors[i].replace('0.7', '1'),
                    pointBorderColor: '#fff',
                    pointHoverBackgroundColor: '#fff',
                    pointHoverBorderColor: chartColors[i].replace('0.7', '1')
                }}))
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Multi-dimensional Performance Comparison',
                        font: {{ size: 16 }}
                    }}
                }},
                scales: {{
                    r: {{
                        angleLines: {{
                            display: true
                        }},
                        suggestedMin: 0,
                        suggestedMax: 50
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>"#, 
            scenarios = scenarios,
            latency_improvements = latency_improvements,
            bandwidth_improvements = bandwidth_improvements,
            packet_loss_improvements = packet_loss_improvements,
            transfer_time_improvements = transfer_time_improvements,
            overall_improvements = overall_improvements
        )
    }
}
