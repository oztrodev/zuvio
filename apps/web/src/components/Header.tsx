import React from 'react';
import { Zap, ShieldCheck, UserCheck, Car, HelpCircle } from 'lucide-react';
import { Role } from '../types';

interface HeaderProps {
  currentRole: Role;
  onRoleChange: (role: Role) => void;
  onOpenSupport: () => void;
}

export const Header: React.FC<HeaderProps> = ({ currentRole, onRoleChange, onOpenSupport }) => {
  return (
    <header style={{
      borderBottom: '1px solid var(--border-subtle)',
      backgroundColor: 'rgba(13, 15, 18, 0.95)',
      backdropFilter: 'blur(12px)',
      position: 'sticky',
      top: 0,
      zIndex: 50,
      padding: '12px 16px'
    }}>
      {/* Top brand row */}
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
          {/* Zuvio Logo */}
          <div style={{
            width: 38,
            height: 38,
            borderRadius: 10,
            background: 'var(--gradient-zuvio)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            boxShadow: '0 0 15px rgba(139, 92, 246, 0.5)'
          }}>
            <Zap size={22} color="#FFFFFF" />
          </div>
          <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
              <span style={{ fontSize: 20, fontWeight: 800, letterSpacing: '-0.03em' }}>
                ZUV<span className="text-gradient">IO</span>
              </span>
              <span className="badge-ev">100% ELÉCTRICO</span>
            </div>
            <p style={{ fontSize: 11, color: 'var(--text-muted)', fontWeight: 500 }}>
              “Muévete a tu manera” · Santiago de Chile
            </p>
          </div>
        </div>

        <button
          onClick={onOpenSupport}
          className="btn-secondary"
          style={{ padding: '8px 12px', fontSize: 12, borderRadius: 20 }}
          title="Centro de Ayuda y Soporte"
        >
          <HelpCircle size={15} />
          <span>Ayuda</span>
        </button>
      </div>

      {/* Role Switcher (Demo Context) */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        background: 'var(--bg-surface)',
        borderRadius: 10,
        padding: 3,
        border: '1px solid var(--border-subtle)'
      }}>
        <button
          onClick={() => onRoleChange('passenger')}
          style={{
            flex: 1,
            padding: '7px 4px',
            fontSize: 12,
            fontWeight: 600,
            borderRadius: 8,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 5,
            background: currentRole === 'passenger' ? 'var(--gradient-zuvio)' : 'transparent',
            color: currentRole === 'passenger' ? '#FFFFFF' : 'var(--text-secondary)',
          }}
        >
          <UserCheck size={14} />
          <span>Pasajero</span>
        </button>

        <button
          onClick={() => onRoleChange('driver')}
          style={{
            flex: 1,
            padding: '7px 4px',
            fontSize: 12,
            fontWeight: 600,
            borderRadius: 8,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 5,
            background: currentRole === 'driver' ? 'var(--gradient-zuvio)' : 'transparent',
            color: currentRole === 'driver' ? '#FFFFFF' : 'var(--text-secondary)',
          }}
        >
          <Car size={14} />
          <span>Conductor</span>
        </button>

        <button
          onClick={() => onRoleChange('admin')}
          style={{
            flex: 1,
            padding: '7px 4px',
            fontSize: 12,
            fontWeight: 600,
            borderRadius: 8,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 5,
            background: currentRole === 'admin' ? 'var(--gradient-zuvio)' : 'transparent',
            color: currentRole === 'admin' ? '#FFFFFF' : 'var(--text-secondary)',
          }}
        >
          <ShieldCheck size={14} />
          <span>Operaciones</span>
        </button>
      </div>
    </header>
  );
};

