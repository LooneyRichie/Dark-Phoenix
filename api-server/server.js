#!/usr/bin/env node

/**
 * 🔥 Dark Phoenix Cloud API Server 🔥
 * Emergency Services Integration & Real-time Drone Communication
 * 
 * "From the digital realm, the Phoenix coordinates protection."
 */

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const cors = require('cors');
const helmet = require('helmet');
const winston = require('winston');
const { v4: uuidv4 } = require('uuid');
require('dotenv').config();

// Security and rate limiting
const { RateLimiterMemory } = require('rate-limiter-flexible');

// Configure logging
const logger = winston.createLogger({
    level: 'info',
    format: winston.format.combine(
        winston.format.timestamp(),
        winston.format.errors({ stack: true }),
        winston.format.json()
    ),
    defaultMeta: { service: 'dark-phoenix-api' },
    transports: [
        new winston.transports.File({ filename: 'logs/error.log', level: 'error' }),
        new winston.transports.File({ filename: 'logs/combined.log' }),
        new winston.transports.Console({
            format: winston.format.combine(
                winston.format.colorize(),
                winston.format.simple()
            )
        })
    ]
});

// Initialize Express app
const app = express();
const server = http.createServer(app);
const io = socketIo(server, {
    cors: {
        origin: process.env.ALLOWED_ORIGINS?.split(',') || ["http://localhost:3000"],
        methods: ["GET", "POST"]
    }
});

// Rate limiting
const rateLimiter = new RateLimiterMemory({
    keyName: 'ip',
    points: 100, // Number of requests
    duration: 60, // Per 60 seconds
});

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json({ limit: '10mb' }));
app.use(express.urlencoded({ extended: true }));

// Rate limiting middleware
app.use(async (req, res, next) => {
    try {
        await rateLimiter.consume(req.ip);
        next();
    } catch (rejRes) {
        res.status(429).json({
            error: 'Too Many Requests',
            message: 'Rate limit exceeded'
        });
    }
});

// In-memory storage for demo (use database in production)
const droneRegistry = new Map();
const emergencyContacts = new Map();
const threatAlerts = [];
const missionLogs = [];

// Threat level definitions
const ThreatLevel = {
    GREEN: 0,
    YELLOW: 1,
    ORANGE: 2,
    RED: 3,
    OMEGA: 4
};

const ThreatLevelNames = {
    0: 'GREEN',
    1: 'YELLOW', 
    2: 'ORANGE',
    3: 'RED',
    4: 'OMEGA'
};

/**
 * 🚨 EMERGENCY SERVICES INTEGRATION
 */

// Emergency contact simulation (in real deployment, integrate with 911 systems)
class EmergencyDispatch {
    static async contact911(alertData) {
        const emergencyCall = {
            id: uuidv4(),
            timestamp: new Date().toISOString(),
            type: 'SECURITY_THREAT',
            priority: alertData.threatLevel >= ThreatLevel.RED ? 'CRITICAL' : 'HIGH',
            location: alertData.location,
            description: alertData.description,
            droneId: alertData.droneId,
            videoFeedUrl: alertData.videoFeedUrl,
            contactInfo: alertData.contactInfo,
            status: 'DISPATCHED'
        };

        logger.warn('🚨 EMERGENCY SERVICES CONTACTED', emergencyCall);
        
        // In production: integrate with real 911 API
        // await fetch('https://emergency-api.gov/dispatch', { ... });
        
        // Simulate emergency response delay
        setTimeout(() => {
            io.emit('emergency_response', {
                ...emergencyCall,
                status: 'UNITS_DISPATCHED',
                estimatedArrival: '5-8 minutes'
            });
        }, 2000);

        return emergencyCall;
    }

    static async contactMedical(medicalData) {
        const medicalCall = {
            id: uuidv4(),
            timestamp: new Date().toISOString(),
            type: 'MEDICAL_EMERGENCY',
            priority: 'CRITICAL',
            location: medicalData.location,
            vitals: medicalData.vitals,
            injuries: medicalData.injuries,
            droneId: medicalData.droneId,
            status: 'EMS_DISPATCHED'
        };

        logger.error('🚑 EMS CONTACTED', medicalCall);
        return medicalCall;
    }
}

/**
 * 🛡️ DRONE REGISTRATION & MANAGEMENT
 */

app.post('/api/drone/register', (req, res) => {
    try {
        const { droneId, name, capabilities, location, operatorInfo } = req.body;
        
        const registration = {
            id: droneId || uuidv4(),
            name: name || 'Dark Phoenix Unit',
            capabilities: capabilities || [],
            location: location || { lat: 0, lng: 0, alt: 0 },
            operatorInfo,
            status: 'ACTIVE',
            registeredAt: new Date().toISOString(),
            lastHeartbeat: new Date().toISOString(),
            threatLevel: ThreatLevel.GREEN,
            missionStatus: 'GUARDIAN_WATCH'
        };

        droneRegistry.set(registration.id, registration);
        
        logger.info(`🔥 Drone registered: ${registration.name} (${registration.id})`);
        
        // Broadcast new drone online
        io.emit('drone_online', registration);
        
        res.status(201).json({
            success: true,
            drone: registration,
            message: 'Dark Phoenix unit registered successfully'
        });
        
    } catch (error) {
        logger.error('Drone registration failed:', error);
        res.status(500).json({ error: 'Registration failed' });
    }
});

/**
 * 🚨 THREAT ALERT SYSTEM
 */

app.post('/api/threat/alert', async (req, res) => {
    try {
        const { droneId, threatLevel, threatTypes, location, description, evidence, confidence } = req.body;
        
        const alert = {
            id: uuidv4(),
            droneId,
            timestamp: new Date().toISOString(),
            threatLevel: parseInt(threatLevel),
            threatLevelName: ThreatLevelNames[threatLevel],
            threatTypes: threatTypes || [],
            location,
            description,
            evidence,
            confidence: parseFloat(confidence) || 0.0,
            status: 'ACTIVE',
            responseActions: []
        };

        threatAlerts.push(alert);
        
        // Update drone status
        if (droneRegistry.has(droneId)) {
            const drone = droneRegistry.get(droneId);
            drone.threatLevel = alert.threatLevel;
            drone.lastThreat = alert.timestamp;
            droneRegistry.set(droneId, drone);
        }

        logger.warn(`🚨 THREAT ALERT: ${alert.threatLevelName} - ${description}`);
        
        // Broadcast threat alert to all connected clients
        io.emit('threat_alert', alert);
        
        // Auto-contact emergency services for high threats
        if (alert.threatLevel >= ThreatLevel.RED) {
            const emergencyCall = await EmergencyDispatch.contact911({
                ...alert,
                videoFeedUrl: `https://phoenix-stream.com/drone/${droneId}/live`,
                contactInfo: req.body.contactInfo
            });
            
            alert.emergencyCallId = emergencyCall.id;
            alert.responseActions.push('Emergency services contacted');
        }
        
        res.status(200).json({
            success: true,
            alert,
            message: `Threat alert processed - Level ${alert.threatLevelName}`
        });
        
    } catch (error) {
        logger.error('Threat alert processing failed:', error);
        res.status(500).json({ error: 'Alert processing failed' });
    }
});

/**
 * 📍 REAL-TIME POSITION UPDATES
 */

app.post('/api/drone/:droneId/position', (req, res) => {
    try {
        const { droneId } = req.params;
        const { location, status, systemHealth } = req.body;
        
        if (droneRegistry.has(droneId)) {
            const drone = droneRegistry.get(droneId);
            drone.location = location;
            drone.status = status || drone.status;
            drone.systemHealth = systemHealth;
            drone.lastHeartbeat = new Date().toISOString();
            droneRegistry.set(droneId, drone);
            
            // Broadcast position update
            io.emit('drone_position', {
                droneId,
                location,
                status,
                systemHealth,
                timestamp: drone.lastHeartbeat
            });
            
            res.json({ success: true, message: 'Position updated' });
        } else {
            res.status(404).json({ error: 'Drone not found' });
        }
        
    } catch (error) {
        logger.error('Position update failed:', error);
        res.status(500).json({ error: 'Position update failed' });
    }
});

/**
 * 📊 STATUS & MONITORING
 */

app.get('/api/status', (req, res) => {
    const activeDrones = Array.from(droneRegistry.values());
    const activeThreats = threatAlerts.filter(alert => alert.status === 'ACTIVE');
    
    res.json({
        timestamp: new Date().toISOString(),
        system: 'Dark Phoenix Command Center',
        status: 'OPERATIONAL',
        activeDrones: activeDrones.length,
        activeThreats: activeThreats.length,
        totalAlerts: threatAlerts.length,
        drones: activeDrones,
        recentThreats: activeThreats.slice(-10)
    });
});

app.get('/api/drone/:droneId/status', (req, res) => {
    const { droneId } = req.params;
    
    if (droneRegistry.has(droneId)) {
        const drone = droneRegistry.get(droneId);
        const droneThreats = threatAlerts.filter(alert => alert.droneId === droneId);
        
        res.json({
            drone,
            recentThreats: droneThreats.slice(-5),
            totalThreats: droneThreats.length
        });
    } else {
        res.status(404).json({ error: 'Drone not found' });
    }
});

/**
 * 🔗 WEBSOCKET REAL-TIME COMMUNICATION
 */

io.on('connection', (socket) => {
    logger.info(`🔌 Client connected: ${socket.id}`);
    
    // Send current system status on connection
    socket.emit('system_status', {
        activeDrones: Array.from(droneRegistry.values()),
        recentThreats: threatAlerts.slice(-10)
    });
    
    // Handle drone heartbeat
    socket.on('drone_heartbeat', (data) => {
        const { droneId, status } = data;
        if (droneRegistry.has(droneId)) {
            const drone = droneRegistry.get(droneId);
            drone.lastHeartbeat = new Date().toISOString();
            drone.status = status;
            droneRegistry.set(droneId, drone);
        }
    });
    
    // Handle emergency override
    socket.on('emergency_override', async (data) => {
        logger.error(`🚨 EMERGENCY OVERRIDE: ${data.reason}`);
        
        const emergencyCall = await EmergencyDispatch.contact911({
            threatLevel: ThreatLevel.OMEGA,
            description: `MANUAL EMERGENCY OVERRIDE: ${data.reason}`,
            location: data.location,
            droneId: data.droneId,
            contactInfo: data.contactInfo
        });
        
        io.emit('emergency_override', {
            ...data,
            emergencyCallId: emergencyCall.id,
            timestamp: new Date().toISOString()
        });
    });
    
    socket.on('disconnect', () => {
        logger.info(`🔌 Client disconnected: ${socket.id}`);
    });
});

/**
 * 🌐 SERVER STARTUP
 */

const PORT = process.env.PORT || 3001;

server.listen(PORT, () => {
    console.log(`
🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥

🔥 DARK PHOENIX COMMAND CENTER ONLINE 🔥
⚡ Cloud API & Emergency Services Integration ⚡

🌐 Server running on port ${PORT}
🛡️  Emergency dispatch ready
📡 Real-time communication active
🚨 Threat monitoring enabled

"From the cloud, the Phoenix coordinates all protection."

🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥
    `);
    
    logger.info(`Dark Phoenix API Server started on port ${PORT}`);
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\n🛑 Shutting down Dark Phoenix Command Center...');
    server.close(() => {
        logger.info('🔥 Dark Phoenix API Server shutdown complete');
        process.exit(0);
    });
});

module.exports = app;
