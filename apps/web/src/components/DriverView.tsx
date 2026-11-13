import React, { useState, useEffect } from 'react';
import {
  Car,
  Power,
  CheckCircle2,
  Navigation,
  ExternalLink,
  AlertTriangle,
  Sliders
} from 'lucide-react';
import { ApiClient } from '../api';
import { DriverProfile, Ride, Vehicle } from '../types';

export const DriverView: React.FC = () => {
  const [profile, setProfile] = useState<DriverProfile | null>(null);
  const [vehicle, setVehicle] = useState<Vehicle | null>(null);
  const [isOnline, setIsOnline] = useState(false);
  const [coverageRadius, setCoverageRadius] = useState<number>(8.0);
  const [availableRides, setAvailableRides] = useState<Ride[]>([]);
  const [activeRide, setActiveRide] = useState<Ride | null>(null);
  const [completedRides, setCompletedRides] = useState<Ride[]>([]);
  const [loading, setLoading] = useState(true);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);
  const [successMsg, setSuccessMsg] = useState<string | null>(null);

  // Register EV Form
  const [showRegisterEV, setShowRegisterEV] = useState(false);
  const [evBrand, setEvBrand] = useState('BYD');
  const [evModel, setEvModel] = useState('Dolphin EV');
  const [evYear, setEvYear] = useState(2024);
  const [evPlate, setEvPlate] = useState('PP-45-12');
  const [evBattery, setEvBattery] = useState(44.9);
  const [evCapacity, setEvCapacity] = useState(4);

  const refreshData = async () => {
    try {
      const data = await ApiClient.getDriverProfile();
      setProfile(data.profile);
      setVehicle(data.vehicle);
      if (data.profile) {
        setIsOnline(data.profile.is_online);
        setCoverageRadius(data.profile.coverage_radius_km || 8.0);
      }

      const activeData = await ApiClient.getActiveRide();
      setActiveRide(activeData.ride);

      if (data.profile?.status === 'approved') {
        const available = await ApiClient.getAvailableRides();
        setAvailableRides(available);
      }

      const history = await ApiClient.listRides();
      setCompletedRides(history.filter((r) => r.status === 'completed'));
    } catch (_) {
      // ignore
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refreshData();
    const interval = setInterval(refreshData, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleToggleOnline = async () => {
    setErrorMsg(null);
    setSuccessMsg(null);
    try {
      const res = await ApiClient.toggleAvailability(!isOnline);
      setIsOnline(res.is_online);
      setSuccessMsg(res.message);
      refreshData();
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al cambiar disponibilidad');
    }
  };

  const handleCoverageRadiusChange = async (newRadius: number) => {
    setCoverageRadius(newRadius);
    setErrorMsg(null);
    setSuccessMsg(null);
    try {
      const res = await ApiClient.updateCoverageRadius(newRadius);
      setSuccessMsg(res.message);
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al actualizar radio de cobertura');
    }
  };

  const handleRegisterEV = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg(null);
    setSuccessMsg(null);
    try {
      const res = await ApiClient.registerEV({
        brand: evBrand,
        model: evModel,
        year: Number(evYear),
        license_plate: evPlate,
        propulsion: 'BEV',
        battery_capacity_kwh: Number(evBattery),
        passenger_capacity: Number(evCapacity),
      });
      setSuccessMsg(res.message);
      setShowRegisterEV(false);
      refreshData();
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al registrar vehículo eléctrico');
    }
  };

  const handleAcceptRide = async (rideId: string) => {
    setErrorMsg(null);
    try {
      const res = await ApiClient.acceptRide(rideId);
      setSuccessMsg(res.message);
      setActiveRide(res.ride);
      refreshData();
    } catch (err: any) {
      setErrorMsg(err.message || 'No fue posible tomar la solicitud');
    }
  };

  const handleUpdateStatus = async (nextStatus: string) => {
    if (!activeRide) return;
    setErrorMsg(null);
    try {
      const res = await ApiClient.updateRideStatus(activeRide.id, nextStatus);
      setActiveRide(res.ride);
      refreshData();
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al actualizar estado');
    }
  };

  if (loading) {
    return <div style={{ padding: 20, textAlign: 'center', color: 'var(--text-muted)' }}>Cargando portal conductor...</div>;
  }

  const isApproved = profile?.status === 'approved';

  return (
    <div style={{ padding: 16, display: 'flex', flexDirection: 'column', gap: 16 }}>
      {/* Alert Banners */}
      {errorMsg && (
        <div style={{
          padding: '12px 14px',
          background: 'rgba(239, 68, 68, 0.15)',
          border: '1px solid rgba(239, 68, 68, 0.3)',
          borderRadius: 12,
          color: '#F87171',
          fontSize: 13,
          display: 'flex',
          alignItems: 'center',
          gap: 8
        }}>
          <AlertTriangle size={16} />
          <span>{errorMsg}</span>
        </div>
      )}

      {successMsg && (
        <div style={{
          padding: '12px 14px',
          background: 'rgba(16, 185, 129, 0.15)',
          border: '1px solid rgba(16, 185, 129, 0.3)',
          borderRadius: 12,
          color: '#34D399',
          fontSize: 13,
          display: 'flex',
          alignItems: 'center',
          gap: 8
        }}>
          <CheckCircle2 size={16} />
          <span>{successMsg}</span>
        </div>
      )}

      {/* Driver Status Card */}
      <div className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div>
            <span style={{ fontSize: 11, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
              Estado del Conductor Zuvio
            </span>
            <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginTop: 2 }}>
              <h3 style={{ fontSize: 16 }}>Rodrigo Araya</h3>
              <span className={`badge-role ${isApproved ? 'badge-role-driver' : 'badge-role-admin'}`}>
                {isApproved ? 'Aprobado' : profile?.status || 'En Revisión'}
              </span>
            </div>
          </div>

          <button
            onClick={handleToggleOnline}
            disabled={!isApproved}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: 6,
              padding: '8px 14px',
              borderRadius: 20,
              fontSize: 12,
              fontWeight: 700,
              background: isOnline ? 'rgba(16, 185, 129, 0.2)' : 'var(--bg-surface)',
              border: `1px solid ${isOnline ? '#10B981' : 'var(--border-subtle)'}`,
              color: isOnline ? '#34D399' : 'var(--text-muted)',
            }}
          >
            <Power size={14} />
            <span>{isOnline ? 'CONECTADO' : 'DESCONECTADO'}</span>
          </button>
        </div>

        {/* Radio de Cobertura Configurable por el Chofer */}
        <div style={{
          background: 'var(--bg-surface)',
          borderRadius: 12,
          padding: 12,
          border: '1px solid var(--border-subtle)',
          display: 'flex',
          flexDirection: 'column',
          gap: 8
        }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
              <Sliders size={16} color="#8B5CF6" />
              <span style={{ fontSize: 13, fontWeight: 700 }}>Radio de Cobertura en Santiago</span>
            </div>
            <span style={{ fontSize: 15, fontWeight: 800, color: '#38BDF8' }}>
              {coverageRadius} km
            </span>
          </div>

          <p style={{ fontSize: 11, color: 'var(--text-secondary)' }}>
            Solo recibirás solicitudes cuyo punto de recogida esté a un máximo de <strong>{coverageRadius} km</strong> de tu posición.
          </p>

          <input
            type="range"
            min="2"
            max="30"
            step="1"
            value={coverageRadius}
            onChange={(e) => handleCoverageRadiusChange(Number(e.target.value))}
            style={{ margin: '6px 0' }}
          />

          <div style={{ display: 'flex', gap: 6 }}>
            {[3, 5, 8, 10, 15, 20].map((r) => (
              <button
                key={r}
                type="button"
                onClick={() => handleCoverageRadiusChange(r)}
                style={{
                  flex: 1,
                  padding: '4px 0',
                  fontSize: 11,
                  fontWeight: 600,
                  borderRadius: 6,
                  background: coverageRadius === r ? 'var(--gradient-zuvio)' : 'var(--bg-main)',
                  color: coverageRadius === r ? '#FFFFFF' : 'var(--text-secondary)',
                  border: '1px solid var(--border-subtle)'
                }}
              >
                {r}k
              </button>
            ))}
          </div>
        </div>

        {/* Vehicle Badge */}
        {vehicle ? (
          <div style={{
            background: 'var(--bg-surface)',
            borderRadius: 10,
            padding: 10,
            border: '1px solid var(--border-subtle)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between'
          }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
              <Car size={20} color="#8B5CF6" />
              <div>
                <div style={{ fontSize: 13, fontWeight: 600 }}>{vehicle.brand} {vehicle.model} ({vehicle.year})</div>
                <div style={{ fontSize: 11, color: 'var(--text-muted)' }}>
                  Patente: {vehicle.license_plate} · Batería: {vehicle.battery_capacity_kwh} kWh
                </div>
              </div>
            </div>
            <span className="badge-ev">{vehicle.status === 'approved' ? 'EV Verificado' : 'En Revisión'}</span>
          </div>
        ) : (
          <div style={{
            background: 'rgba(245, 158, 11, 0.1)',
            border: '1px solid rgba(245, 158, 11, 0.3)',
            borderRadius: 10,
            padding: 12,
            color: '#FBBF24',
            fontSize: 12,
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center'
          }}>
            <span>No tienes vehículo eléctrico registrado</span>
            <button
              onClick={() => setShowRegisterEV(true)}
              className="btn-secondary"
              style={{ padding: '6px 12px', fontSize: 11 }}
            >
              Registrar EV
            </button>
          </div>
        )}

        {!isApproved && (
          <div style={{
            background: 'rgba(239, 68, 68, 0.1)',
            border: '1px solid rgba(239, 68, 68, 0.25)',
            borderRadius: 10,
            padding: 10,
            fontSize: 12,
            color: '#FCA5A5'
          }}>
            Tu perfil o auto eléctrico se encuentran en validación manual según la Ley EAT. Puedes aprobarlo desde el rol de <strong>Operaciones</strong> para probar la recepción de viajes.
          </div>
        )}
      </div>

      {/* Active Trip Workflow */}
      {activeRide && (
        <div className="z-card z-card-glow" style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <span className="badge-ev">VIAJE EN CURSO (CONDUCTOR)</span>
            <span style={{ fontSize: 12, fontWeight: 700, color: '#38BDF8' }}>
              ${activeRide.estimated_fare_clp.toLocaleString('es-CL')} CLP
            </span>
          </div>

          <div style={{ fontSize: 13, lineHeight: 1.6 }}>
            <div><strong>Recoger en:</strong> {activeRide.origin_address}</div>
            <div><strong>Destino:</strong> {activeRide.destination_address}</div>
            <div style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 4 }}>
              Distancia: {activeRide.distance_km} km · Duración estimada: ~{activeRide.duration_minutes} min
            </div>
          </div>

          <a
            href={`https://maps.google.com/?q=${activeRide.origin_lat},${activeRide.origin_lng}`}
            target="_blank"
            rel="noopener noreferrer"
            className="btn-secondary"
            style={{ fontSize: 12, textDecoration: 'none' }}
          >
            <ExternalLink size={14} />
            <span>Abrir navegación hacia el punto de recogida</span>
          </a>

          <div style={{ display: 'flex', flexDirection: 'column', gap: 8, marginTop: 4 }}>
            {activeRide.status === 'accepted' && (
              <button onClick={() => handleUpdateStatus('driver_en_route')} className="btn-primary">
                <Navigation size={16} />
                <span>Iniciar Ruta hacia el Pasajero</span>
              </button>
            )}

            {activeRide.status === 'driver_en_route' && (
              <button onClick={() => handleUpdateStatus('arrived')} className="btn-primary" style={{ background: '#059669' }}>
                <CheckCircle2 size={16} />
                <span>Llegué al Punto de Encuentro</span>
              </button>
            )}

            {activeRide.status === 'arrived' && (
              <button onClick={() => handleUpdateStatus('in_progress')} className="btn-primary">
                <Navigation size={16} />
                <span>Pasajero a Bordo: Iniciar Viaje a Destino</span>
              </button>
            )}

            {activeRide.status === 'in_progress' && (
              <button onClick={() => handleUpdateStatus('completed')} className="btn-primary" style={{ background: 'var(--gradient-zuvio)' }}>
                <CheckCircle2 size={16} />
                <span>Finalizar Viaje en Destino</span>
              </button>
            )}
          </div>
        </div>
      )}

      {/* Available Matching Requests Queue */}
      {!activeRide && isOnline && (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <h3 style={{ fontSize: 15 }}>Solicitudes Disponibles en tu Radio ({coverageRadius} km)</h3>
            <span style={{ fontSize: 12, color: 'var(--text-muted)' }}>{availableRides.length} en espera</span>
          </div>

          {availableRides.length === 0 ? (
            <div className="z-card" style={{ textAlign: 'center', padding: '24px 16px', color: 'var(--text-muted)' }}>
              No hay solicitudes pendientes en tu radio de cobertura. Solicita un viaje desde la vista de <strong>Pasajero</strong> para recibirlo aquí.
            </div>
          ) : (
            availableRides.map((r) => (
              <div key={r.id} className="z-card z-card-glow" style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
                <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                  <span className="badge-ev">{r.origin_commune} → {r.destination_commune}</span>
                  <span style={{ fontWeight: 800, color: '#38BDF8', fontSize: 16 }}>
                    ${r.estimated_fare_clp.toLocaleString('es-CL')} CLP
                  </span>
                </div>
                <div style={{ fontSize: 13 }}>
                  <div><strong>Desde:</strong> {r.origin_address}</div>
                  <div><strong>Hasta:</strong> {r.destination_address}</div>
                  <div style={{ fontSize: 12, color: 'var(--text-muted)', marginTop: 4 }}>
                    {r.distance_km} km · ~{r.duration_minutes} min
                  </div>
                </div>
                <button onClick={() => handleAcceptRide(r.id)} className="btn-primary">
                  <CheckCircle2 size={16} />
                  <span>Aceptar Solicitud</span>
                </button>
              </div>
            ))
          )}
        </div>
      )}

      {/* Register EV Modal */}
      {showRegisterEV && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          background: 'rgba(0,0,0,0.85)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          padding: 16,
          zIndex: 100
        }}>
          <form onSubmit={handleRegisterEV} className="z-card" style={{ maxWidth: 400, width: '100%', display: 'flex', flexDirection: 'column', gap: 12 }}>
            <h3 style={{ fontSize: 16 }}>Registrar Auto Eléctrico (BEV)</h3>
            <p style={{ fontSize: 12, color: 'var(--text-secondary)' }}>
              Solo se admiten vehículos 100% eléctricos de batería año 2018 o superior con patente chilena.
            </p>

            <div>
              <label className="z-label">Marca</label>
              <input value={evBrand} onChange={(e) => setEvBrand(e.target.value)} required className="z-input" placeholder="Ej: Tesla, BYD, Hyundai" />
            </div>

            <div>
              <label className="z-label">Modelo</label>
              <input value={evModel} onChange={(e) => setEvModel(e.target.value)} required className="z-input" placeholder="Ej: Model 3, Dolphin, Ioniq 5" />
            </div>

            <div style={{ display: 'flex', gap: 8 }}>
              <div style={{ flex: 1 }}>
                <label className="z-label">Año (≥ 2018)</label>
                <input type="number" min="2018" value={evYear} onChange={(e) => setEvYear(Number(e.target.value))} required className="z-input" />
              </div>
              <div style={{ flex: 1 }}>
                <label className="z-label">Patente Chilena</label>
                <input value={evPlate} onChange={(e) => setEvPlate(e.target.value)} required className="z-input" placeholder="LW-72-91" />
              </div>
            </div>

            <div style={{ display: 'flex', gap: 8 }}>
              <div style={{ flex: 1 }}>
                <label className="z-label">Batería (kWh ≥ 30)</label>
                <input type="number" step="0.1" value={evBattery} onChange={(e) => setEvBattery(Number(e.target.value))} required className="z-input" />
              </div>
              <div style={{ flex: 1 }}>
                <label className="z-label">Capacidad Pasajeros</label>
                <input type="number" min="3" value={evCapacity} onChange={(e) => setEvCapacity(Number(e.target.value))} required className="z-input" />
              </div>
            </div>

            <div style={{ display: 'flex', gap: 8, marginTop: 8 }}>
              <button type="button" onClick={() => setShowRegisterEV(false)} className="btn-secondary" style={{ flex: 1 }}>
                Cancelar
              </button>
              <button type="submit" className="btn-primary" style={{ flex: 1 }}>
                Guardar EV
              </button>
            </div>
          </form>
        </div>
      )}

      {/* Completed Trips & Disclaimer */}
      <div className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
        <h4 style={{ fontSize: 14 }}>Resumen de Operación Conductor</h4>
        <div style={{ fontSize: 13, color: 'var(--text-secondary)' }}>
          Viajes completados: <strong>{completedRides.length}</strong>
        </div>
        <p style={{
          fontSize: 11,
          color: 'var(--text-muted)',
          background: 'var(--bg-surface)',
          padding: 8,
          borderRadius: 8,
          border: '1px solid var(--border-subtle)'
        }}>
          * Nota de Cumplimiento: La visualización de liquidaciones y comisiones reales está desactivada hasta la definición y validación formal del modelo comercial y tributario en Chile.
        </p>
      </div>
    </div>
  );
};
