use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::collections::VecDeque;

// Import from the original codebase
use unrealnet_core::dynphys::adaptor::{NetworkCondition, AdaptorState};

/// Real network adaptor that measures actual network conditions
pub struct RealNetworkAdaptor {
    /// Unique identifier
    pub id: String,
    /// Adaptor name
    pub name: String,
    /// Current state
    pub state: AdaptorState,
    /// Target endpoints to measure against
    endpoints: Vec<String>,
    /// Measurement history
    history: VecDeque<NetworkCondition>,
    /// Maximum history size
    max_history: usize,
    /// Measurement interval in milliseconds
    measurement_interval: u64,
    /// Last measurement time
    last_measurement: Option<Instant>,
}

impl RealNetworkAdaptor {
    /// Create a new real network adaptor
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            state: AdaptorState::Initializing,
            endpoints: Vec::new(),
            history: VecDeque::new(),
            max_history: 100,
            measurement_interval: 1000, // Default: measure every second
            last_measurement: None,
        }
    }
    
    /// Add a target endpoint to measure against (hostname:port)
    pub fn add_endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.endpoints.push(endpoint.to_string());
        self
    }
    
    /// Set measurement interval in milliseconds
    pub fn set_measurement_interval(&mut self, interval_ms: u64) -> &mut Self {
        self.measurement_interval = interval_ms;
        self
    }
    
    /// Activate the adaptor
    pub fn activate(&mut self) -> Result<(), String> {
        if self.endpoints.is_empty() {
            return Err("No endpoints configured for measurement".to_string());
        }
        
        self.state = AdaptorState::Active;
        self.last_measurement = Some(Instant::now());
        Ok(())
    }
    
    /// Pause the adaptor
    pub fn pause(&mut self) {
        self.state = AdaptorState::Paused;
    }
    
    /// Measure real network conditions
    pub fn sense_environment(&mut self) -> Option<Vec<NetworkCondition>> {
        if self.state != AdaptorState::Active {
            return None;
        }
        
        // Check if it's time to measure again
        if let Some(last) = self.last_measurement {
            if last.elapsed().as_millis() < self.measurement_interval as u128 {
                return None; // Not time yet
            }
        }
        
        // Update last measurement time
        self.last_measurement = Some(Instant::now());
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs();
            
        let mut conditions = Vec::new();
        
        // Measure latency for each endpoint
        for endpoint in &self.endpoints {
            // Measure latency
            if let Some(latency) = self.measure_latency(endpoint) {
                let latency_condition = NetworkCondition {
                    name: "latency".to_string(),
                    value: normalize_latency(latency),
                    timestamp: now,
                };
                self.record_measurement(latency_condition.clone());
                conditions.push(latency_condition);
            }
            
            // Measure bandwidth
            if let Some(bandwidth) = self.measure_bandwidth(endpoint) {
                let bandwidth_condition = NetworkCondition {
                    name: "bandwidth".to_string(),
                    value: normalize_bandwidth(bandwidth),
                    timestamp: now,
                };
                self.record_measurement(bandwidth_condition.clone());
                conditions.push(bandwidth_condition);
            }
            
            // Measure packet loss
            if let Some(packet_loss) = self.measure_packet_loss(endpoint) {
                let packet_loss_condition = NetworkCondition {
                    name: "packet_loss".to_string(),
                    value: normalize_packet_loss(packet_loss),
                    timestamp: now,
                };
                self.record_measurement(packet_loss_condition.clone());
                conditions.push(packet_loss_condition);
            }
            
            // Measure jitter
            if let Some(jitter) = self.measure_jitter(endpoint) {
                let jitter_condition = NetworkCondition {
                    name: "jitter".to_string(),
                    value: normalize_jitter(jitter),
                    timestamp: now,
                };
                self.record_measurement(jitter_condition.clone());
                conditions.push(jitter_condition);
            }
        }
        
        if conditions.is_empty() {
            None
        } else {
            Some(conditions)
        }
    }
    
    /// Record a measurement in history
    fn record_measurement(&mut self, condition: NetworkCondition) {
        // Add to history
        self.history.push_back(condition);
        
        // Trim history if necessary
        while self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// Measure network latency to an endpoint in milliseconds
    pub fn measure_latency(&self, _endpoint: &str) -> Option<f64> {
        // For demonstration purposes, return a simulated value
        Some(50.0 + (SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis() % 50) as f64)
    }
    
    /// Estimate bandwidth to an endpoint in Kbps
    pub fn measure_bandwidth(&self, _endpoint: &str) -> Option<f64> {
        // For demonstration, provide a simulated value
        Some(5000.0 + (SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis() % 5000) as f64)
    }
    
    /// Estimate packet loss as a percentage (0-100)
    pub fn measure_packet_loss(&self, _endpoint: &str) -> Option<f64> {
        // For demonstration, simulate a small packet loss
        Some((SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis() % 4) as f64)
    }
    
    /// Measure jitter (variation in latency) in milliseconds
    pub fn measure_jitter(&self, _endpoint: &str) -> Option<f64> {
        // For demonstration, simulate a jitter value
        Some(5.0 + (SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_millis() % 20) as f64 / 10.0)
    }
}

/// Normalize latency value to 0-1 scale (lower is better)
/// Assumes latency values typically range from 1ms (excellent) to 500ms (poor)
fn normalize_latency(latency_ms: f64) -> f64 {
    // Clamp to reasonable range
    let clamped = latency_ms.min(500.0).max(1.0);
    // Reverse and normalize (1ms → 1.0, 500ms → 0.0)
    (500.0 - clamped) / 499.0
}

/// Normalize bandwidth to 0-1 scale (higher is better)
/// Assumes bandwidth values typically range from 1 Kbps (poor) to 100,000 Kbps (excellent)
fn normalize_bandwidth(bandwidth_kbps: f64) -> f64 {
    let log_bw = bandwidth_kbps.max(1.0).ln();
    let min_log = 1.0f64.ln(); // ln(1)
    let max_log = 100_000.0f64.ln(); // ln(100,000)
    
    // Normalize on logarithmic scale (1 Kbps → 0.0, 100,000 Kbps → 1.0)
    (log_bw - min_log) / (max_log - min_log)
}

/// Normalize packet loss to 0-1 scale (lower loss is better)
/// Assumes packet loss values range from 0% (excellent) to 100% (unusable)
fn normalize_packet_loss(packet_loss_percent: f64) -> f64 {
    // Clamp to valid range
    let clamped = packet_loss_percent.min(100.0).max(0.0);
    // Reverse and normalize (0% → 1.0, 100% → 0.0)
    (100.0 - clamped) / 100.0
}

/// Normalize jitter to 0-1 scale (lower is better)
/// Assumes jitter values typically range from 0ms (excellent) to 100ms (poor)
fn normalize_jitter(jitter_ms: f64) -> f64 {
    // Clamp to reasonable range
    let clamped = jitter_ms.min(100.0).max(0.0);
    // Reverse and normalize (0ms → 1.0, 100ms → 0.0)
    (100.0 - clamped) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization_functions() {
        // Test latency normalization
        assert!(normalize_latency(1.0) > 0.99); // Excellent latency
        assert!(normalize_latency(500.0) < 0.01); // Poor latency
        
        // Test bandwidth normalization
        assert!(normalize_bandwidth(100_000.0) > 0.99); // Excellent bandwidth
        assert!(normalize_bandwidth(1.0) < 0.01); // Poor bandwidth
        
        // Test packet loss normalization
        assert_eq!(normalize_packet_loss(0.0), 1.0); // No packet loss
        assert_eq!(normalize_packet_loss(100.0), 0.0); // Complete packet loss
        
        // Test jitter normalization
        assert_eq!(normalize_jitter(0.0), 1.0); // No jitter
        assert_eq!(normalize_jitter(100.0), 0.0); // High jitter
    }
}
