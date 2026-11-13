-- ============================================================================
-- Zuvio - Migration 00004: marketing_consents (remoto), triggers updated_at
-- e índices compuestos para historial de viajes rápido en móvil.
-- Idempotente: seguro de re-ejecutar sobre 00001-00003.
-- ============================================================================

CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER
LANGUAGE plpgsql
SET search_path = public
AS $$
BEGIN
    NEW.updated_at = timezone('utc'::text, now());
    RETURN NEW;
END;
$$;

CREATE TABLE IF NOT EXISTS public.marketing_consents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES public.profiles(id) ON DELETE CASCADE,
    service_notifications BOOLEAN NOT NULL DEFAULT true,
    marketing_emails BOOLEAN NOT NULL DEFAULT false,
    consented_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT timezone('utc'::text, now())
);

ALTER TABLE public.marketing_consents ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "Users can view own marketing consent" ON public.marketing_consents;
CREATE POLICY "Users can view own marketing consent" ON public.marketing_consents FOR SELECT
USING (user_id = auth.uid() OR public.is_admin());

DROP POLICY IF EXISTS "Users can update own marketing consent" ON public.marketing_consents;
CREATE POLICY "Users can update own marketing consent" ON public.marketing_consents FOR UPDATE
USING (user_id = auth.uid()) WITH CHECK (user_id = auth.uid());

DROP POLICY IF EXISTS "Users can insert own marketing consent" ON public.marketing_consents;
CREATE POLICY "Users can insert own marketing consent" ON public.marketing_consents FOR INSERT
WITH CHECK (user_id = auth.uid());

DROP TRIGGER IF EXISTS trg_profiles_updated_at ON public.profiles;
CREATE TRIGGER trg_profiles_updated_at BEFORE UPDATE ON public.profiles FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
DROP TRIGGER IF EXISTS trg_driver_profiles_updated_at ON public.driver_profiles;
CREATE TRIGGER trg_driver_profiles_updated_at BEFORE UPDATE ON public.driver_profiles FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
DROP TRIGGER IF EXISTS trg_vehicles_updated_at ON public.vehicles;
CREATE TRIGGER trg_vehicles_updated_at BEFORE UPDATE ON public.vehicles FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
DROP TRIGGER IF EXISTS trg_rides_updated_at ON public.rides;
CREATE TRIGGER trg_rides_updated_at BEFORE UPDATE ON public.rides FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
DROP TRIGGER IF EXISTS trg_support_tickets_updated_at ON public.support_tickets;
CREATE TRIGGER trg_support_tickets_updated_at BEFORE UPDATE ON public.support_tickets FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
DROP TRIGGER IF EXISTS trg_marketing_consents_updated_at ON public.marketing_consents;
CREATE TRIGGER trg_marketing_consents_updated_at BEFORE UPDATE ON public.marketing_consents FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE INDEX IF NOT EXISTS idx_rides_passenger_history ON public.rides(passenger_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_rides_driver_history ON public.rides(driver_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ride_locations_ride_recorded ON public.ride_locations(ride_id, recorded_at DESC);

INSERT INTO public.marketing_consents (user_id, service_notifications, marketing_emails)
VALUES
    ('00000000-0000-0000-0000-000000000002', true, false),
    ('00000000-0000-0000-0000-000000000010', true, true)
ON CONFLICT (user_id) DO NOTHING;

