-- ============================================================================
-- Zuvio - Migration 00005: Endurecimiento de funciones (Supabase advisors)
-- - search_path fijo en funciones
-- - is_admin() y get_nearby_drivers() solo para usuarios autenticados
--   (la ubicación de conductores no se expone a usuarios anónimos)
-- ============================================================================

ALTER FUNCTION public.sync_driver_geom() SET search_path = public;
ALTER FUNCTION public.get_nearby_drivers(double precision, double precision, double precision) SET search_path = public;
ALTER FUNCTION public.is_admin() SET search_path = public;

REVOKE EXECUTE ON FUNCTION public.is_admin() FROM PUBLIC, anon;
GRANT EXECUTE ON FUNCTION public.is_admin() TO authenticated;

REVOKE EXECUTE ON FUNCTION public.get_nearby_drivers(double precision, double precision, double precision) FROM PUBLIC, anon;
GRANT EXECUTE ON FUNCTION public.get_nearby_drivers(double precision, double precision, double precision) TO authenticated;

