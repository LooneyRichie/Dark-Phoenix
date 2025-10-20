# Dark Phoenix C++ Flight Control System

cmake_minimum_required(VERSION 3.16)
project(DarkPhoenixFlightControl VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find packages
find_package(PkgConfig REQUIRED)

# MAVLink library (for drone communication)
# In real implementation, you'd use: find_package(MAVLink REQUIRED)
set(MAVLINK_INCLUDE_DIR "/usr/include/mavlink/v2.0")

# JSON library for configuration and telemetry
# find_package(nlohmann_json 3.2.0 REQUIRED)  # Commented out for now

# Thread support
find_package(Threads REQUIRED)

# Include directories
include_directories(include)
include_directories(${MAVLINK_INCLUDE_DIR})

# Executable
add_executable(dark_phoenix_flight
    src/flight_systems.cpp
    src/main.cpp
)

# Link libraries
target_link_libraries(dark_phoenix_flight
    # nlohmann_json::nlohmann_json  # Commented out for now
    Threads::Threads
)

# Compiler flags for optimization and safety
target_compile_options(dark_phoenix_flight PRIVATE
    -Wall -Wextra -Wpedantic
    -O3  # High optimization for real-time performance
    -march=native  # Optimize for target hardware
)

# Install rules
install(TARGETS dark_phoenix_flight
    RUNTIME DESTINATION bin
)

# CPack configuration for packaging
set(CPACK_PACKAGE_NAME "DarkPhoenixFlightControl")
set(CPACK_PACKAGE_VERSION ${PROJECT_VERSION})
set(CPACK_PACKAGE_DESCRIPTION "Dark Phoenix C++ Flight Control System")
set(CPACK_PACKAGE_CONTACT "looney@darkphoenix.com")

include(CPack)
