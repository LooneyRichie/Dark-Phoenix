/**
 * 🔥 Dark Phoenix Flight Control Types & Definitions 🔥
 */

#pragma once
#include <cstdint>
#include <string>

namespace DarkPhoenix {

/**
 * Threat level enumeration (matches Rust core definitions)
 */
enum class ThreatLevel : uint8_t {
    GREEN = 0,   // All systems nominal
    YELLOW = 1,  // Anomaly detected
    ORANGE = 2,  // Moderate threat
    RED = 3,     // High threat
    OMEGA = 4    // Critical threat - maximum protection
};

/**
 * Mission modes based on threat level
 */
enum class MissionMode {
    PATROL,              // Normal patrol pattern
    ENHANCED_WATCH,      // Increased alertness
    DEFENSIVE,           // Defensive positioning
    ACTIVE_PROTECTION,   // Active protection mode
    OMEGA_PROTOCOL       // Maximum protection protocol
};

/**
 * 3D Position structure
 */
struct Position {
    double latitude;     // Degrees
    double longitude;    // Degrees  
    double altitude;     // Meters above ground level
};

/**
 * Velocity vector
 */
struct Velocity {
    double x;  // North velocity (m/s)
    double y;  // East velocity (m/s)
    double z;  // Up velocity (m/s)
};

/**
 * Attitude (orientation) structure
 */
struct Attitude {
    double roll;   // Roll angle (radians)
    double pitch;  // Pitch angle (radians)
    double yaw;    // Yaw angle (radians)
};

/**
 * Flight status information
 */
struct FlightStatus {
    bool in_flight;
    bool armed;
    Position position;
    Velocity velocity;
    Attitude attitude;
    double battery_voltage;
    double flight_time_remaining;  // seconds
    ThreatLevel current_threat_level;
    MissionMode current_mission;
};

/**
 * Sensor data from flight controller
 */
struct SensorData {
    // IMU data
    double accel_x, accel_y, accel_z;  // m/s²
    double gyro_x, gyro_y, gyro_z;     // rad/s
    double mag_x, mag_y, mag_z;        // gauss
    
    // GPS data
    Position gps_position;
    double gps_accuracy;  // meters
    
    // Barometric data
    double pressure;      // Pascal
    double temperature;   // Celsius
    
    // Battery data
    double voltage;       // Volts
    double current;       // Amperes
    double remaining;     // Percentage
    
    uint64_t timestamp;   // microseconds since boot
};

/**
 * Flight control commands
 */
struct FlightCommands {
    // Position control
    Position target_position;
    double target_yaw;
    
    // Velocity control
    Velocity target_velocity;
    
    // Control mode
    enum class ControlMode {
        POSITION_HOLD,
        VELOCITY_CONTROL,
        MANUAL_CONTROL,
        AUTO_MISSION,
        EMERGENCY_LAND
    } mode;
    
    // Emergency flags
    bool emergency_stop;
    bool return_to_launch;
};

/**
 * System health status
 */
struct SystemHealth {
    bool gps_healthy;
    bool imu_healthy;
    bool battery_healthy;
    bool communication_healthy;
    bool motors_healthy;
    
    double battery_percentage;
    double cpu_load;
    double memory_usage;
    
    bool critical_failure;
    bool degraded_performance;
    
    std::string status_message;
};

/**
 * Navigation status
 */
struct NavigationStatus {
    Position current_position;
    Position target_position;
    double distance_to_target;
    double bearing_to_target;
    bool waypoint_reached;
    bool collision_risk;
};

/**
 * Telemetry packet for ground station
 */
struct TelemetryPacket {
    FlightStatus flight_status;
    NavigationStatus navigation_status;
    SystemHealth system_health;
    ThreatLevel threat_level;
    uint64_t timestamp;
};

/**
 * Utility functions
 */
inline const char* threat_level_name(ThreatLevel level) {
    switch (level) {
        case ThreatLevel::GREEN: return "GREEN";
        case ThreatLevel::YELLOW: return "YELLOW";
        case ThreatLevel::ORANGE: return "ORANGE";
        case ThreatLevel::RED: return "RED";
        case ThreatLevel::OMEGA: return "OMEGA";
        default: return "UNKNOWN";
    }
}

inline const char* mission_mode_name(MissionMode mode) {
    switch (mode) {
        case MissionMode::PATROL: return "PATROL";
        case MissionMode::ENHANCED_WATCH: return "ENHANCED_WATCH";
        case MissionMode::DEFENSIVE: return "DEFENSIVE";
        case MissionMode::ACTIVE_PROTECTION: return "ACTIVE_PROTECTION";
        case MissionMode::OMEGA_PROTOCOL: return "OMEGA_PROTOCOL";
        default: return "UNKNOWN";
    }
}

} // namespace DarkPhoenix
