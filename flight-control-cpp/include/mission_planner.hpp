/**
 * 🔥 Dark Phoenix Mission Planner 🔥
 * High-level mission planning and threat response coordination
 */

#pragma once
#include "common_types.hpp"
#include <memory>

namespace DarkPhoenix {

/**
 * Mission planning and execution system
 */
class MissionPlanner {
public:
    MissionPlanner();
    ~MissionPlanner();
    
    /**
     * Set current mission mode
     */
    void set_mission_mode(MissionMode mode);
    
    /**
     * Get current mission mode
     */
    MissionMode get_mission_mode() const;
    
    /**
     * Get commands for current mission state
     */
    FlightCommands get_current_commands();
    
    /**
     * Update mission based on current threat assessment
     */
    void update_mission(ThreatLevel threat_level, const Position& threat_location);
    
    /**
     * Set patrol parameters
     */
    void set_patrol_area(const Position& center, double radius);
    void set_patrol_altitude(double altitude);
    void set_patrol_speed(double speed);

private:
    class Impl;
    std::unique_ptr<Impl> pimpl_;
};

} // namespace DarkPhoenix
