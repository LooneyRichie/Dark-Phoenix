use dark_phoenix_core::ThreatLevel;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::time::Duration;
use tokio::time::{sleep, interval};
use tracing::{info, warn, error};

/// Configuration for deterrence systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterrenceConfig {
    pub max_siren_volume: u8,        // 0-100, maps to actual dB
    pub strobe_frequency_hz: f32,    // Strobe rate
    pub voice_volume: u8,            // Voice broadcast volume
    pub escalation_delay_ms: u64,    // Delay between escalation steps
    pub auto_de_escalate: bool,      // Auto reduce intensity over time
}

impl Default for DeterrenceConfig {
    fn default() -> Self {
        Self {
            max_siren_volume: 85, // 120+ dB capable, but start reasonable
            strobe_frequency_hz: 8.0,
            voice_volume: 75,
            escalation_delay_ms: 2000,
            auto_de_escalate: true,
        }
    }
}

/// Current state of deterrence systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterrenceState {
    pub siren_active: bool,
    pub siren_volume: u8,
    pub strobe_active: bool,
    pub strobe_pattern: StrobePattern,
    pub voice_active: bool,
    pub current_message: Option<String>,
    pub last_activation: Option<DateTime<Utc>>,
    pub activation_count: u32,
}

impl Default for DeterrenceState {
    fn default() -> Self {
        Self {
            siren_active: false,
            siren_volume: 0,
            strobe_active: false,
            strobe_pattern: StrobePattern::Off,
            voice_active: false,
            current_message: None,
            last_activation: None,
            activation_count: 0,
        }
    }
}

/// Strobe light patterns for different threat levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum StrobePattern {
    Off,
    Pulse,          // Gentle pulsing for awareness
    Alert,          // Fast alternating for attention
    Warning,        // Rapid strobing for deterrence
    Emergency,      // Maximum intensity disorientation
    Phoenix,        // Mythic pattern - rising flame effect
}

impl StrobePattern {
    pub fn frequency_hz(&self) -> f32 {
        match self {
            StrobePattern::Off => 0.0,
            StrobePattern::Pulse => 1.0,
            StrobePattern::Alert => 4.0,
            StrobePattern::Warning => 8.0,
            StrobePattern::Emergency => 15.0,
            StrobePattern::Phoenix => 3.0, // Slower, more dramatic
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            StrobePattern::Off => "Strobes disabled",
            StrobePattern::Pulse => "Gentle awareness pulse",
            StrobePattern::Alert => "Alert attention strobe",
            StrobePattern::Warning => "Warning deterrence flash",
            StrobePattern::Emergency => "Emergency disorientation strobe",
            StrobePattern::Phoenix => "Phoenix rising ceremonial pattern",
        }
    }
}

/// Mythic voice messages for different situations
pub struct MythicVoice;

impl MythicVoice {
    /// Get appropriate voice message based on threat level
    pub fn get_message(threat_level: ThreatLevel, situation: &str) -> String {
        match threat_level {
            ThreatLevel::Green => Self::green_messages(),
            ThreatLevel::Yellow => Self::yellow_messages(situation),
            ThreatLevel::Orange => Self::orange_messages(situation),
            ThreatLevel::Red => Self::red_messages(situation),
            ThreatLevel::Omega => Self::omega_messages(),
        }
    }

    fn green_messages() -> String {
        "Guardian protocols active. Area under protection.".to_string()
    }

    fn yellow_messages(situation: &str) -> String {
        match situation {
            "anomaly" => "Anomaly detected. Please maintain calm behavior.".to_string(),
            "proximity" => "You are entering a protected zone. Please identify yourself.".to_string(),
            _ => "Dark Phoenix monitoring. Please proceed with caution.".to_string(),
        }
    }

    fn orange_messages(situation: &str) -> String {
        match situation {
            "aggression" => "Aggressive behavior detected. Cease immediately or authorities will be contacted.".to_string(),
            "weapon" => "Weapon detected. Drop the weapon and step back immediately.".to_string(),
            "group_threat" => "Multiple aggressors detected. Disperse immediately or law enforcement will be summoned.".to_string(),
            _ => "Warning: Threat level elevated. You are being recorded. Authorities have been notified.".to_string(),
        }
    }

    fn red_messages(situation: &str) -> String {
        match situation {
            "imminent_danger" => "IMMINENT DANGER DETECTED. EMERGENCY SERVICES CONTACTED. RETREAT IMMEDIATELY.".to_string(),
            "weapon_drawn" => "WEAPON DRAWN. DROP WEAPON NOW. POLICE EN ROUTE. YOU ARE BEING RECORDED.".to_string(),
            "physical_attack" => "PHYSICAL ATTACK IN PROGRESS. MEDICAL AND POLICE ASSISTANCE REQUESTED.".to_string(),
            _ => "HIGH THREAT CONFIRMED. ALL DETERRENCE SYSTEMS ACTIVE. SURRENDER IMMEDIATELY.".to_string(),
        }
    }

    fn omega_messages() -> String {
        "⚠️ OMEGA PROTOCOL ACTIVATED ⚠️ DARK PHOENIX RISING ⚠️ MAXIMUM PROTECTION AUTHORIZED ⚠️ SURRENDER OR FACE CONSEQUENCES ⚠️".to_string()
    }

    /// Get ceremonial announcement for special occasions
    pub fn ceremonial_announcement(event: &str) -> String {
        match event {
            "activation" => "From the ashes of danger, the Dark Phoenix rises to protect the innocent.".to_string(),
            "victory" => "The Phoenix has prevailed. Peace is restored. Guardian watch continues.".to_string(),
            "retreat" => "Threat neutralized. The Phoenix returns to the shadows, ever watchful.".to_string(),
            _ => "Dark Phoenix stands eternal vigil. None shall harm the protected.".to_string(),
        }
    }
}

/// Main deterrence system controller
pub struct DeterrenceSuite {
    config: DeterrenceConfig,
    state: DeterrenceState,
    // Hardware interfaces (placeholders for now)
    siren_controller: SirenController,
    strobe_controller: StrobeController,
    voice_controller: VoiceController,
}

impl DeterrenceSuite {
    pub fn new(config: DeterrenceConfig) -> Self {
        Self {
            config,
            state: DeterrenceState::default(),
            siren_controller: SirenController::new(),
            strobe_controller: StrobeController::new(),
            voice_controller: VoiceController::new(),
        }
    }

    /// Activate deterrence systems based on threat level
    pub async fn activate(&mut self, threat_level: ThreatLevel, situation: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚨 Activating deterrence systems for threat level: {}", threat_level.as_str());
        
        self.state.last_activation = Some(Utc::now());
        self.state.activation_count += 1;

        match threat_level {
            ThreatLevel::Green => {
                self.deactivate_all().await?;
            },
            ThreatLevel::Yellow => {
                self.activate_low_deterrence(situation).await?;
            },
            ThreatLevel::Orange => {
                self.activate_medium_deterrence(situation).await?;
            },
            ThreatLevel::Red => {
                self.activate_high_deterrence(situation).await?;
            },
            ThreatLevel::Omega => {
                self.activate_omega_protocol().await?;
            },
        }

        Ok(())
    }

    /// Low-level deterrence for Yellow threats
    async fn activate_low_deterrence(&mut self, situation: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Gentle strobe to get attention
        self.strobe_controller.set_pattern(StrobePattern::Pulse).await?;
        self.state.strobe_active = true;
        self.state.strobe_pattern = StrobePattern::Pulse;

        // Calm voice message
        let message = MythicVoice::get_message(ThreatLevel::Yellow, situation);
        self.voice_controller.speak(&message, self.config.voice_volume / 2).await?;
        self.state.voice_active = true;
        self.state.current_message = Some(message);

        info!("🟡 Low deterrence activated: {}", StrobePattern::Pulse.description());
        Ok(())
    }

    /// Medium deterrence for Orange threats
    async fn activate_medium_deterrence(&mut self, situation: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Warning strobe
        self.strobe_controller.set_pattern(StrobePattern::Warning).await?;
        self.state.strobe_active = true;
        self.state.strobe_pattern = StrobePattern::Warning;

        // Low-volume siren
        let siren_volume = self.config.max_siren_volume / 3;
        self.siren_controller.activate(siren_volume).await?;
        self.state.siren_active = true;
        self.state.siren_volume = siren_volume;

        // Authoritative voice message
        let message = MythicVoice::get_message(ThreatLevel::Orange, situation);
        self.voice_controller.speak(&message, self.config.voice_volume).await?;
        self.state.current_message = Some(message);

        warn!("🟠 Medium deterrence activated: Siren {}%, Strobe {}", 
              siren_volume, StrobePattern::Warning.description());
        Ok(())
    }

    /// High deterrence for Red threats
    async fn activate_high_deterrence(&mut self, situation: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Emergency strobe
        self.strobe_controller.set_pattern(StrobePattern::Emergency).await?;
        self.state.strobe_active = true;
        self.state.strobe_pattern = StrobePattern::Emergency;

        // High-volume siren
        let siren_volume = (self.config.max_siren_volume * 2) / 3;
        self.siren_controller.activate(siren_volume).await?;
        self.state.siren_active = true;
        self.state.siren_volume = siren_volume;

        // Commanding voice message
        let message = MythicVoice::get_message(ThreatLevel::Red, situation);
        self.voice_controller.speak(&message, self.config.voice_volume).await?;
        self.state.current_message = Some(message);

        error!("🔴 High deterrence activated: Siren {}%, Emergency strobe", siren_volume);
        Ok(())
    }

    /// Maximum deterrence for Omega threats
    async fn activate_omega_protocol(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        error!("💀 OMEGA PROTOCOL ACTIVATED - DARK PHOENIX RISING 💀");

        // Phoenix ceremonial strobe pattern
        self.strobe_controller.set_pattern(StrobePattern::Phoenix).await?;
        self.state.strobe_active = true;
        self.state.strobe_pattern = StrobePattern::Phoenix;

        // Maximum siren volume
        self.siren_controller.activate(self.config.max_siren_volume).await?;
        self.state.siren_active = true;
        self.state.siren_volume = self.config.max_siren_volume;

        // Omega protocol voice message
        let message = MythicVoice::get_message(ThreatLevel::Omega, "omega");
        self.voice_controller.speak(&message, 100).await?; // Maximum volume
        self.state.current_message = Some(message);

        // Wait, then ceremonial announcement
        sleep(Duration::from_millis(self.config.escalation_delay_ms)).await;
        let ceremonial = MythicVoice::ceremonial_announcement("activation");
        self.voice_controller.speak(&ceremonial, 100).await?;

        error!("🔥 OMEGA PROTOCOL FULLY DEPLOYED 🔥");
        Ok(())
    }

    /// Deactivate all deterrence systems
    pub async fn deactivate_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.siren_controller.deactivate().await?;
        self.strobe_controller.set_pattern(StrobePattern::Off).await?;
        self.voice_controller.stop().await?;

        self.state.siren_active = false;
        self.state.siren_volume = 0;
        self.state.strobe_active = false;
        self.state.strobe_pattern = StrobePattern::Off;
        self.state.voice_active = false;
        self.state.current_message = None;

        info!("🕊️ All deterrence systems deactivated - peaceful mode");
        Ok(())
    }

    /// Get current deterrence status
    pub fn get_status(&self) -> &DeterrenceState {
        &self.state
    }

    /// Emergency test of all systems
    pub async fn system_test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🧪 Starting deterrence system test...");

        // Test each component briefly
        self.voice_controller.speak("System test initiated", 50).await?;
        sleep(Duration::from_millis(1000)).await;

        self.strobe_controller.set_pattern(StrobePattern::Alert).await?;
        sleep(Duration::from_millis(2000)).await;

        self.siren_controller.activate(20).await?; // Low volume test
        sleep(Duration::from_millis(1000)).await;

        self.deactivate_all().await?;
        self.voice_controller.speak("System test complete. All systems operational.", 50).await?;

        info!("✅ Deterrence system test completed successfully");
        Ok(())
    }
}

/// Siren controller (placeholder for hardware interface)
struct SirenController;

impl SirenController {
    fn new() -> Self {
        Self
    }

    async fn activate(&self, volume: u8) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder - would interface with actual siren hardware
        info!("🔊 Siren activated at {}% volume (~{} dB)", volume, 80 + (volume * 40 / 100));
        Ok(())
    }

    async fn deactivate(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔇 Siren deactivated");
        Ok(())
    }
}

/// Strobe light controller (placeholder for hardware interface)
struct StrobeController;

impl StrobeController {
    fn new() -> Self {
        Self
    }

    async fn set_pattern(&self, pattern: StrobePattern) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder - would control LED arrays/strobe hardware
        match pattern {
            StrobePattern::Off => info!("💡 Strobes OFF"),
            StrobePattern::Phoenix => info!("🔥 Phoenix strobe pattern: Rising flames effect"),
            _ => info!("⚡ Strobe pattern: {} at {:.1}Hz", pattern.description(), pattern.frequency_hz()),
        }
        Ok(())
    }
}

/// Voice synthesis controller (placeholder for TTS system)
struct VoiceController;

impl VoiceController {
    fn new() -> Self {
        Self
    }

    async fn speak(&self, message: &str, volume: u8) -> Result<(), Box<dyn std::error::Error>> {
        // Placeholder - would use TTS engine and speaker hardware
        info!("🗣️  Speaking at {}% volume: \"{}\"", volume, message);
        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🤐 Voice system stopped");
        Ok(())
    }
}
