import React, { useState, useEffect } from 'react';
import {
  ShieldCheck,
  CheckCircle2,
  XCircle,
  FileText,
  Car,
  Settings,
  AlertTriangle
} from 'lucide-react';
import { ApiClient } from '../api';
import { AuditLog, DriverProfile, MarketConfig, User, Vehicle } from '../types';

export const AdminView: React.FC = () => {
  const [pendingDrivers, setPendingDrivers] = useState<Array<{ profile: DriverProfile; user: User; vehicle: Vehicle | null }>>([]);
  const [auditLogs, setAuditLogs] = useState<AuditLog[]>([]);
  const [marketConfig, setMarketConfig] = useState<MarketConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [reviewNotes, setReviewNotes] = useState('');
  const [actionMsg, setActionMsg] = useState<string | null>(null);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const refreshData = async () => {
    try {
      const [pending, logs, config] = await Promise.all([
        ApiClient.listPendingDrivers(),
        ApiClient.listAuditLogs(),
        ApiClient.getMarketConfig(),
      ]);
      setPendingDrivers(pending);
      setAuditLogs(logs);
      setMarketConfig(config);
    } catch (err: any) {
      // ignore
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refreshData();
    const interval = setInterval(refreshData, 4000);
    return () => clearInterval(interval);
  }, []);

  const handleReview = async (driverId: string, status: 'approved' | 'rejected') => {
    setErrorMsg(null);
    setActionMsg(null);
    try {
      const res = await ApiClient.reviewDriver(driverId, status, reviewNotes || undefined);
      setActionMsg(res.message);
      setReviewNotes('');
      refreshData();
    } catch (err: any) {
      setErrorMsg(err.message || 'Error al procesar revisión');
    }
  };

  if (loading) {
    return <div style={{ padding: 20, textAlign: 'center', color: 'var(--text-muted)' }}>Cargando panel de operaciones...</div>;
  }

  return (
    <div style={{ padding: 16, display: 'flex', flexDirection: 'column', gap: 16 }}>
      {/* Status Alerts */}
      {actionMsg && (
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
          <span>{actionMsg}</span>
        </div>
      )}

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

      {/* Header Banner */}
      <div className="z-card" style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
        <div style={{
          width: 40,
          height: 40,
          borderRadius: 10,
          background: 'rgba(245, 158, 11, 0.15)',
          border: '1px solid rgba(245, 158, 11, 0.3)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center'
        }}>
          <ShieldCheck size={22} color="#F59E0B" />
        </div>
        <div>
          <h3 style={{ fontSize: 16 }}>Panel de Operaciones & Cumplimiento</h3>
          <p style={{ fontSize: 12, color: 'var(--text-secondary)' }}>
            Supervisión de flota 100% eléctrica y auditoría Ley EAT en Santiago de Chile
          </p>
        </div>
      </div>

      {/* Pending Drivers Verification Queue */}
      <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <h4 style={{ fontSize: 15, display: 'flex', alignItems: 'center', gap: 6 }}>
            <span>Solicitudes de Conductor Pendientes</span>
            <span className="badge-role badge-role-admin">{pendingDrivers.length}</span>
          </h4>
        </div>

        {pendingDrivers.length === 0 ? (
          <div className="z-card" style={{ textAlign: 'center', padding: '24px 16px', color: 'var(--text-muted)' }}>
            No hay conductores ni vehículos eléctricos pendientes de revisión en este momento.
          </div>
        ) : (
          pendingDrivers.map(({ profile, user, vehicle }) => (
            <div key={profile.id} className="z-card z-card-glow" style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                <div>
                  <h4 style={{ fontSize: 15 }}>{user.full_name}</h4>
                  <div style={{ fontSize: 12, color: 'var(--text-secondary)' }}>{user.email} · {user.phone || 'Sin teléfono'}</div>
                  <div style={{ fontSize: 11, color: 'var(--text-muted)', marginTop: 2 }}>
                    Licencia Clase {profile.license_class} N° {profile.license_number}
                  </div>
                </div>
                <span className="badge-role badge-role-admin">Pendiente Revisión</span>
              </div>

              {/* Electric Vehicle Details */}
              {vehicle ? (
                <div style={{
                  background: 'var(--bg-surface)',
                  borderRadius: 8,
                  padding: 10,
                  border: '1px solid var(--border-subtle)',
                  display: 'flex',
                  alignItems: 'center',
                  gap: 10
                }}>
                  <Car size={20} color="#06B6D4" />
                  <div>
                    <div style={{ fontSize: 13, fontWeight: 600 }}>
                      {vehicle.brand} {vehicle.model} ({vehicle.year}) · Patente: <strong>{vehicle.license_plate}</strong>
                    </div>
                    <div style={{ fontSize: 11, color: '#34D399' }}>
                      Batería: {vehicle.battery_capacity_kwh} kWh · 100% Eléctrico (BEV)
                    </div>
                  </div>
                </div>
              ) : (
                <div style={{ fontSize: 12, color: '#F87171' }}>Sin vehículo eléctrico asociado</div>
              )}

              {/* Review notes input */}
              <input
                type="text"
                placeholder="Notas de auditoría (ej: Licencia y homologación 3CV validadas)"
                value={reviewNotes}
                onChange={(e) => setReviewNotes(e.target.value)}
                className="z-input"
                style={{ fontSize: 12, padding: '8px 10px' }}
              />

              {/* Approve / Reject Actions */}
              <div style={{ display: 'flex', gap: 8 }}>
                <button
                  onClick={() => handleReview(profile.id, 'rejected')}
                  className="btn-danger"
                  style={{ flex: 1, padding: '10px 14px', fontSize: 12 }}
                >
                  <XCircle size={15} />
                  <span>Rechazar</span>
                </button>
                <button
                  onClick={() => handleReview(profile.id, 'approved')}
                  className="btn-primary"
                  style={{ flex: 1, padding: '10px 14px', fontSize: 12 }}
                >
                  <CheckCircle2 size={15} />
                  <span>Aprobar Conductor y EV</span>
                </button>
              </div>
            </div>
          ))
        )}
      </div>

      {/* Market Parameters Viewer */}
      {marketConfig && (
        <div className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <Settings size={16} color="#8B5CF6" />
            <h4 style={{ fontSize: 14 }}>Parámetros de Configuración Santiago</h4>
          </div>
          <div style={{ fontSize: 12, display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
            <div style={{ background: 'var(--bg-surface)', padding: 8, borderRadius: 6 }}>
              <span className="z-label">Tarifa Base</span>
              <strong>${marketConfig.pricing.base_fare_clp} CLP</strong>
            </div>
            <div style={{ background: 'var(--bg-surface)', padding: 8, borderRadius: 6 }}>
              <span className="z-label">Costo por Km</span>
              <strong>${marketConfig.pricing.per_km_clp} CLP</strong>
            </div>
            <div style={{ background: 'var(--bg-surface)', padding: 8, borderRadius: 6 }}>
              <span className="z-label">Costo por Minuto</span>
              <strong>${marketConfig.pricing.per_minute_clp} CLP</strong>
            </div>
            <div style={{ background: 'var(--bg-surface)', padding: 8, borderRadius: 6 }}>
              <span className="z-label">Tarifa Mínima</span>
              <strong>${marketConfig.pricing.minimum_fare_clp} CLP</strong>
            </div>
          </div>
          <div style={{ fontSize: 11, color: 'var(--text-muted)', marginTop: 4 }}>
            Comunas activas: {marketConfig.service_communes.slice(0, 5).join(', ')}...
          </div>
        </div>
      )}

      {/* Audit Logs Viewer */}
      <div className="z-card" style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
          <FileText size={16} color="#06B6D4" />
          <h4 style={{ fontSize: 14 }}>Registro de Auditoría (Audit Logs)</h4>
        </div>

        <div style={{ display: 'flex', flexDirection: 'column', gap: 8, maxHeight: 200, overflowY: 'auto' }}>
          {auditLogs.slice(0, 5).map((log) => (
            <div key={log.id} style={{
              fontSize: 11,
              background: 'var(--bg-surface)',
              padding: 8,
              borderRadius: 6,
              border: '1px solid var(--border-subtle)',
              lineHeight: 1.5
            }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', color: 'var(--text-muted)' }}>
                <strong>{log.action}</strong>
                <span>{new Date(log.created_at).toLocaleTimeString('es-CL')}</span>
              </div>
              <div style={{ color: 'var(--text-secondary)' }}>
                Entidad: {log.target_entity} {log.target_id ? `(${log.target_id.slice(0, 8)}...)` : ''}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
