import React, { useState } from 'react';
import { ShieldAlert, ChevronDown, ChevronUp, AlertCircle } from 'lucide-react';

export const SafetyBanner: React.FC = () => {
  const [expanded, setExpanded] = useState(false);

  return (
    <div style={{
      margin: '12px 16px 0',
      background: 'rgba(24, 27, 36, 0.95)',
      border: '1px solid rgba(139, 92, 246, 0.25)',
      borderRadius: 12,
      padding: '10px 14px',
      fontSize: 12,
    }}>
      <div
        onClick={() => setExpanded(!expanded)}
        style={{
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
          cursor: 'pointer',
          userSelect: 'none'
        }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: 8, color: '#A78BFA', fontWeight: 600 }}>
          <ShieldAlert size={16} color="#8B5CF6" />
          <span>Seguridad Zuvio · Tolerancia Cero en Santiago</span>
        </div>
        <button style={{ background: 'transparent', color: 'var(--text-muted)' }}>
          {expanded ? <ChevronUp size={16} /> : <ChevronDown size={16} />}
        </button>
      </div>

      {expanded && (
        <div style={{ marginTop: 10, paddingTop: 10, borderTop: '1px solid var(--border-subtle)', color: 'var(--text-secondary)', lineHeight: 1.6 }}>
          <p style={{ display: 'flex', alignItems: 'flex-start', gap: 6, marginBottom: 6 }}>
            <AlertCircle size={15} color="#F59E0B" style={{ flexShrink: 0, marginTop: 2 }} />
            <span>
              <strong>Ley de Tránsito N° 18.290 y Ley Emilia:</strong> Prohibido terminantemente conducir bajo la influencia del alcohol, estupefacientes o cannabis.
            </span>
          </p>
          <p style={{ display: 'flex', alignItems: 'flex-start', gap: 6, marginBottom: 6 }}>
            <AlertCircle size={15} color="#10B981" style={{ flexShrink: 0, marginTop: 2 }} />
            <span>
              <strong>Ambiente 100% libre de humo:</strong> Queda prohibido fumar o consumir tabaco, cannabis o cualquier sustancia al interior de los vehículos.
            </span>
          </p>
          <p style={{ fontSize: 11, color: 'var(--text-muted)', marginTop: 8 }}>
            Zuvio promueve un espacio urbano acogedor y respetuoso para todos en la Región Metropolitana.
          </p>
        </div>
      )}
    </div>
  );
};

