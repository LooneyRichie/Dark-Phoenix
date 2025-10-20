/**
 * 🔥 Dark Phoenix Flight Controller Interface 🔥
 * Real-time flight control and hardware interface
 */

#pragma once
#include "common_types.hpp"
#include <memory>
#include <vector>

namespace DarkPhoenix {

/**
 * Main flight controller interface
 * Handles low-level flight control, sensor data, and hardware communication
 */
class FlightController {
public:
    FlightController();
    ~FlightController();
    
    /**
     * Initialize flight controller hardware
     */
    bool initialize();
    
    /**
     * Read current sensor data
     */
    SensorData read_sensors();
    
    /**
     * Get current flight status
     */
    FlightStatus get_status() const;
    
    /**
     * Execute flight commands
     */
    bool execute_commands(const FlightCommands& commands);
    
    /**
     * Emergency landing procedure
     */
    void emergency_land();
    
    /**
     * Adjust for degraded system performance
     */
    void adjust_for_degraded_mode();
    
    /**
     * Arm/disarm the drone
     */
    bool arm();
    bool disarm();
    
    /**
     * Takeoff to specified altitude
     */
    bool takeoff(double target_altitude);
    
    /**
     * Land at current position
     */
    bool land();

private:
    class Impl;
    std::unique_ptr<Impl> pimpl_;
};

} // namespace DarkPhoenix
