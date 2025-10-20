/**
 * Dark Phoenix Cloud API Server
 * 
 * Node.js/Express server for:
 * - Emergency services integration (911, police, EMS)
 * - Real-time drone communication and telemetry
 * - Cloud storage and incident logging
 * - User authentication and fleet management
 * - Web dashboard and mobile app API
 */

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const cors = require('cors');
const helmet = require('helmet');
const compression = require('compression');
const morgan = require('morgan');
require('dotenv').config();

const emergencyRoutes = require('./routes/emergency');
const droneRoutes = require('./routes/drone');
const authRoutes = require('./routes/auth');
const incidentRoutes = require('./routes/incidents');
const telemetryRoutes = require('./routes/telemetry');

const EmergencyService = require('./services/EmergencyService');
const DroneManager = require('./services/DroneManager');
const TelemetryProcessor = require('./services/TelemetryProcessor');

class DarkPhoenixCloudAPI {
    constructor() {
        this.app = express();
        this.server = http.createServer(this.app);
        this.io = socketIo(this.server, {
            cors: {
                origin: process.env.ALLOWED_ORIGINS?.split(',') || ["http://localhost:3000"],
                methods: ["GET", "POST"]
            }
        });
        
        this.port = process.env.PORT || 3000;
        
        // Initialize services
        this.emergencyService = new EmergencyService();
        this.droneManager = new DroneManager();
        this.telemetryProcessor = new TelemetryProcessor();
        
        this.setupMiddleware();
        this.setupRoutes();
        this.setupSocketHandlers();
        this.setupErrorHandling();
    }

    setupMiddleware() {
        // Security middleware
        this.app.use(helmet({
            contentSecurityPolicy: {
                directives: {
                    defaultSrc: ["'self'"],
                    scriptSrc: ["'self'", "'unsafe-inline'"],
                    styleSrc: ["'self'", "'unsafe-inline'"],
                    imgSrc: ["'self'", "data:", "https:"],
                }
            }
        }));
        
        // CORS configuration
        this.app.use(cors({
            origin: process.env.ALLOWED_ORIGINS?.split(',') || ["http://localhost:3000"],
            credentials: true
        }));
        
        // Compression and logging
        this.app.use(compression());
        this.app.use(morgan('combined'));
        
        // Body parsing
        this.app.use(express.json({ limit: '10mb' }));
        this.app.use(express.urlencoded({ extended: true }));
        
        // Rate limiting for API protection
        if (process.env.NODE_ENV === 'production') {
            const rateLimit = require('express-rate-limit');
            this.app.use(rateLimit({
                windowMs: 15 * 60 * 1000, // 15 minutes
                max: 100 // limit each IP to 100 requests per windowMs
            }));
        }
    }

    setupRoutes() {
        // Health check
        this.app.get('/health', (req, res) => {
            res.json({
                status: 'online',
                timestamp: new Date().toISOString(),
                service: 'Dark Phoenix Cloud API',
                version: '1.0.0',
                uptime: process.uptime()
            });
        });

        // API routes
        this.app.use('/api/emergency', emergencyRoutes);
        this.app.use('/api/drones', droneRoutes);
        this.app.use('/api/auth', authRoutes);
        this.app.use('/api/incidents', incidentRoutes);
        this.app.use('/api/telemetry', telemetryRoutes);

        // Serve static files for web dashboard
        this.app.use(express.static('public'));

        // Catch-all for SPA routing
        this.app.get('*', (req, res) => {
            res.sendFile(path.join(__dirname, 'public', 'index.html'));
        });
    }

    setupSocketHandlers() {
        this.io.on('connection', (socket) => {
            console.log(`🔗 Client connected: ${socket.id}`);

            // Drone registration
            socket.on('drone:register', async (droneData) => {
                try {
                    await this.droneManager.registerDrone(socket.id, droneData);
                    socket.emit('drone:registered', { 
                        status: 'success', 
                        droneId: droneData.id 
                    });
                    console.log(`🤖 Drone registered: ${droneData.name} (${droneData.id})`);
                } catch (error) {
                    socket.emit('drone:error', { error: error.message });
                }
            });

            // Real-time telemetry updates
            socket.on('telemetry:update', async (telemetryData) => {
                try {
                    await this.telemetryProcessor.processTelemetry(telemetryData);
                    
                    // Broadcast to monitoring clients
                    socket.broadcast.emit('telemetry:live', telemetryData);
                    
                    // Check for emergency conditions
                    if (telemetryData.threatLevel === 'OMEGA' || telemetryData.threatLevel === 'RED') {
                        await this.handleEmergencyTelemetry(telemetryData);
                    }
                } catch (error) {
                    console.error('Telemetry processing error:', error);
                }
            });

            // Emergency alert from drone
            socket.on('emergency:alert', async (alertData) => {
                try {
                    console.log(`🚨 EMERGENCY ALERT from ${alertData.droneId}`);
                    const response = await this.emergencyService.handleEmergencyAlert(alertData);
                    
                    // Confirm emergency services contacted
                    socket.emit('emergency:confirmed', response);
                    
                    // Broadcast to all monitoring clients
                    this.io.emit('emergency:broadcast', {
                        ...alertData,
                        timestamp: new Date().toISOString(),
                        status: 'EMERGENCY_SERVICES_CONTACTED'
                    });
                } catch (error) {
                    socket.emit('emergency:error', { error: error.message });
                }
            });

            // Command from control center to drone
            socket.on('drone:command', async (commandData) => {
                try {
                    const targetSocket = await this.droneManager.getDroneSocket(commandData.droneId);
                    if (targetSocket) {
                        targetSocket.emit('command:execute', commandData);
                    } else {
                        socket.emit('drone:error', { error: 'Drone not connected' });
                    }
                } catch (error) {
                    socket.emit('drone:error', { error: error.message });
                }
            });

            // Client disconnection
            socket.on('disconnect', async () => {
                console.log(`❌ Client disconnected: ${socket.id}`);
                await this.droneManager.unregisterDrone(socket.id);
            });
        });
    }

    async handleEmergencyTelemetry(telemetryData) {
        const emergencyAlert = {
            droneId: telemetryData.droneId,
            threatLevel: telemetryData.threatLevel,
            location: {
                latitude: telemetryData.position.latitude,
                longitude: telemetryData.position.longitude
            },
            timestamp: new Date().toISOString(),
            description: `${telemetryData.threatLevel} threat detected by Dark Phoenix ${telemetryData.droneName}`,
            threats: telemetryData.threats || [],
            systemHealth: telemetryData.systemHealth
        };

        // Auto-trigger emergency services if OMEGA level
        if (telemetryData.threatLevel === 'OMEGA') {
            await this.emergencyService.autoDispatch911(emergencyAlert);
        }
    }

    setupErrorHandling() {
        // Global error handler
        this.app.use((error, req, res, next) => {
            console.error('API Error:', error);
            
            res.status(error.status || 500).json({
                error: {
                    message: error.message || 'Internal server error',
                    status: error.status || 500,
                    timestamp: new Date().toISOString()
                }
            });
        });

        // 404 handler
        this.app.use((req, res) => {
            res.status(404).json({
                error: {
                    message: 'Endpoint not found',
                    status: 404,
                    path: req.path
                }
            });
        });
    }

    async start() {
        try {
            // Initialize database connections
            // await this.connectDatabase();
            
            this.server.listen(this.port, () => {
                console.log(`
🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥
               
    ██████╗  █████╗ ██████╗ ██╗  ██╗    
    ██╔══██╗██╔══██╗██╔══██╗██║ ██╔╝    
    ██║  ██║███████║██████╔╝█████╔╝     
    ██║  ██║██╔══██║██╔══██╗██╔═██╗     
    ██████╔╝██║  ██║██║  ██║██║  ██╗    
    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝    
    
    ██████╗ ██╗  ██╗ ██████╗ ███████╗███╗   ██╗██╗██╗  ██╗
    ██╔══██╗██║  ██║██╔═══██╗██╔════╝████╗  ██║██║╚██╗██╔╝
    ██████╔╝███████║██║   ██║█████╗  ██╔██╗ ██║██║ ╚███╔╝ 
    ██╔═══╝ ██╔══██║██║   ██║██╔══╝  ██║╚██╗██║██║ ██╔██╗ 
    ██║     ██║  ██║╚██████╔╝███████╗██║ ╚████║██║██╔╝ ██╗
    ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝
    
    CLOUD API & EMERGENCY SERVICES
    "Guardian in the Cloud, Protector of the Innocent"
    
🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥

🌐 Dark Phoenix Cloud API Server Online
📡 Port: ${this.port}
🚨 Emergency Services: Ready
📊 Real-time Telemetry: Active
🔐 Security: Enabled
⚡ Socket.IO: Connected

Ready to protect and serve...
                `);
            });

        } catch (error) {
            console.error('❌ Failed to start Dark Phoenix Cloud API:', error);
            process.exit(1);
        }
    }

    async stop() {
        console.log('🛑 Shutting down Dark Phoenix Cloud API...');
        this.server.close();
        process.exit(0);
    }
}

// Graceful shutdown handling
process.on('SIGINT', async () => {
    console.log('\n🛑 Received SIGINT, shutting down gracefully...');
    process.exit(0);
});

process.on('SIGTERM', async () => {
    console.log('\n🛑 Received SIGTERM, shutting down gracefully...');
    process.exit(0);
});

// Start the server
const api = new DarkPhoenixCloudAPI();
api.start();

module.exports = DarkPhoenixCloudAPI;
