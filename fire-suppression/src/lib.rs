use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::Duration;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Fire suppression system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireSuppressionConfig {
    /// Temperature threshold for automatic activation (Celsius)
    pub auto_activation_temp: f32,
    /// Smoke detection sensitivity (0.0-1.0)
    pub smoke_sensitivity: f32,
    /// Maximum discharge duration in seconds
    pub max_discharge_duration: u32,
    /// Cooldown period between activations (seconds)
    pub cooldown_period: u32,
    /// Allow manual override even during cooldown
    pub allow_manual_override: bool,
    /// Minimum extinguisher pressure for operation (PSI)
    pub min_pressure: f32,
}

impl Default for FireSuppressionConfig {
    fn default() -> Self {
        Self {
            auto_activation_temp: 60.0,  // 60°C / 140°F
            smoke_sensitivity: 0.7,
            max_discharge_duration: 10,   // 10 seconds max burst
            cooldown_period: 30,          // 30 second cooldown
            allow_manual_override: true,
            min_pressure: 100.0,          // 100 PSI minimum
        }
    }
}

/// Current state of the fire suppression system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireSuppressionState {
    pub system_armed: bool,
    pub extinguisher_pressure: f32,      // PSI
    pub extinguisher_capacity: f32,      // Percentage remaining
    pub nozzle_position: NozzlePosition,
    pub current_temperature: f32,        // Celsius
    pub smoke_level: f32,               // 0.0-1.0
    pub last_activation: Option<DateTime<Utc>>,
    pub total_activations: u32,
    pub system_health: SystemHealth,
    pub discharge_active: bool,
    pub manual_override_active: bool,
}

impl Default for FireSuppressionState {
    fn default() -> Self {
        Self {
            system_armed: true,
            extinguisher_pressure: 150.0,  // Full pressure
            extinguisher_capacity: 100.0,  // Full capacity
            nozzle_position: NozzlePosition::Retracted,
            current_temperature: 20.0,     // Room temperature
            smoke_level: 0.0,              // No smoke
            last_activation: None,
            total_activations: 0,
            system_health: SystemHealth::Optimal,
            discharge_active: false,
            manual_override_active: false,
        }
    }
}

/// Nozzle positioning system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NozzlePosition {
    Retracted,    // Stored position
    Deployed,     // Extended for suppression
    Targeting,    // Aiming at fire source
    Emergency,    // Emergency deployment position
}

impl NozzlePosition {
    pub fn description(&self) -> &'static str {
        match self {
            NozzlePosition::Retracted => "Nozzle retracted and secured",
            NozzlePosition::Deployed => "Nozzle deployed and ready",
            NozzlePosition::Targeting => "Nozzle targeting fire source",
            NozzlePosition::Emergency => "Emergency deployment active",
        }
    }
}

/// System health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SystemHealth {
    Optimal,      // All systems green
    Degraded,     // Some issues but functional
    Critical,     // Major problems, limited function
    Offline,      // System non-functional
}

/// Fire detection event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: FireEventType,
    pub temperature: f32,
    pub smoke_level: f32,
    pub location_estimate: Option<(f32, f32)>, // Relative x, y coordinates
    pub severity: FireSeverity,
    pub response_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FireEventType {
    TemperatureSpike,
    SmokeDetected,
    FlameDetected,
    FireSuppressed,
    SystemActivated,
    ManualOverride,
    EmergencyShutdown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum FireSeverity {
    Low,      // Minor heat/smoke
    Medium,   // Significant fire risk
    High,     // Active fire requiring suppression
    Critical, // Major fire emergency
}

/// Main fire suppression system
pub struct FireSuppressionSystem {
    config: FireSuppressionConfig,
    state: FireSuppressionState,
    event_history: Vec<FireEvent>,
    // Hardware controllers (placeholders)
    temperature_sensor: TemperatureSensor,
    smoke_detector: SmokeDetector,
    extinguisher_valve: ExtinguisherValve,
    nozzle_actuator: NozzleActuator,
}

impl FireSuppressionSystem {
    pub fn new(config: FireSuppressionConfig) -> Self {
        Self {
            config,
            state: FireSuppressionState::default(),
            event_history: Vec::new(),
            temperature_sensor: TemperatureSensor::new(),
            smoke_detector: SmokeDetector::new(),
            extinguisher_valve: ExtinguisherValve::new(),
            nozzle_actuator: NozzleActuator::new(),
        }
    }

    /// Main monitoring and response loop
    pub async fn monitor_and_respond(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Update sensor readings
        self.update_sensors().await?;
        
        // Assess fire risk
        let fire_risk = self.assess_fire_risk();
        
        // Respond based on risk level
        match fire_risk {
            FireSeverity::Low => {
                // Continue monitoring
                if self.state.discharge_active {
                    self.stop_discharge().await?;
                }
            },
            FireSeverity::Medium => {
                // Prepare for suppression
                self.prepare_for_suppression().await?;
            },
            FireSeverity::High => {
                // Activate suppression
                self.activate_suppression(false).await?;
            },
            FireSeverity::Critical => {
                // Emergency suppression
                self.activate_suppression(true).await?;
            },
        }

        Ok(())
    }

    /// Update sensor readings
    async fn update_sensors(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Read temperature
        self.state.current_temperature = self.temperature_sensor.read_temperature().await?;
        
        // Read smoke level
        self.state.smoke_level = self.smoke_detector.read_smoke_level().await?;
        
        // Update extinguisher status
        self.state.extinguisher_pressure = self.extinguisher_valve.read_pressure().await?;
        
        // Check system health
        self.update_system_health();

        Ok(())
    }

    /// Assess current fire risk level
    fn assess_fire_risk(&self) -> FireSeverity {
        let temp_factor = if self.state.current_temperature > self.config.auto_activation_temp {
            (self.state.current_temperature - 20.0) / 50.0 // Normalize to 0-1 range
        } else {
            0.0
        };

        let smoke_factor = self.state.smoke_level;
        
        // Combined risk score
        let risk_score = (temp_factor * 0.6) + (smoke_factor * 0.4);

        if risk_score >= 0.8 {
            FireSeverity::Critical
        } else if risk_score >= 0.6 {
            FireSeverity::High
        } else if risk_score >= 0.3 {
            FireSeverity::Medium
        } else {
            FireSeverity::Low
        }
    }

    /// Prepare suppression system for activation
    async fn prepare_for_suppression(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.nozzle_position == NozzlePosition::Retracted {
            info!("🔥 Preparing fire suppression system...");
            
            // Deploy nozzle
            self.nozzle_actuator.deploy().await?;
            self.state.nozzle_position = NozzlePosition::Deployed;
            
            // Log preparation event
            self.log_fire_event(
                FireEventType::SystemActivated,
                "Fire suppression system prepared for activation".to_string()
            );
        }

        Ok(())
    }

    /// Activate fire suppression
    pub async fn activate_suppression(&mut self, emergency: bool) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we're in cooldown period (unless emergency or manual override)
        if !emergency && !self.state.manual_override_active {
            if let Some(last_activation) = self.state.last_activation {
                let elapsed = Utc::now().signed_duration_since(last_activation);
                if elapsed.num_seconds() < self.config.cooldown_period as i64 {
                    warn!("Fire suppression in cooldown period, skipping activation");
                    return Ok(());
                }
            }
        }

        // Check system readiness
        if !self.is_system_ready() {
            error!("Fire suppression system not ready for activation");
            return Err("System not ready".into());
        }

        let activation_type = if emergency { "EMERGENCY" } else { "STANDARD" };
        error!("🔥🚨 {} FIRE SUPPRESSION ACTIVATED 🚨🔥", activation_type);

        // Position nozzle for optimal coverage
        if emergency {
            self.nozzle_actuator.emergency_deploy().await?;
            self.state.nozzle_position = NozzlePosition::Emergency;
        } else {
            self.nozzle_actuator.target_fire().await?;
            self.state.nozzle_position = NozzlePosition::Targeting;
        }

        // Open extinguisher valve
        self.extinguisher_valve.open().await?;
        self.state.discharge_active = true;
        self.state.last_activation = Some(Utc::now());
        self.state.total_activations += 1;

        // Log suppression event
        self.log_fire_event(
            FireEventType::SystemActivated,
            format!("{} fire suppression activated", activation_type)
        );

        // Schedule automatic stop after max duration
        let max_duration = Duration::from_secs(self.config.max_discharge_duration as u64);
        tokio::spawn({
            let valve = self.extinguisher_valve.clone();
            async move {
                tokio::time::sleep(max_duration).await;
                if let Err(e) = valve.close().await {
                    error!("Failed to auto-stop extinguisher: {}", e);
                }
            }
        });

        info!("Fire suppression will auto-stop in {} seconds", self.config.max_discharge_duration);
        Ok(())
    }

    /// Manual activation override
    pub async fn manual_activate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        warn!("🔥 Manual fire suppression override activated");
        
        self.state.manual_override_active = true;
        self.activate_suppression(false).await?;
        
        self.log_fire_event(
            FireEventType::ManualOverride,
            "Manual fire suppression override activated".to_string()
        );

        Ok(())
    }

    /// Stop fire suppression discharge
    pub async fn stop_discharge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.state.discharge_active {
            info!("🛑 Stopping fire suppression discharge");
            
            self.extinguisher_valve.close().await?;
            self.state.discharge_active = false;
            self.state.manual_override_active = false;
            
            // Retract nozzle after suppression
            tokio::time::sleep(Duration::from_secs(2)).await;
            self.nozzle_actuator.retract().await?;
            self.state.nozzle_position = NozzlePosition::Retracted;
            
            self.log_fire_event(
                FireEventType::FireSuppressed,
                "Fire suppression discharge stopped".to_string()
            );
        }

        Ok(())
    }

    /// Check if system is ready for activation
    fn is_system_ready(&self) -> bool {
        self.state.system_armed &&
        self.state.extinguisher_pressure >= self.config.min_pressure &&
        self.state.extinguisher_capacity > 5.0 && // At least 5% capacity
        self.state.system_health != SystemHealth::Offline
    }

    /// Update system health based on current status
    fn update_system_health(&mut self) {
        if self.state.extinguisher_pressure < self.config.min_pressure {
            self.state.system_health = SystemHealth::Critical;
        } else if self.state.extinguisher_capacity < 20.0 {
            self.state.system_health = SystemHealth::Degraded;
        } else {
            self.state.system_health = SystemHealth::Optimal;
        }
    }

    /// Log fire-related event
    fn log_fire_event(&mut self, event_type: FireEventType, description: String) {
        let event = FireEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            temperature: self.state.current_temperature,
            smoke_level: self.state.smoke_level,
            location_estimate: None, // Would be calculated from sensors
            severity: self.assess_fire_risk(),
            response_actions: vec![description],
        };

        self.event_history.push(event);
        
        // Keep only recent events
        if self.event_history.len() > 100 {
            self.event_history.drain(0..10);
        }
    }

    /// Get current system status
    pub fn get_status(&self) -> &FireSuppressionState {
        &self.state
    }

    /// Get system status summary
    pub fn status_summary(&self) -> String {
        let health_emoji = match self.state.system_health {
            SystemHealth::Optimal => "✅",
            SystemHealth::Degraded => "⚠️",
            SystemHealth::Critical => "🔴",
            SystemHealth::Offline => "💀",
        };

        let status_emoji = if self.state.discharge_active {
            "🔥🚨"
        } else if self.state.nozzle_position != NozzlePosition::Retracted {
            "⚡"
        } else {
            "🛡️"
        };

        format!(
            "{} Fire Suppression {} | Health: {} | Pressure: {:.0} PSI | Capacity: {:.0}% | Temp: {:.1}°C | Smoke: {:.1}%",
            status_emoji,
            self.state.nozzle_position.description(),
            health_emoji,
            self.state.extinguisher_pressure,
            self.state.extinguisher_capacity,
            self.state.current_temperature,
            self.state.smoke_level * 100.0
        )
    }

    /// Emergency system test
    pub async fn system_test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧪 Starting fire suppression system test...");

        // Test nozzle deployment
        self.nozzle_actuator.deploy().await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        // Test pressure check
        let pressure = self.extinguisher_valve.read_pressure().await?;
        info!("Extinguisher pressure: {:.1} PSI", pressure);
        
        // Test sensors
        let temp = self.temperature_sensor.read_temperature().await?;
        let smoke = self.smoke_detector.read_smoke_level().await?;
        info!("Temperature: {:.1}°C, Smoke: {:.1}%", temp, smoke * 100.0);

        // Retract nozzle
        self.nozzle_actuator.retract().await?;
        
        info!("✅ Fire suppression system test completed");
        Ok(())
    }
}

// Hardware interface placeholders
#[derive(Clone)]
struct TemperatureSensor;

impl TemperatureSensor {
    fn new() -> Self { Self }
    
    async fn read_temperature(&self) -> Result<f32, Box<dyn std::error::Error>> {
        // Placeholder - would read from actual thermal sensor
        Ok(22.0 + (rand::random::<f32>() * 5.0)) // Simulated room temp + noise
    }
}

#[derive(Clone)]
struct SmokeDetector;

impl SmokeDetector {
    fn new() -> Self { Self }
    
    async fn read_smoke_level(&self) -> Result<f32, Box<dyn std::error::Error>> {
        // Placeholder - would read from actual smoke sensor
        Ok(rand::random::<f32>() * 0.1) // Low random smoke levels
    }
}

#[derive(Clone)]
struct ExtinguisherValve;

impl ExtinguisherValve {
    fn new() -> Self { Self }
    
    async fn open(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("💨 Extinguisher valve OPENED - CO₂ discharge active");
        Ok(())
    }
    
    async fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🛑 Extinguisher valve CLOSED - discharge stopped");
        Ok(())
    }
    
    async fn read_pressure(&self) -> Result<f32, Box<dyn std::error::Error>> {
        // Placeholder - would read from pressure sensor
        Ok(145.0 + (rand::random::<f32>() * 10.0)) // Simulated pressure
    }
}

struct NozzleActuator;

impl NozzleActuator {
    fn new() -> Self { Self }
    
    async fn deploy(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔧 Fire suppression nozzle deployed");
        Ok(())
    }
    
    async fn retract(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔧 Fire suppression nozzle retracted");
        Ok(())
    }
    
    async fn target_fire(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🎯 Nozzle targeting fire source");
        Ok(())
    }
    
    async fn emergency_deploy(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚨 Emergency nozzle deployment - maximum coverage");
        Ok(())
    }
}
