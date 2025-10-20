/**
 * 🔥 Dark Phoenix Safety Monitor 🔥
 * System health monitoring and safety protocols
 */

#pragma once
#include "common_types.hpp"
#include <memory>

namespace DarkPhoenix {

/**
 * Safety monitoring system
 */
class SafetyMonitor {
public:
    SafetyMonitor();
    ~SafetyMonitor();
    
    /**
     * Check overall system health
     */
    SystemHealth check_system_health();
    
    /**
     * Get current health status
     */
    SystemHealth get_health_status() const;
    
    /**
     * Set safety thresholds
     */
    void set_battery_warning_threshold(double percentage);
    void set_communication_timeout(double seconds);
    void set_max_flight_time(double seconds);
    
    /**
     * Emergency safety checks
     */
    bool is_safe_to_fly() const;
    bool requires_immediate_landing() const;

private:
    class Impl;
    std::unique_ptr<Impl> pimpl_;
};

} // namespace DarkPhoenix
