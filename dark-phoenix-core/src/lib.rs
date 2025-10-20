use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Core threat level classification system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    /// No threats detected - all systems nominal
    Green = 0,
    /// Minor anomaly - increased awareness mode
    Yellow = 1,
    /// Moderate threat - defensive measures activated
    Orange = 2,
    /// High threat - all deterrence systems online
    Red = 3,
    /// Critical threat - lethal force authorized, maximum protection
    Omega = 4,
}

impl ThreatLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThreatLevel::Green => "GREEN",
            ThreatLevel::Yellow => "YELLOW",
            ThreatLevel::Orange => "ORANGE", 
            ThreatLevel::Red => "RED",
            ThreatLevel::Omega => "OMEGA",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ThreatLevel::Green => "All systems nominal. Guardian mode active.",
            ThreatLevel::Yellow => "Anomaly detected. Heightened awareness engaged.",
            ThreatLevel::Orange => "Moderate threat identified. Defensive protocols online.",
            ThreatLevel::Red => "High threat confirmed. All deterrence systems activated.",
            ThreatLevel::Omega => "Critical threat. Dark Phoenix rising. Maximum protection authorized.",
        }
    }
}

/// Position and movement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub timestamp: DateTime<Utc>,
}

/// Vitals and health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSigns {
    pub heart_rate: Option<u16>,
    pub blood_oxygen: Option<u8>,
    pub temperature: Option<f32>,
    pub stress_level: Option<u8>, // 0-100
    pub timestamp: DateTime<Utc>,
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub battery_level: u8,
    pub flight_time_remaining: u32, // seconds
    pub shield_integrity: u8,       // 0-100%
    pub fire_suppression_ready: bool,
    pub medical_supplies: u8,       // 0-100%
    pub communication_status: bool,
    pub gps_lock: bool,
    pub timestamp: DateTime<Utc>,
}

/// Central command state for the Dark Phoenix drone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneState {
    pub id: Uuid,
    pub name: String,
    pub threat_level: ThreatLevel,
    pub position: Position,
    pub target_vitals: Option<VitalSigns>,
    pub system_health: SystemHealth,
    pub active_modules: HashMap<String, bool>,
    pub mission_log: Vec<MissionEvent>,
    pub last_update: DateTime<Utc>,
}

/// Mission event logging for ceremonial record-keeping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub description: String,
    pub threat_level: ThreatLevel,
    pub position: Position,
    pub response_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    ThreatDetected,
    TerrenceActivated,
    PoliceContacted,
    ShieldDeployed,
    FireSuppressed,
    MedicalAidDeployed,
    HackingAttempt,
    SystemMalfunction,
    MissionComplete,
    PhoenixRising, // Special ceremonial event
}

impl DroneState {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            threat_level: ThreatLevel::Green,
            position: Position {
                latitude: 0.0,
                longitude: 0.0,
                altitude: 0.0,
                timestamp: Utc::now(),
            },
            target_vitals: None,
            system_health: SystemHealth {
                battery_level: 100,
                flight_time_remaining: 3600, // 1 hour
                shield_integrity: 100,
                fire_suppression_ready: true,
                medical_supplies: 100,
                communication_status: true,
                gps_lock: true,
                timestamp: Utc::now(),
            },
            active_modules: HashMap::new(),
            mission_log: Vec::new(),
            last_update: Utc::now(),
        }
    }

    /// Log a mission event with ceremonial significance
    pub fn log_event(&mut self, event_type: EventType, description: String, response_actions: Vec<String>) {
        let event = MissionEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            description,
            threat_level: self.threat_level,
            position: self.position.clone(),
            response_actions,
        };
        
        self.mission_log.push(event);
        self.last_update = Utc::now();
    }

    /// Escalate threat level with proper ceremonial protocol
    pub fn escalate_threat(&mut self, new_level: ThreatLevel, reason: String) {
        if new_level > self.threat_level {
            self.threat_level = new_level;
            self.log_event(
                EventType::ThreatDetected,
                format!("Threat level escalated to {}: {}", new_level.as_str(), reason),
                vec![format!("Threat assessment: {}", new_level.description())],
            );
        }
    }

    /// Check if the drone is in a critical state requiring immediate intervention
    pub fn is_critical(&self) -> bool {
        self.threat_level >= ThreatLevel::Red || 
        self.system_health.battery_level < 20 ||
        !self.system_health.communication_status ||
        self.system_health.shield_integrity < 50
    }

    /// Generate mythic status report
    pub fn mythic_status(&self) -> String {
        let status_emoji = match self.threat_level {
            ThreatLevel::Green => "🕊️",
            ThreatLevel::Yellow => "⚠️",
            ThreatLevel::Orange => "🔥",
            ThreatLevel::Red => "⚔️",
            ThreatLevel::Omega => "🔥💀🔥",
        };

        format!(
            "{} Dark Phoenix {} - Status: {} {}\nBattery: {}% | Shield: {}% | Flight Time: {}min\n{}",
            status_emoji,
            self.name,
            self.threat_level.as_str(),
            status_emoji,
            self.system_health.battery_level,
            self.system_health.shield_integrity,
            self.system_health.flight_time_remaining / 60,
            self.threat_level.description()
        )
    }
}
