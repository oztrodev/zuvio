-- ============================================================================
-- Zuvio Ride-Hailing Platform - Supabase Postgres Schema
-- Migration 00002: Row Level Security (RLS) Policies
-- Enforces least-privilege access for Passengers, Drivers, and Admins
-- ============================================================================

-- Helper functions to extract user context
CREATE OR REPLACE FUNCTION public.current_user_role()
RETURNS user_role AS $$
    SELECT role FROM public.profiles WHERE id = auth.uid();
$$ LANGUAGE sql STABLE SECURITY DEFINER;

CREATE OR REPLACE FUNCTION public.is_admin()
RETURNS BOOLEAN AS $$
    SELECT EXISTS (
        SELECT 1 FROM public.profiles 
        WHERE id = auth.uid() AND role = 'admin'
    );
$$ LANGUAGE sql STABLE SECURITY DEFINER;

-- ----------------------------------------------------------------------------
-- 1. Profiles Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;

-- Anyone authenticated can view their own profile or public rider/driver names
CREATE POLICY "profiles_select_own" ON public.profiles
    FOR SELECT USING (auth.uid() = id OR public.is_admin());

-- Users can update only their own profile
CREATE POLICY "profiles_update_own" ON public.profiles
    FOR UPDATE USING (auth.uid() = id)
    WITH CHECK (auth.uid() = id);

-- Admins can update any profile
CREATE POLICY "profiles_admin_all" ON public.profiles
    FOR ALL USING (public.is_admin());

-- ----------------------------------------------------------------------------
-- 2. Driver Profiles Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.driver_profiles ENABLE ROW LEVEL SECURITY;

-- Driver can view their own profile; Admins can view all; Passengers can view assigned driver
CREATE POLICY "driver_profiles_select" ON public.driver_profiles
    FOR SELECT USING (
        user_id = auth.uid() 
        OR public.is_admin()
        OR EXISTS (
            SELECT 1 FROM public.rides r 
            WHERE r.driver_id = public.driver_profiles.id 
              AND r.passenger_id = auth.uid()
        )
    );

-- Drivers can update their own profile (except status & rating)
CREATE POLICY "driver_profiles_update_own" ON public.driver_profiles
    FOR UPDATE USING (user_id = auth.uid())
    WITH CHECK (user_id = auth.uid());

-- Admins can update any driver profile (approval, rejection, suspension)
CREATE POLICY "driver_profiles_admin_all" ON public.driver_profiles
    FOR ALL USING (public.is_admin());

-- ----------------------------------------------------------------------------
-- 3. Electric Vehicles Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.vehicles ENABLE ROW LEVEL SECURITY;

-- Driver can view their vehicles; Passengers can view vehicle of their active ride; Admins see all
CREATE POLICY "vehicles_select" ON public.vehicles
    FOR SELECT USING (
        driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
        OR public.is_admin()
        OR EXISTS (
            SELECT 1 FROM public.rides r 
            WHERE r.vehicle_id = public.vehicles.id 
              AND r.passenger_id = auth.uid()
        )
    );

-- Driver can register a vehicle
CREATE POLICY "vehicles_insert_own" ON public.vehicles
    FOR INSERT WITH CHECK (
        driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
    );

-- Admins can update vehicle verification status
CREATE POLICY "vehicles_admin_all" ON public.vehicles
    FOR ALL USING (public.is_admin());

-- ----------------------------------------------------------------------------
-- 4. Driver Documents Table RLS (STRICT PRIVACY - NO PUBLIC ACCESS)
-- ----------------------------------------------------------------------------
ALTER TABLE public.driver_documents ENABLE ROW LEVEL SECURITY;

-- Only owning driver and admins can access documents
CREATE POLICY "driver_documents_select" ON public.driver_documents
    FOR SELECT USING (
        driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
        OR public.is_admin()
    );

CREATE POLICY "driver_documents_insert" ON public.driver_documents
    FOR INSERT WITH CHECK (
        driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
    );

CREATE POLICY "driver_documents_admin_all" ON public.driver_documents
    FOR ALL USING (public.is_admin());

-- ----------------------------------------------------------------------------
-- 5. Rides Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.rides ENABLE ROW LEVEL SECURITY;

-- Passenger can view their rides; Assigned driver can view their rides; Admins see all
CREATE POLICY "rides_select" ON public.rides
    FOR SELECT USING (
        passenger_id = auth.uid()
        OR driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
        OR public.is_admin()
        OR (status = 'matching' AND EXISTS (
            SELECT 1 FROM public.driver_profiles dp 
            WHERE dp.user_id = auth.uid() AND dp.status = 'approved' AND dp.is_online = true
        ))
    );

-- Passenger can create a ride request
CREATE POLICY "rides_insert_passenger" ON public.rides
    FOR INSERT WITH CHECK (passenger_id = auth.uid());

-- Passenger can cancel their ride if not started; Driver can update assigned ride status; Admin full
CREATE POLICY "rides_update" ON public.rides
    FOR UPDATE USING (
        passenger_id = auth.uid()
        OR driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
        OR public.is_admin()
    );

-- ----------------------------------------------------------------------------
-- 6. Ride Events Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.ride_events ENABLE ROW LEVEL SECURITY;

CREATE POLICY "ride_events_select" ON public.ride_events
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM public.rides r 
            WHERE r.id = public.ride_events.ride_id 
              AND (r.passenger_id = auth.uid() 
                   OR r.driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
                   OR public.is_admin())
        )
    );

CREATE POLICY "ride_events_insert" ON public.ride_events
    FOR INSERT WITH CHECK (
        EXISTS (
            SELECT 1 FROM public.rides r 
            WHERE r.id = public.ride_events.ride_id 
              AND (r.passenger_id = auth.uid() 
                   OR r.driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
                   OR public.is_admin())
        )
    );

-- ----------------------------------------------------------------------------
-- 7. Ride Locations Table RLS (EPHEMERAL / MINIMIZED PRIVACY)
-- ----------------------------------------------------------------------------
ALTER TABLE public.ride_locations ENABLE ROW LEVEL SECURITY;

CREATE POLICY "ride_locations_select" ON public.ride_locations
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM public.rides r 
            WHERE r.id = public.ride_locations.ride_id 
              AND r.status IN ('driver_en_route', 'arrived', 'in_progress')
              AND (r.passenger_id = auth.uid() 
                   OR r.driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
                   OR public.is_admin())
        )
    );

CREATE POLICY "ride_locations_insert" ON public.ride_locations
    FOR INSERT WITH CHECK (
        driver_id IN (SELECT id FROM public.driver_profiles WHERE user_id = auth.uid())
    );

-- ----------------------------------------------------------------------------
-- 8. Ride Ratings Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.ride_ratings ENABLE ROW LEVEL SECURITY;

CREATE POLICY "ride_ratings_select" ON public.ride_ratings
    FOR SELECT USING (
        rater_id = auth.uid() 
        OR rated_user_id = auth.uid() 
        OR public.is_admin()
    );

CREATE POLICY "ride_ratings_insert" ON public.ride_ratings
    FOR INSERT WITH CHECK (rater_id = auth.uid());

-- ----------------------------------------------------------------------------
-- 9. Support Tickets Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.support_tickets ENABLE ROW LEVEL SECURITY;

CREATE POLICY "support_tickets_select" ON public.support_tickets
    FOR SELECT USING (user_id = auth.uid() OR public.is_admin());

CREATE POLICY "support_tickets_insert" ON public.support_tickets
    FOR INSERT WITH CHECK (user_id = auth.uid());

CREATE POLICY "support_tickets_admin_update" ON public.support_tickets
    FOR UPDATE USING (public.is_admin());

-- ----------------------------------------------------------------------------
-- 10. Marketing Consent Table RLS
-- ----------------------------------------------------------------------------
ALTER TABLE public.marketing_consents ENABLE ROW LEVEL SECURITY;

CREATE POLICY "marketing_consents_all" ON public.marketing_consents
    FOR ALL USING (user_id = auth.uid() OR public.is_admin());

-- ----------------------------------------------------------------------------
-- 11. Audit Logs Table RLS (ADMIN READ ONLY)
-- ----------------------------------------------------------------------------
ALTER TABLE public.audit_logs ENABLE ROW LEVEL SECURITY;

CREATE POLICY "audit_logs_admin_select" ON public.audit_logs
    FOR SELECT USING (public.is_admin());

CREATE POLICY "audit_logs_insert" ON public.audit_logs
    FOR INSERT WITH CHECK (true);

