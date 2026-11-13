import React, { useState, useEffect } from 'react';
import { X, Send, MessageSquare, CheckCircle2 } from 'lucide-react';
import { ApiClient } from '../api';
import { SupportTicket } from '../types';

interface SupportModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const SupportModal: React.FC<SupportModalProps> = ({ isOpen, onClose }) => {
  const [tickets, setTickets] = useState<SupportTicket[]>([]);
  const [subject, setSubject] = useState('');
  const [message, setMessage] = useState('');
  const [sending, setSending] = useState(false);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);

  const loadTickets = async () => {
    try {
      const data = await ApiClient.listTickets();
      setTickets(data);
    } catch (_) {
      // ignore
    }
  };

  useEffect(() => {
    if (isOpen) {
      loadTickets();
    }
  }, [isOpen]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!subject.trim() || !message.trim()) return;

    setSending(true);
    setStatusMsg(null);
    try {
      const res = await ApiClient.createTicket({ subject, message });
      setStatusMsg(res.message);
      setSubject('');
      setMessage('');
      loadTickets();
    } catch (err: any) {
      setStatusMsg(err.message || 'Error al enviar ticket');
    } finally {
      setSending(false);
    }
  };

  if (!isOpen) return null;

  return (
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
      zIndex: 110,
    }}>
      <div className="z-card" style={{ maxWidth: 440, width: '100%', maxHeight: '90vh', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: 14 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <MessageSquare size={18} color="#8B5CF6" />
            <h3 style={{ fontSize: 16 }}>Soporte y Ayuda Zuvio</h3>
          </div>
          <button onClick={onClose} style={{ background: 'transparent', color: 'var(--text-muted)' }}>
            <X size={20} />
          </button>
        </div>

        {statusMsg && (
          <div style={{
            padding: 10,
            borderRadius: 8,
            background: 'rgba(16, 185, 129, 0.15)',
            border: '1px solid rgba(16, 185, 129, 0.3)',
            color: '#34D399',
            fontSize: 12,
            display: 'flex',
            alignItems: 'center',
            gap: 6
          }}>
            <CheckCircle2 size={15} />
            <span>{statusMsg}</span>
          </div>
        )}

        {/* Create ticket form */}
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
          <div>
            <label className="z-label">Asunto</label>
            <input
              type="text"
              placeholder="Ej: Consulta sobre boleta electrónica o viaje"
              value={subject}
              onChange={(e) => setSubject(e.target.value)}
              required
              className="z-input"
            />
          </div>
          <div>
            <label className="z-label">Mensaje o Detalle</label>
            <textarea
              rows={3}
              placeholder="Describe tu consulta para nuestro equipo en Santiago..."
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              required
              className="z-input"
              style={{ resize: 'none' }}
            />
          </div>
          <button type="submit" disabled={sending} className="btn-primary" style={{ padding: '10px 16px', fontSize: 13 }}>
            <Send size={15} />
            <span>{sending ? 'Enviando...' : 'Crear Ticket de Soporte'}</span>
          </button>
        </form>

        {/* Past tickets list */}
        <div style={{ borderTop: '1px solid var(--border-subtle)', paddingTop: 12, display: 'flex', flexDirection: 'column', gap: 8 }}>
          <h4 style={{ fontSize: 13, color: 'var(--text-secondary)' }}>Tus Tickets Anteriores</h4>
          {tickets.length === 0 ? (
            <div style={{ fontSize: 12, color: 'var(--text-muted)', textAlign: 'center', padding: 8 }}>
              No tienes tickets registrados.
            </div>
          ) : (
            tickets.map((t) => (
              <div key={t.id} style={{
                background: 'var(--bg-surface)',
                borderRadius: 8,
                padding: 10,
                border: '1px solid var(--border-subtle)',
                fontSize: 12,
                display: 'flex',
                flexDirection: 'column',
                gap: 4
              }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <strong>{t.subject}</strong>
                  <span className={`badge-role ${t.status === 'resolved' ? 'badge-role-passenger' : 'badge-role-admin'}`} style={{ fontSize: 10 }}>
                    {t.status === 'resolved' ? 'Resuelto' : t.status}
                  </span>
                </div>
                <div style={{ color: 'var(--text-secondary)' }}>{t.message}</div>
                {t.resolution_notes && (
                  <div style={{ color: '#34D399', background: 'rgba(16, 185, 129, 0.1)', padding: 6, borderRadius: 6, marginTop: 4 }}>
                    <strong>Respuesta Operaciones:</strong> {t.resolution_notes}
                  </div>
                )}
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};

