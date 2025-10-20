/**
 * 🔥 Dark Phoenix Navigation System 🔥
 * Autonomous navigation, pathfinding, and collision avoidance
 */

#pragma once
#include "common_types.hpp"
#include <memory>
#include <vector>

namespace DarkPhoenix {

/**
 * Navigation system for autonomous flight
 */
class NavigationSystem {
public:
    NavigationSystem();
    ~NavigationSystem();
    
    /**
     * Initialize navigation system
     */
    bool initialize();
    
    /**
     * Update navigation based on current position and mission
     */
    void update_navigation();
    
    /**
     * Get current navigation status
     */
    NavigationStatus get_status() const;
    
    /**
     * Threat response positioning
     */
    void move_to_protective_position(const Position& threat_location);
    void move_to_intercept_position(const Position& threat_location);
    void move_to_protection_position(const Position& threat_location);
    void execute_omega_maneuver(const Position& threat_location);
    
    /**
     * Collision avoidance
     */
    bool detect_collision_risk();
    void execute_avoidance_maneuver();
    
    /**
     * Set navigation waypoints
     */
    void set_waypoints(const std::vector<Position>& waypoints);
    void add_waypoint(const Position& waypoint);
    void clear_waypoints();
    
    /**
     * Set protected area/target
     */
    void set_protected_target(const Position& target, double protection_radius);

private:
    class Impl;
    std::unique_ptr<Impl> pimpl_;
};

} // namespace DarkPhoenix
