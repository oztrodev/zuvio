-- ============================================================================
-- Zuvio - Migration 00006: Cierra funciones expuestas por la API REST
-- - is_admin() se mueve al esquema privado (PostgREST no lo expone);
--   las politicas RLS que la usan siguen funcionando (referencia por OID).
-- - Se revoca EXECUTE de funciones utilitarias a anon/authenticated.
-- Nota: public.spatial_ref_sys y la extension postgis en public pertenecen a
-- la extension; no se pueden modificar sin privilegios de propietario.
-- ============================================================================

REVOKE EXECUTE ON FUNCTION public.rls_auto_enable() FROM PUBLIC, anon, authenticated;
REVOKE EXECUTE ON FUNCTION public.st_estimatedextent(text, text) FROM PUBLIC, anon, authenticated;
REVOKE EXECUTE ON FUNCTION public.st_estimatedextent(text, text, text) FROM PUBLIC, anon, authenticated;
REVOKE EXECUTE ON FUNCTION public.st_estimatedextent(text, text, text, boolean) FROM PUBLIC, anon, authenticated;

CREATE SCHEMA IF NOT EXISTS private;
GRANT USAGE ON SCHEMA private TO authenticated;
ALTER FUNCTION public.is_admin() SET SCHEMA private;
REVOKE EXECUTE ON FUNCTION private.is_admin() FROM PUBLIC, anon;
GRANT EXECUTE ON FUNCTION private.is_admin() TO authenticated;

