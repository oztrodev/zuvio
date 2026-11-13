import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { SafetyBanner } from './components/SafetyBanner';
import { PassengerView } from './components/PassengerView';
import { DriverView } from './components/DriverView';
import { AdminView } from './components/AdminView';
import { SupportModal } from './components/SupportModal';
import { ApiClient } from './api';
import { Role } from './types';
import { Zap } from 'lucide-react';

export const App: React.FC = () => {
  const [currentRole, setCurrentRole] = useState<Role>('passenger');
  const [isSupportOpen, setIsSupportOpen] = useState(false);

  useEffect(() => {
    const session = ApiClient.getSession();
    setCurrentRole(session.role);
  }, []);

  const handleRoleChange = (newRole: Role) => {
    setCurrentRole(newRole);
    ApiClient.setSession(newRole);
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
      {/* Top sticky brand & role bar */}
      <Header
        currentRole={currentRole}
        onRoleChange={handleRoleChange}
        onOpenSupport={() => setIsSupportOpen(true)}
      />

      {/* Safety & Chilean Zero Tolerance Banner */}
      <SafetyBanner />

      {/* Main role view */}
      <main style={{ flex: 1, paddingBottom: 24 }}>
        {currentRole === 'passenger' && <PassengerView />}
        {currentRole === 'driver' && <DriverView />}
        {currentRole === 'admin' && <AdminView />}
      </main>

      {/* Support & Help Modal */}
      <SupportModal
        isOpen={isSupportOpen}
        onClose={() => setIsSupportOpen(false)}
      />

      {/* Mobile Footer */}
      <footer style={{
        padding: '16px',
        borderTop: '1px solid var(--border-subtle)',
        fontSize: 11,
        color: 'var(--text-muted)',
        textAlign: 'center',
        background: 'var(--bg-main)',
        display: 'flex',
        flexDirection: 'column',
        gap: 6
      }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 6, color: 'var(--text-secondary)' }}>
          <Zap size={13} color="#8B5CF6" />
          <span>Zuvio Chile — Movilidad 100% Eléctrica (BEV)</span>
        </div>
        <div>
          Santiago de Chile · Providencia, Las Condes, Vitacura, Stgo Centro, Ñuñoa
        </div>
        <div style={{ color: 'var(--text-muted)', fontSize: 10 }}>
          Cumplimiento Ley EAT N° 21.553 · Valores y comisiones en modo demostración
        </div>
      </footer>
    </div>
  );
};

export default App;
