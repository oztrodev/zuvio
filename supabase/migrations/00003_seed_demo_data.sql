-- ============================================================================
-- Zuvio Ride-Hailing Platform - Supabase Postgres Schema
-- Migration 00003: Seed Demo Data (Santiago de Chile 100% Electric Fleet)
-- All data is demonstrative and isolated from production
-- ============================================================================

-- Fixed Demo UUIDs for reproducible testing
DO $$
DECLARE
    -- Profiles
    v_admin_id UUID := '00000000-0000-0000-0000-000000000001';
    v_passenger1_id UUID := '00000000-0000-0000-0000-000000000002';
    v_passenger2_id UUID := '00000000-0000-0000-0000-000000000003';
    v_driver1_user_id UUID := '00000000-0000-0000-0000-000000000010';
    v_driver2_user_id UUID := '00000000-0000-0000-0000-000000000020';
    v_driver3_user_id UUID := '00000000-0000-0000-0000-000000000030';
    
    -- Driver Profiles
    v_driver1_profile_id UUID := '10000000-0000-0000-0000-000000000001';
    v_driver2_profile_id UUID := '10000000-0000-0000-0000-000000000002';
    v_driver3_profile_id UUID := '10000000-0000-0000-0000-000000000003';
    
    -- Vehicles
    v_vehicle1_id UUID := '20000000-0000-0000-0000-000000000001';
    v_vehicle2_id UUID := '20000000-0000-0000-0000-000000000002';
    v_vehicle3_id UUID := '20000000-0000-0000-0000-000000000003';
    
    -- Sample Completed Ride
    v_sample_ride_id UUID := '30000000-0000-0000-0000-000000000001';
BEGIN
    -- 1. Profiles
    INSERT INTO public.profiles (id, email, full_name, phone, role)
    VALUES
        (v_admin_id, 'admin@zuvio.cl', 'Carlos Morales (Operaciones Zuvio)', '+56 9 8888 1111', 'admin'),
        (v_passenger1_id, 'pasajero@zuvio.cl', 'Valentina Silva', '+56 9 9123 4567', 'passenger'),
        (v_passenger2_id, 'rider2@zuvio.cl', 'Matias Gomez', '+56 9 7654 3210', 'passenger'),
        (v_driver1_user_id, 'conductor1@zuvio.cl', 'Rodrigo Araya', '+56 9 8111 2222', 'driver'),
        (v_driver2_user_id, 'conductor2@zuvio.cl', 'Camila Soto', '+56 9 8222 3333', 'driver'),
        (v_driver3_user_id, 'conductor.nuevo@zuvio.cl', 'Esteban Fuentes', '+56 9 8333 4444', 'driver')
    ON CONFLICT (id) DO UPDATE SET full_name = EXCLUDED.full_name;

    -- 2. Driver Profiles
    INSERT INTO public.driver_profiles (
        id, user_id, status, license_number, license_class, background_check_status, 
        rating_avg, total_trips, is_online, current_lat, current_lng, coverage_radius_km, last_seen_at
    )
    VALUES
        (
            v_driver1_profile_id, v_driver1_user_id, 'approved', '15.489.231-K', 'A2', 
            'approved', 4.96, 142, true, -33.424500, -70.612000, 8.00, now()
        ),
        (
            v_driver2_profile_id, v_driver2_user_id, 'approved', '16.712.980-3', 'A2', 
            'approved', 4.92, 89, true, -33.411000, -70.589000, 10.00, now()
        ),
        (
            v_driver3_profile_id, v_driver3_user_id, 'pending_review', '18.123.456-7', 'A2', 
            'pending', 5.00, 0, false, -33.448900, -70.669300, 5.00, now()
        )
    ON CONFLICT (id) DO NOTHING;

    -- 3. Vehicles (Strictly BEV)
    INSERT INTO public.vehicles (
        id, driver_id, brand, model, year, license_plate, propulsion, 
        battery_capacity_kwh, passenger_capacity, status, is_active, verified_at
    )
    VALUES
        (
            v_vehicle1_id, v_driver1_profile_id, 'Tesla', 'Model 3 Standard Range Plus', 
            2023, 'LW-72-91', 'BEV', 60.00, 4, 'approved', true, now()
        ),
        (
            v_vehicle2_id, v_driver2_profile_id, 'BYD', 'Dolphin EV', 
            2024, 'PP-45-12', 'BEV', 44.90, 4, 'approved', true, now()
        ),
        (
            v_vehicle3_id, v_driver3_profile_id, 'Hyundai', 'Ioniq 5', 
            2023, 'TG-88-34', 'BEV', 72.60, 4, 'pending_review', false, NULL
        )
    ON CONFLICT (id) DO NOTHING;

    -- 4. Sample Completed Ride in Santiago (Providencia -> Las Condes)
    INSERT INTO public.rides (
        id, passenger_id, driver_id, vehicle_id, status,
        origin_address, origin_commune, origin_lat, origin_lng,
        destination_address, destination_commune, destination_lat, destination_lng,
        distance_km, duration_minutes, estimated_fare_clp, final_fare_clp,
        created_at, accepted_at, started_at, completed_at
    )
    VALUES (
        v_sample_ride_id, v_passenger1_id, v_driver1_profile_id, v_vehicle1_id, 'completed',
        'Av. Providencia 1234 (Metro Manuel Montt)', 'Providencia', -33.428500, -70.618900,
        'Av. Apoquindo 4500 (Metro Escuela Militar)', 'Las Condes', -33.415000, -70.579000,
        5.4, 16, 5200, 5200,
        now() - INTERVAL '3 hours',
        now() - INTERVAL '2 hours 55 minutes',
        now() - INTERVAL '2 hours 45 minutes',
        now() - INTERVAL '2 hours 29 minutes'
    )
    ON CONFLICT (id) DO NOTHING;

    -- 5. Rating for the completed ride
    INSERT INTO public.ride_ratings (ride_id, rater_id, rated_user_id, score, comment)
    VALUES (
        v_sample_ride_id, v_passenger1_id, v_driver1_user_id, 5, 
        'Excelente viaje, auto 100% eléctrico silencioso y muy cómodo. Conductor muy profesional.'
    )
    ON CONFLICT (ride_id) DO NOTHING;

    -- 6. Marketing Consents
    INSERT INTO public.marketing_consents (user_id, service_notifications, marketing_emails)
    VALUES
        (v_passenger1_id, true, false),
        (v_driver1_user_id, true, true)
    ON CONFLICT (user_id) DO NOTHING;

    -- 7. Sample Support Ticket
    INSERT INTO public.support_tickets (user_id, ride_id, subject, message, status)
    VALUES (
        v_passenger1_id, v_sample_ride_id, 
        'Consulta sobre detalle del viaje eléctrico', 
        'Hola equipo Zuvio, me encantó el viaje en el Tesla eléctrico. ¿Dónde puedo ver el comprobante digital?', 
        'resolved'
    )
    ON CONFLICT (id) DO NOTHING;

END $$;

