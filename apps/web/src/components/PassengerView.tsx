import React, { useState, useEffect } from 'react';
import {
  Navigation,
  Leaf,
  CheckCircle2,
  XCircle,
  Star,
  Compass,
  AlertTriangle,
  History,
  Info,
  Car,
  Radar
} from 'lucide-react';
import { ApiClient } from '../api';
import { FareEstimate, NearbyDriver, Ride, User, Vehicle } from '../types';

const SANTIAGO_PRESETS = [
  { name: 'Costanera Center (Providencia)', commune: 'Providencia', lat: -33.4178, lng: -70.6065 },
  { name: 'Metro Manuel Montt (Providencia)', commune: 'Providencia', lat: -33.4285, lng: -70.6189 },
  { name: 'Parque Araucano (Las Condes)', commune: 'Las Condes', lat: -33.4022, lng: -70.5732 },
  { name: 'Metro El Golf (Las Condes)', commune: 'Las Condes', lat: -33.4170, lng: -70.5960 },
  { name: 'Plaza Baquedano / Italia (Stgo Centro)', commune: 'Santiago Centro', lat: -33.4372, lng: -70.6347 },
  { name: 'Barrio Italia (Ñuñoa)', commune: 'Ñuñoa', lat: -33.4475, lng: -70.6270 },
  { name: 'Aeropuerto AMB (Pudahuel)', commune: 'Pudahuel', lat: -33.3930, lng: -70.7950 },
];

export const PassengerView: React.FC = () => {
  const [originIndex, setOriginIndex] = useState(1);
  const [destIndex, setDestIndex] = useState(0);
  const [estimate, setEstimate] = useState<FareEstimate | null>(null);
  const [estimating, setEstimating] = useState(false);
  const [requesting, setRequesting] = useState(false);
  
  const [activeRide, setActiveRide] = useState<Ride | null>(null);
  const [assignedDriver, setAssignedDriver] = useState<User | null>(null);
  const [assignedVehicle, setAssignedVehicle] = useState<Vehicle | null>(null);
  const [nearbyDrivers, setNearbyDrivers] = useState<NearbyDriver[]>([]);

  const [ratingScore, setRatingScore] = useState(5);
  const [ratingComment, setRatingComment] = useState('');
  const [ratingSubmitted, setRatingSubmitted] = useState(false);
  const [showCancelModal, setShowCancelModal] = useState(false);
  const [cancelReason, setCancelReason] = useState('Cambio de planes');

  const [ridesHistory, setRidesHistory] = useState<Ride[]>([]);
  const [showHistory, setShowHistory] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const refreshActiveRide = async () => {
    try {
      const data = await ApiClient.getActiveRide();
      setActiveRide(data.ride);
      if (data.driver) setAssignedDriver(data.driver);
      if (data.vehicle) setAssignedVehicle(data.vehicle);
    } catch (_) {
      // ignore
    }
  };

  const loadHistory = async () => {
    try {
      const list = await ApiClient.listRides();
      setRidesHistory(list);
    } catch (_) {
      // ignore
    }
  };

  const loadNearbyDrivers = async () => {
    try {
      const origin = SANTIAGO_PRESETS[originIndex];
      const drivers = await ApiClient.getNearbyDrivers(origin.lat, origin.lng, 15.0);
      setNearbyDrivers(drivers);
    } catch (_) {
      // ignore
    }
  };

  useEffect(() => {
    refreshActiveRide();
    loadHistory();
    loadNearbyDrivers();
    const interval = setInterval(() => {
      refreshActiveRide();
      loadNearbyDrivers();
    }, 4000);
    return () => clearInterval(interval);
  }, [originIndex]);

  useEffect(() => {
    const fetchEstimate = async () => {
      setEstimating(true);
      setErrorMsg(null);
      try {
        const origin = SANTIAGO_PRESETS[originIndex];
        const dest = SANTIAGO_PRESETS[destIndex];
        const est = await ApiClient.estimateRide({
          origin_address: origin.name,
          origin_commune: origin.commune,
          origin_lat: origin.lat,
          origin_lng: origin.lng,
          destination_address: dest.name,
          destination_commune: dest.commune,
          destination_lat: dest.lat,
          destination_lng: dest.lng,
        });
        setEstimate(est);
      } catch (err: any) {
        setErrorMsg(err.message || 'Error al calcular estimación');
      } finally {
        setEstimating(false);
      }
    };

    fetchEstimate();
  }, [originIndex, destIndex]);

  const handleRequestRide = async () => {
    setRequesting(true);
    setErrorMsg(null);
    try {
      const origin = SANTIAGO_PRESETS[originIndex];
      const dest = SANTIAGO_PRESETS[destIndex];
      const newRide = await ApiClient.createRide({
        origin_address: origin.name,
        origin_commune: origin.commune,
        origin_lat: origin.lat,
        origin_lng: origin.lng,
        destination_address: dest.name,
        destination_commune: dest.commune,
        destination_lat: dest.lat,
        destination_lng: dest.lng,
      });
      setActiveRide(newRide);
      setRatingSubmitted(false);
    } catch (err: any) {
      setErrorMsg(err.message || 'No se pudo crear la solicitud de viaje');
    } finally {
      setRequesting(false);
    }
  };

  const handleCancelRide = async () => {
    if (!activeRide) return;
    try {
      await ApiClient.cancelRide(activeRide.id, cancelReason);
      setShowCancelModal(false);
      refreshActiveRide();
      loadHistory();
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al cancelar');
    }
  };

  const handleRateRide = async () => {
    if (!activeRide) return;
    try {
      await ApiClient.rateRide(activeRide.id, ratingScore, ratingComment);
      setRatingSubmitted(true);
      setTimeout(() => {
        setActiveRide(null);
        refreshActiveRide();
        loadHistory();
      }, 1500);
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al enviar calificación');
    }
  };

  return (
    <div style={{ padding: '16px', display: 'flex', flexDirection: 'column', gap: 16 }}>
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

      {/* Tab bar */}
      <div style={{ display: 'flex', gap: 8 }}>
        <button
          onClick={() => setShowHistory(false)}
          className={!showHistory ? 'btn-primary' : 'btn-secondary'}
          style={{ flex: 1, padding: '10px 14px', fontSize: 13 }}
        >
          <Compass size={16} />
          <span>Pedir Viaje</span>
        </button>
        <button
          onClick={() => { setShowHistory(true); loadHistory(); }}
          className={showHistory ? 'btn-primary' : 'btn-secondary'}
          style={{ flex: 1, padding: '10px 14px', fontSize: 13 }}
        >
          <History size={16} />
          <span>Historial ({ridesHistory.length})</span>
        </button>
      </div>

      {showHistory ? (
        /* History View */
        <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
          <h3 style={{ fontSize: 16 }}>Tus viajes Zuvio en Santiago</h3>
          {ridesHistory.length === 0 ? (
            <div className="z-card" style={{ textAlign: 'center', padding: '30px 20px', color: 'var(--text-muted)' }}>
              No tienes viajes anteriores registrados aún.
            </div>
          ) : (
            ridesHistory.map((r) => (
              <div key={r.id} className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <span style={{ fontSize: 12, color: 'var(--text-muted)' }}>
                    {new Date(r.created_at).toLocaleDateString('es-CL', {
                      day: 'numeric',
                      month: 'short',
                      hour: '2-digit',
                      minute: '2-digit',
                    })}
                  </span>
                  <span className={`badge-role ${r.status === 'completed' ? 'badge-role-passenger' : 'badge-role-admin'}`}>
                    {r.status === 'completed' ? 'Completado' : r.status}
                  </span>
                </div>
                <div style={{ fontSize: 13, fontWeight: 600 }}>
                  {r.origin_address} → {r.destination_address}
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: 12, color: 'var(--text-secondary)' }}>
                  <span>{r.distance_km} km · {r.duration_minutes} min</span>
                  <span style={{ fontWeight: 700, color: '#38BDF8' }}>
                    ${(r.final_fare_clp || r.estimated_fare_clp).toLocaleString('es-CL')} CLP
                  </span>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: 4, fontSize: 11, color: '#34D399' }}>
                  <Leaf size={13} />
                  <span>Ahorro estimado de {r.co2_saved_kg} kg de CO2</span>
                </div>
              </div>
            ))
          )}
        </div>
      ) : activeRide ? (
        /* Active Ride Flow */
        <div className="z-card z-card-glow" style={{ display: 'flex', flexDirection: 'column', gap: 14 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
              <span className="badge-ev">VIAJE ACTIVO</span>
              <span style={{ fontSize: 12, color: 'var(--text-muted)' }}>
                {activeRide.origin_commune} → {activeRide.destination_commune}
              </span>
            </div>
            {activeRide.status !== 'completed' && activeRide.status !== 'cancelled' && (
              <button
                onClick={() => setShowCancelModal(true)}
                style={{
                  background: 'transparent',
                  color: '#F87171',
                  fontSize: 12,
                  fontWeight: 600,
                  display: 'flex',
                  alignItems: 'center',
                  gap: 4
                }}
              >
                <XCircle size={15} />
                <span>Cancelar</span>
              </button>
            )}
          </div>

          {/* Status Tracker */}
          <div style={{
            background: 'var(--bg-surface)',
            borderRadius: 12,
            padding: 14,
            border: '1px solid var(--border-subtle)',
            textAlign: 'center'
          }}>
            {activeRide.status === 'matching' && (
              <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 8 }}>
                <div style={{
                  width: 50,
                  height: 50,
                  borderRadius: '50%',
                  background: 'var(--gradient-glow)',
                  border: '2px solid var(--violet-electric)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                }} className="animate-pulse-glow">
                  <Navigation size={24} color="#8B5CF6" />
                </div>
                <h4 style={{ fontSize: 15 }}>Conectando con un auto eléctrico...</h4>
                <p style={{ fontSize: 12, color: 'var(--text-secondary)' }}>
                  Buscando conductor con vehículo 100% BEV en Santiago
                </p>
                <p style={{ fontSize: 11, color: 'var(--text-muted)' }}>
                  (Puedes probar aceptar este viaje cambiando al rol de <strong>Conductor</strong>)
                </p>
              </div>
            )}

            {activeRide.status === 'accepted' && (
              <div>
                <h4 style={{ fontSize: 15, color: '#38BDF8' }}>¡Viaje Aceptado!</h4>
                <p style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 4 }}>
                  El conductor se está preparando para salir a tu encuentro.
                </p>
              </div>
            )}

            {activeRide.status === 'driver_en_route' && (
              <div>
                <h4 style={{ fontSize: 15, color: '#A78BFA' }}>Conductor en camino</h4>
                <p style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 4 }}>
                  Llegando a <strong>{activeRide.origin_address}</strong>
                </p>
              </div>
            )}

            {activeRide.status === 'arrived' && (
              <div>
                <h4 style={{ fontSize: 15, color: '#34D399' }}>¡El conductor ha llegado!</h4>
                <p style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 4 }}>
                  Tu auto eléctrico te espera en el punto de encuentro.
                </p>
              </div>
            )}

            {activeRide.status === 'in_progress' && (
              <div>
                <h4 style={{ fontSize: 15, color: '#38BDF8' }}>Viaje en curso</h4>
                <p style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 4 }}>
                  Rumbo a <strong>{activeRide.destination_address}</strong>
                </p>
              </div>
            )}

            {activeRide.status === 'completed' && (
              <div>
                <div style={{ display: 'flex', justifyContent: 'center', marginBottom: 6 }}>
                  <CheckCircle2 size={32} color="#10B981" />
                </div>
                <h4 style={{ fontSize: 16, color: '#34D399' }}>¡Has llegado a tu destino!</h4>
                <p style={{ fontSize: 13, color: 'var(--text-secondary)', marginTop: 4 }}>
                  Tarifa final: <strong>${(activeRide.final_fare_clp || activeRide.estimated_fare_clp).toLocaleString('es-CL')} CLP</strong>
                </p>
              </div>
            )}
          </div>

          {assignedDriver && assignedVehicle && (
            <div style={{
              background: 'var(--bg-surface)',
              borderRadius: 12,
              padding: 12,
              border: '1px solid var(--border-subtle)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'space-between'
            }}>
              <div>
                <div style={{ fontSize: 14, fontWeight: 700 }}>{assignedDriver.full_name}</div>
                <div style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 2 }}>
                  {assignedVehicle.brand} {assignedVehicle.model} ({assignedVehicle.year})
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginTop: 4 }}>
                  <span style={{
                    background: '#0F172A',
                    border: '1px solid #475569',
                    borderRadius: 4,
                    padding: '2px 6px',
                    fontSize: 11,
                    fontWeight: 700,
                    letterSpacing: '0.05em'
                  }}>
                    {assignedVehicle.license_plate}
                  </span>
                  <span className="badge-ev" style={{ fontSize: 10 }}>
                    {assignedVehicle.battery_capacity_kwh} kWh
                  </span>
                </div>
              </div>
              <div style={{ textAlign: 'right' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 3, justifyContent: 'flex-end', color: '#FBBF24', fontWeight: 700 }}>
                  <Star size={14} fill="#FBBF24" />
                  <span>4.96</span>
                </div>
                <div style={{ fontSize: 11, color: 'var(--text-muted)', marginTop: 4 }}>
                  Conductor Certificado
                </div>
              </div>
            </div>
          )}

          {activeRide.status === 'completed' && !ratingSubmitted && (
            <div style={{
              marginTop: 10,
              padding: 14,
              background: 'var(--bg-surface)',
              borderRadius: 12,
              border: '1px solid rgba(139, 92, 246, 0.3)'
            }}>
              <h4 style={{ fontSize: 14, marginBottom: 8, textAlign: 'center' }}>
                Califica tu experiencia eléctrica
              </h4>
              <div style={{ display: 'flex', justifyContent: 'center', gap: 8, marginBottom: 12 }}>
                {[1, 2, 3, 4, 5].map((s) => (
                  <button
                    key={s}
                    onClick={() => setRatingScore(s)}
                    style={{ background: 'transparent', padding: 4 }}
                  >
                    <Star
                      size={26}
                      fill={s <= ratingScore ? '#FBBF24' : 'transparent'}
                      color={s <= ratingScore ? '#FBBF24' : 'var(--text-muted)'}
                    />
                  </button>
                ))}
              </div>
              <input
                type="text"
                placeholder="Comentarios sobre el viaje o vehículo eléctrico..."
                value={ratingComment}
                onChange={(e) => setRatingComment(e.target.value)}
                className="z-input"
                style={{ marginBottom: 10 }}
              />
              <button onClick={handleRateRide} className="btn-primary">
                Enviar Calificación
              </button>
            </div>
          )}

          {ratingSubmitted && (
            <div style={{ textAlign: 'center', color: '#34D399', fontSize: 13, padding: 8 }}>
              ¡Gracias por calificar tu viaje en Zuvio!
            </div>
          )}
        </div>
      ) : (
        /* Ride Request Configuration Flow */
        <div style={{ display: 'flex', flexDirection: 'column', gap: 14 }}>
          {/* Route Selector */}
          <div className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
            <h3 style={{ fontSize: 15 }}>¿Hacia dónde te mueves en Santiago?</h3>

            <div>
              <label className="z-label">Punto de Partida (Origen)</label>
              <select
                value={originIndex}
                onChange={(e) => setOriginIndex(Number(e.target.value))}
                className="z-input"
                style={{ cursor: 'pointer' }}
              >
                {SANTIAGO_PRESETS.map((p, idx) => (
                  <option key={idx} value={idx}>
                    📍 {p.name}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="z-label">Punto de Llegada (Destino)</label>
              <select
                value={destIndex}
                onChange={(e) => setDestIndex(Number(e.target.value))}
                className="z-input"
                style={{ cursor: 'pointer' }}
              >
                {SANTIAGO_PRESETS.map((p, idx) => (
                  <option key={idx} value={idx} disabled={idx === originIndex}>
                    🏁 {p.name}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Realtime Nearby Drivers Radar Widget */}
          <div className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <Radar size={16} color="#06B6D4" className="animate-spin" style={{ animationDuration: '4s' }} />
                <h4 style={{ fontSize: 14 }}>Choferes Eléctricos Cercanos en Vivo</h4>
              </div>
              <span className="badge-ev">{nearbyDrivers.length} disponibles</span>
            </div>

            {nearbyDrivers.length === 0 ? (
              <div style={{ fontSize: 12, color: 'var(--text-muted)', textAlign: 'center', padding: '10px 0' }}>
                Buscando autos eléctricos en tu zona de Santiago...
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                {nearbyDrivers.map((driver) => (
                  <div key={driver.driver_id} style={{
                    background: 'var(--bg-surface)',
                    border: '1px solid var(--border-subtle)',
                    borderRadius: 10,
                    padding: '8px 12px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between'
                  }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                      <div style={{
                        width: 32,
                        height: 32,
                        borderRadius: 8,
                        background: 'rgba(6, 182, 212, 0.15)',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center'
                      }}>
                        <Car size={16} color="#06B6D4" />
                      </div>
                      <div>
                        <div style={{ fontSize: 13, fontWeight: 700 }}>
                          {driver.vehicle_brand} {driver.vehicle_model}
                        </div>
                        <div style={{ fontSize: 11, color: 'var(--text-muted)' }}>
                          {driver.full_name} · ★ {driver.rating_avg} ({driver.total_trips} viajes)
                        </div>
                      </div>
                    </div>

                    <div style={{ textAlign: 'right' }}>
                      <div style={{ fontSize: 13, fontWeight: 800, color: '#34D399' }}>
                        ~{driver.estimated_arrival_minutes} min
                      </div>
                      <div style={{ fontSize: 10, color: 'var(--text-secondary)' }}>
                        a {driver.distance_km} km (radio {driver.coverage_radius_km} km)
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Fare Estimation & EV Breakdown Card */}
          {estimate && (
            <div className="z-card z-card-glow" style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <span className="badge-ev">ESTIMACIÓN EN SANTIAGO</span>
                <span style={{ fontSize: 11, color: 'var(--text-muted)' }}>
                  {estimate.is_minimum_applied ? 'Tarifa mínima aplicada' : 'Cálculo transparente'}
                </span>
              </div>

              <div style={{ display: 'flex', alignItems: 'baseline', justifyContent: 'space-between' }}>
                <div>
                  <span style={{ fontSize: 26, fontWeight: 800 }} className="text-gradient">
                    ${estimate.total_fare_clp.toLocaleString('es-CL')}
                  </span>
                  <span style={{ fontSize: 14, color: 'var(--text-secondary)', marginLeft: 4 }}>CLP</span>
                </div>
                <div style={{ textAlign: 'right', fontSize: 12, color: 'var(--text-secondary)' }}>
                  <div>{estimate.distance_km} km · ~{estimate.duration_minutes} min</div>
                </div>
              </div>

              {/* Eco footprint */}
              <div style={{
                background: 'rgba(16, 185, 129, 0.1)',
                border: '1px solid rgba(16, 185, 129, 0.25)',
                borderRadius: 8,
                padding: '8px 10px',
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                fontSize: 12,
                color: '#34D399'
              }}>
                <Leaf size={16} />
                <span>
                  Ahorras <strong>{estimate.co2_saved_kg} kg de CO2</strong> comparado con un auto a gasolina.
                </span>
              </div>

              <div style={{ fontSize: 11, color: 'var(--text-muted)', display: 'flex', justifyContent: 'space-between' }}>
                <span>Base: ${estimate.base_fare_clp} CLP</span>
                <span>Distancia: ${estimate.distance_fare_clp} CLP</span>
                <span>Tiempo: ${estimate.time_fare_clp} CLP</span>
              </div>

              <div style={{
                fontSize: 11,
                color: 'var(--text-muted)',
                fontStyle: 'italic',
                borderTop: '1px solid var(--border-subtle)',
                paddingTop: 6,
                display: 'flex',
                alignItems: 'center',
                gap: 4
              }}>
                <Info size={13} />
                <span>{estimate.provider_note}</span>
              </div>

              <button
                onClick={handleRequestRide}
                disabled={requesting || estimating}
                className="btn-primary"
                style={{ marginTop: 6 }}
              >
                <Navigation size={18} />
                <span>{requesting ? 'Solicitando...' : 'Pedir Zuvio Eléctrico'}</span>
              </button>
            </div>
          )}
        </div>
      )}

      {/* Cancellation Modal */}
      {showCancelModal && (
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
          <div className="z-card" style={{ maxWidth: 380, width: '100%', display: 'flex', flexDirection: 'column', gap: 14 }}>
            <h4 style={{ fontSize: 16, color: '#F87171' }}>¿Deseas cancelar el viaje?</h4>
            <p style={{ fontSize: 13, color: 'var(--text-secondary)' }}>
              Política de cancelación: Cancelación gratuita si el conductor aún no llega. Si cancelas después de 3 minutos de su llegada, podría aplicar cargo configurable.
            </p>
            <div>
              <label className="z-label">Motivo de cancelación</label>
              <select
                value={cancelReason}
                onChange={(e) => setCancelReason(e.target.value)}
                className="z-input"
              >
                <option value="Cambio de planes">Cambio de planes</option>
                <option value="El conductor demoró más de lo esperado">El conductor demoró más de lo esperado</option>
                <option value="Solicité por error">Solicité por error</option>
                <option value="Otro motivo">Otro motivo</option>
              </select>
            </div>
            <div style={{ display: 'flex', gap: 8 }}>
              <button onClick={() => setShowCancelModal(false)} className="btn-secondary" style={{ flex: 1 }}>
                Volver
              </button>
              <button onClick={handleCancelRide} className="btn-danger" style={{ flex: 1 }}>
                Confirmar Cancelación
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
