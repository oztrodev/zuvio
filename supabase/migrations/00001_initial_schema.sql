-- ============================================================================
-- Zuvio Ride-Hailing Platform - Supabase Postgres Schema
-- Initial Migration: Schema, Enums, Tables, Constraints & Triggers
-- Santiago de Chile 100% Electric Vehicle Fleet
-- ============================================================================

-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "postgis";

-- Enums
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('passenger', 'driver', 'admin');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE driver_status AS ENUM ('pending_review', 'approved', 'rejected', 'suspended');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    -- Only Battery Electric Vehicles (BEV) are permitted on Zuvio
    CREATE TYPE propulsion_type AS ENUM ('BEV');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE vehicle_status AS ENUM ('pending_review', 'approved', 'rejected', 'inactive');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE ride_status AS ENUM (
        'requested',
        'matching',
        'accepted',
        'driver_en_route',
        'arrived',
        'in_progress',
        'completed',
        'cancelled',
        'expired'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE driver_doc_type AS ENUM (
        'driver_license',
        'vehicle_registration',
        'ev_homologation',
        'background_check'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE ticket_status AS ENUM ('open', 'in_progress', 'resolved', 'closed');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- ----------------------------------------------------------------------------
-- 1. Profiles (Linked to auth.users if Supabase Auth is active)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    full_name TEXT NOT NULL,
    phone TEXT,
    role user_role NOT NULL DEFAULT 'passenger',
    avatar_url TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 2. Driver Profiles
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.driver_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES public.profiles(id) ON DELETE CASCADE,
    status driver_status NOT NULL DEFAULT 'pending_review',
    license_number TEXT NOT NULL,
    license_class TEXT NOT NULL DEFAULT 'A2', -- A2 or A3 Chilean Professional License
    background_check_status TEXT NOT NULL DEFAULT 'pending',
    rating_avg NUMERIC(3, 2) NOT NULL DEFAULT 5.00,
    total_trips INTEGER NOT NULL DEFAULT 0,
    is_online BOOLEAN NOT NULL DEFAULT false,
    current_lat NUMERIC(9, 6),
    current_lng NUMERIC(9, 6),
    coverage_radius_km NUMERIC(5, 2) NOT NULL DEFAULT 8.00 CHECK (coverage_radius_km >= 1.0 AND coverage_radius_km <= 50.0),
    geom GEOMETRY(Point, 4326),
    last_seen_at TIMESTAMPTZ,
    reviewed_at TIMESTAMPTZ,
    reviewed_by UUID REFERENCES public.profiles(id),
    review_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 3. Electric Vehicles (STRICT 100% BEV VALIDATION)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.vehicles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    driver_id UUID NOT NULL REFERENCES public.driver_profiles(id) ON DELETE CASCADE,
    brand TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL,
    license_plate TEXT NOT NULL UNIQUE, -- Chilean format e.g. ABCD-12
    propulsion propulsion_type NOT NULL DEFAULT 'BEV',
    battery_capacity_kwh NUMERIC(5, 2) NOT NULL,
    passenger_capacity INTEGER NOT NULL DEFAULT 4,
    status vehicle_status NOT NULL DEFAULT 'pending_review',
    is_active BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    verified_by UUID REFERENCES public.profiles(id),
    verification_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    CONSTRAINT chk_vehicle_is_bev CHECK (propulsion = 'BEV'),
    CONSTRAINT chk_vehicle_year CHECK (year >= 2018), -- Ley EAT Chile modern vehicle requirement
    CONSTRAINT chk_battery_positive CHECK (battery_capacity_kwh > 0),
    CONSTRAINT chk_passenger_capacity CHECK (passenger_capacity >= 3)
);

-- ----------------------------------------------------------------------------
-- 4. Private Driver Documents
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.driver_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    driver_id UUID NOT NULL REFERENCES public.driver_profiles(id) ON DELETE CASCADE,
    doc_type driver_doc_type NOT NULL,
    storage_path TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- pending, verified, rejected
    reviewed_at TIMESTAMPTZ,
    reviewed_by UUID REFERENCES public.profiles(id),
    rejection_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 5. Rides (Trips Lifecycle)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.rides (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    passenger_id UUID NOT NULL REFERENCES public.profiles(id),
    driver_id UUID REFERENCES public.driver_profiles(id),
    vehicle_id UUID REFERENCES public.vehicles(id),
    status ride_status NOT NULL DEFAULT 'requested',
    
    -- Origin in Santiago de Chile
    origin_address TEXT NOT NULL,
    origin_commune TEXT NOT NULL,
    origin_lat NUMERIC(9, 6) NOT NULL,
    origin_lng NUMERIC(9, 6) NOT NULL,
    
    -- Destination in Santiago de Chile
    destination_address TEXT NOT NULL,
    destination_commune TEXT NOT NULL,
    destination_lat NUMERIC(9, 6) NOT NULL,
    destination_lng NUMERIC(9, 6) NOT NULL,
    
    -- Distance & Fare (CLP)
    distance_km NUMERIC(6, 2) NOT NULL,
    duration_minutes INTEGER NOT NULL,
    estimated_fare_clp INTEGER NOT NULL,
    final_fare_clp INTEGER,
    
    -- Cancellation
    cancellation_reason TEXT,
    cancelled_by UUID REFERENCES public.profiles(id),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    accepted_at TIMESTAMPTZ,
    driver_arrived_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ
);

-- ----------------------------------------------------------------------------
-- 6. Ride Events (State transitions audit trail)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.ride_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ride_id UUID NOT NULL REFERENCES public.rides(id) ON DELETE CASCADE,
    previous_status ride_status,
    new_status ride_status NOT NULL,
    triggered_by UUID REFERENCES public.profiles(id),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 7. Ride Locations (Ephemeral minimized location sharing during trip)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.ride_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ride_id UUID NOT NULL REFERENCES public.rides(id) ON DELETE CASCADE,
    driver_id UUID NOT NULL REFERENCES public.driver_profiles(id),
    latitude NUMERIC(9, 6) NOT NULL,
    longitude NUMERIC(9, 6) NOT NULL,
    speed_kmh NUMERIC(5, 2),
    heading NUMERIC(5, 2),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 8. Ride Ratings
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.ride_ratings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ride_id UUID NOT NULL UNIQUE REFERENCES public.rides(id) ON DELETE CASCADE,
    rater_id UUID NOT NULL REFERENCES public.profiles(id),
    rated_user_id UUID NOT NULL REFERENCES public.profiles(id),
    score INTEGER NOT NULL CHECK (score >= 1 AND score <= 5),
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 9. Support Tickets
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.support_tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES public.profiles(id),
    ride_id UUID REFERENCES public.rides(id),
    subject TEXT NOT NULL,
    message TEXT NOT NULL,
    status ticket_status NOT NULL DEFAULT 'open',
    resolution_notes TEXT,
    resolved_by UUID REFERENCES public.profiles(id),
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 10. Marketing & Service Consent (Explicit Separation)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.marketing_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES public.profiles(id) ON DELETE CASCADE,
    service_notifications BOOLEAN NOT NULL DEFAULT true,
    marketing_emails BOOLEAN NOT NULL DEFAULT false, -- Strict opt-in
    consented_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- 11. Audit Logs (Admin & Security Events)
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS public.audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id UUID REFERENCES public.profiles(id),
    action TEXT NOT NULL,
    target_entity TEXT NOT NULL,
    target_id UUID,
    details JSONB,
    ip_address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

-- ----------------------------------------------------------------------------
-- Indexes for Performance & Constraints
-- ----------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_rides_passenger_id ON public.rides(passenger_id);
CREATE INDEX IF NOT EXISTS idx_rides_driver_id ON public.rides(driver_id);
CREATE INDEX IF NOT EXISTS idx_rides_status ON public.rides(status);
CREATE INDEX IF NOT EXISTS idx_rides_created_at ON public.rides(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ride_events_ride_id ON public.ride_events(ride_id);
CREATE INDEX IF NOT EXISTS idx_ride_locations_ride_id ON public.ride_locations(ride_id);
CREATE INDEX IF NOT EXISTS idx_vehicles_driver_id ON public.vehicles(driver_id);
CREATE INDEX IF NOT EXISTS idx_vehicles_status ON public.vehicles(status);
CREATE INDEX IF NOT EXISTS idx_driver_profiles_status ON public.driver_profiles(status);
CREATE INDEX IF NOT EXISTS idx_driver_profiles_is_online ON public.driver_profiles(is_online);
CREATE INDEX IF NOT EXISTS idx_support_tickets_user_id ON public.support_tickets(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON public.audit_logs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_driver_profiles_geom ON public.driver_profiles USING GIST (geom);

-- ----------------------------------------------------------------------------
-- Trigger for automatic updated_at timestamp
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = timezone('utc'::text, now());
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_profiles_updated_at BEFORE UPDATE ON public.profiles FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
CREATE TRIGGER trg_driver_profiles_updated_at BEFORE UPDATE ON public.driver_profiles FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
CREATE TRIGGER trg_vehicles_updated_at BEFORE UPDATE ON public.vehicles FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
CREATE TRIGGER trg_rides_updated_at BEFORE UPDATE ON public.rides FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
CREATE TRIGGER trg_support_tickets_updated_at BEFORE UPDATE ON public.support_tickets FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();


-- ----------------------------------------------------------------------------
-- PostGIS: sincroniza geom desde lat/lng del conductor
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION public.sync_driver_geom()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.current_lat IS NOT NULL AND NEW.current_lng IS NOT NULL THEN
        NEW.geom = ST_SetSRID(ST_MakePoint(NEW.current_lng, NEW.current_lat), 4326);
    END IF;
    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_driver_profiles_geom ON public.driver_profiles;
CREATE TRIGGER trg_driver_profiles_geom
BEFORE INSERT OR UPDATE OF current_lat, current_lng ON public.driver_profiles
FOR EACH ROW EXECUTE FUNCTION public.sync_driver_geom();

-- ----------------------------------------------------------------------------
-- RPC: conductores eléctricos cercanos que cubren al pasajero dentro de su radio
-- ----------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION public.get_nearby_drivers(
    p_lat DOUBLE PRECISION,
    p_lng DOUBLE PRECISION,
    p_max_search_radius_km DOUBLE PRECISION DEFAULT 15.0
)
RETURNS TABLE(
    driver_id UUID, user_id UUID, full_name TEXT, rating_avg NUMERIC, total_trips INTEGER,
    latitude NUMERIC, longitude NUMERIC, coverage_radius_km NUMERIC, distance_km DOUBLE PRECISION,
    estimated_arrival_minutes INTEGER, vehicle_brand TEXT, vehicle_model TEXT,
    license_plate TEXT, battery_kwh NUMERIC
)
LANGUAGE plpgsql
STABLE
AS $$
DECLARE
    v_passenger_point GEOMETRY;
BEGIN
    v_passenger_point := ST_SetSRID(ST_MakePoint(p_lng, p_lat), 4326);

    RETURN QUERY
    SELECT
        dp.id AS driver_id,
        dp.user_id,
        p.full_name,
        dp.rating_avg,
        dp.total_trips,
        dp.current_lat AS latitude,
        dp.current_lng AS longitude,
        dp.coverage_radius_km,
        ROUND((ST_Distance(dp.geom::geography, v_passenger_point::geography) / 1000.0)::numeric, 2)::double precision AS distance_km,
        GREATEST(2, ROUND((ST_Distance(dp.geom::geography, v_passenger_point::geography) / 1000.0 / 25.0 * 60.0)))::integer AS estimated_arrival_minutes,
        v.brand AS vehicle_brand,
        v.model AS vehicle_model,
        v.license_plate,
        v.battery_capacity_kwh AS battery_kwh
    FROM public.driver_profiles dp
    JOIN public.profiles p ON p.id = dp.user_id
    JOIN public.vehicles v ON v.driver_id = dp.id AND v.status = 'approved' AND v.is_active = true
    WHERE dp.status = 'approved'
      AND dp.is_online = true
      AND dp.geom IS NOT NULL
      AND ST_DWithin(dp.geom::geography, v_passenger_point::geography, LEAST(dp.coverage_radius_km, p_max_search_radius_km) * 1000.0)
    ORDER BY ST_Distance(dp.geom::geography, v_passenger_point::geography) ASC
    LIMIT 20;
END;
$$;
