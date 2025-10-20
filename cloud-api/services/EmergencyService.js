/**
 * Emergency Service Integration
 * 
 * Handles 911 calls, police contact, EMS dispatch, and emergency coordination
 * for Dark Phoenix security incidents.
 */

const twilio = require('twilio');
const axios = require('axios');
const geolib = require('geolib');

class EmergencyService {
    constructor() {
        // Initialize Twilio for emergency calling (when not in demo mode)
        this.twilioClient = process.env.TWILIO_SID ? 
            twilio(process.env.TWILIO_SID, process.env.TWILIO_TOKEN) : null;
        
        // Emergency service endpoints (demo - would be real integrations)
        this.emergencyEndpoints = {
            police: process.env.POLICE_API_ENDPOINT || 'https://demo-police-api.example.com',
            ems: process.env.EMS_API_ENDPOINT || 'https://demo-ems-api.example.com',
            fire: process.env.FIRE_API_ENDPOINT || 'https://demo-fire-api.example.com'
        };
        
        this.demoMode = process.env.NODE_ENV !== 'production';
    }

    /**
     * Handle emergency alert from Dark Phoenix drone
     */
    async handleEmergencyAlert(alertData) {
        console.log(`🚨 PROCESSING EMERGENCY ALERT 🚨`);
        console.log(`Drone: ${alertData.droneId}`);
        console.log(`Threat Level: ${alertData.threatLevel}`);
        console.log(`Location: ${alertData.location.latitude}, ${alertData.location.longitude}`);

        const response = {
            alertId: `ALERT_${Date.now()}`,
            timestamp: new Date().toISOString(),
            status: 'PROCESSING',
            emergencyServices: [],
            estimatedResponse: null
        };

        try {
            // Determine required emergency services based on threat type
            const requiredServices = this.determineRequiredServices(alertData);
            
            // Contact emergency services
            for (const service of requiredServices) {
                const serviceResponse = await this.contactEmergencyService(service, alertData);
                response.emergencyServices.push(serviceResponse);
            }

            // Make 911 call if critical threat
            if (alertData.threatLevel === 'OMEGA' || alertData.threatLevel === 'RED') {
                const callResponse = await this.make911Call(alertData);
                response.call911 = callResponse;
            }

            // Calculate estimated response time
            response.estimatedResponse = await this.calculateResponseTime(alertData.location);
            response.status = 'EMERGENCY_SERVICES_CONTACTED';

            console.log(`✅ Emergency services contacted successfully`);
            return response;

        } catch (error) {
            console.error('❌ Emergency service contact failed:', error);
            response.status = 'ERROR';
            response.error = error.message;
            return response;
        }
    }

    /**
     * Auto-dispatch 911 for OMEGA level threats
     */
    async autoDispatch911(alertData) {
        console.log(`💀 OMEGA PROTOCOL - AUTO-DISPATCHING 911 💀`);
        
        if (this.demoMode) {
            console.log(`📞 [DEMO] Would call 911 with:`);
            console.log(`   Location: ${alertData.location.latitude}, ${alertData.location.longitude}`);
            console.log(`   Threat: ${alertData.description}`);
            console.log(`   Drone ID: ${alertData.droneId}`);
            return { status: 'DEMO_MODE', message: '911 call simulated' };
        }

        // Real 911 integration would go here
        return await this.make911Call(alertData);
    }

    /**
     * Determine which emergency services are needed
     */
    determineRequiredServices(alertData) {
        const services = [];
        const threats = alertData.threats || [];
        
        // Always contact police for security threats
        if (alertData.threatLevel !== 'GREEN') {
            services.push('police');
        }

        // Check for specific threat types
        threats.forEach(threat => {
            switch (threat.type) {
                case 'weapon_firearm':
                case 'weapon_knife':
                case 'physical_attack':
                    if (!services.includes('police')) services.push('police');
                    services.push('ems'); // Medical may be needed
                    break;
                    
                case 'fire_smoke':
                case 'fire_suppression':
                    services.push('fire');
                    services.push('ems');
                    break;
                    
                case 'medical_emergency':
                case 'blood':
                case 'injury':
                    services.push('ems');
                    break;
                    
                case 'structural_damage':
                case 'explosion':
                    services.push('fire');
                    services.push('police');
                    services.push('ems');
                    break;
            }
        });

        return [...new Set(services)]; // Remove duplicates
    }

    /**
     * Contact specific emergency service
     */
    async contactEmergencyService(serviceType, alertData) {
        console.log(`📞 Contacting ${serviceType.toUpperCase()} service...`);

        if (this.demoMode) {
            return {
                service: serviceType,
                status: 'DEMO_CONTACTED',
                responseTime: Math.floor(Math.random() * 10) + 3, // 3-12 minutes
                units: Math.floor(Math.random() * 3) + 1,
                message: `[DEMO] ${serviceType} service notified`
            };
        }

        try {
            const payload = {
                incidentId: alertData.alertId || `INC_${Date.now()}`,
                droneId: alertData.droneId,
                threatLevel: alertData.threatLevel,
                location: alertData.location,
                description: alertData.description,
                timestamp: alertData.timestamp,
                threats: alertData.threats,
                contactInfo: {
                    operator: 'Dark Phoenix Security System',
                    phone: process.env.EMERGENCY_CONTACT_PHONE,
                    email: process.env.EMERGENCY_CONTACT_EMAIL
                }
            };

            const response = await axios.post(
                `${this.emergencyEndpoints[serviceType]}/alert`,
                payload,
                {
                    headers: {
                        'Authorization': `Bearer ${process.env.EMERGENCY_API_KEY}`,
                        'Content-Type': 'application/json'
                    },
                    timeout: 10000 // 10 second timeout
                }
            );

            return {
                service: serviceType,
                status: 'CONTACTED',
                responseTime: response.data.estimatedResponseMinutes,
                units: response.data.unitsDispatched,
                incidentNumber: response.data.incidentNumber
            };

        } catch (error) {
            console.error(`❌ Failed to contact ${serviceType}:`, error.message);
            return {
                service: serviceType,
                status: 'FAILED',
                error: error.message
            };
        }
    }

    /**
     * Make 911 call with Twilio
     */
    async make911Call(alertData) {
        if (this.demoMode || !this.twilioClient) {
            console.log(`📞 [DEMO] 911 Call would be made:`);
            console.log(`   "911 Emergency: Dark Phoenix security drone ${alertData.droneId} has detected`);
            console.log(`   a ${alertData.threatLevel} level threat at coordinates`);
            console.log(`   ${alertData.location.latitude}, ${alertData.location.longitude}.`);
            console.log(`   ${alertData.description}. Please dispatch units immediately."`);
            
            return {
                status: 'DEMO_CALL',
                message: '911 call simulated successfully'
            };
        }

        try {
            // Create emergency message
            const emergencyMessage = this.createEmergencyMessage(alertData);

            // Make the call (to emergency services, not actual 911 in demo)
            const call = await this.twilioClient.calls.create({
                to: process.env.EMERGENCY_PHONE_NUMBER, // Pre-configured emergency number
                from: process.env.TWILIO_PHONE_NUMBER,
                twiml: `
                    <Response>
                        <Say voice="alice">
                            Emergency alert from Dark Phoenix security system.
                            ${emergencyMessage}
                        </Say>
                    </Response>
                `
            });

            return {
                status: 'CALL_PLACED',
                callSid: call.sid,
                message: 'Emergency call placed successfully'
            };

        } catch (error) {
            console.error('❌ 911 call failed:', error);
            return {
                status: 'CALL_FAILED',
                error: error.message
            };
        }
    }

    /**
     * Create emergency message for 911 call
     */
    createEmergencyMessage(alertData) {
        const location = `latitude ${alertData.location.latitude}, longitude ${alertData.location.longitude}`;
        const threat = alertData.threatLevel;
        const description = alertData.description || 'Security threat detected';
        
        return `${threat} level threat detected at ${location}. ${description}. Dark Phoenix drone ${alertData.droneId} is on scene. Immediate response required.`;
    }

    /**
     * Calculate estimated emergency response time
     */
    async calculateResponseTime(location) {
        // In real implementation, this would use:
        // - Local emergency service databases
        // - Traffic conditions API
        // - Historical response time data
        
        // Demo calculation based on urban/rural classification
        const isUrbanArea = await this.isUrbanLocation(location);
        
        return {
            police: isUrbanArea ? '4-8 minutes' : '8-15 minutes',
            ems: isUrbanArea ? '6-10 minutes' : '10-20 minutes',
            fire: isUrbanArea ? '5-9 minutes' : '12-25 minutes'
        };
    }

    /**
     * Determine if location is urban (affects response times)
     */
    async isUrbanLocation(location) {
        // Demo implementation - in reality would use geocoding services
        // For now, assume urban if near major coordinates (placeholder)
        return true; // Assume urban for demo
    }

    /**
     * Get nearest emergency facilities
     */
    async getNearestFacilities(location) {
        // Demo data - real implementation would query emergency facility databases
        return {
            police: {
                name: 'Metro Police Station #4',
                distance: '2.3 miles',
                phone: '(555) 123-POLICE'
            },
            hospital: {
                name: 'City General Hospital',
                distance: '1.8 miles',
                phone: '(555) 123-MEDICAL'
            },
            fire: {
                name: 'Fire Station #7',
                distance: '1.2 miles',
                phone: '(555) 123-FIRE'
            }
        };
    }
}

module.exports = EmergencyService;
