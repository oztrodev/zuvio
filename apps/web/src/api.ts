import {
  AuditLog,
  DriverProfile,
  FareEstimate,
  MarketConfig,
  NearbyDriver,
  Ride,
  Role,
  SupportTicket,
  User,
  Vehicle,
} from './types';

// En producción define VITE_API_URL (ej. https://api.zuvio.cl). En local usa el proxy de Vite (/api).
const API_ORIGIN = ((import.meta as unknown as { env?: Record<string, string | undefined> }).env?.VITE_API_URL ?? '').replace(/\/$/, '');
const API_BASE = `${API_ORIGIN}/api/v1`;

export class ApiClient {
  private static currentRole: Role = 'passenger';
  private static currentUserId: string = '00000000-0000-0000-0000-000000000002';

  public static setSession(role: Role, userId?: string) {
    this.currentRole = role;
    if (userId) {
      this.currentUserId = userId;
    } else {
      if (role === 'admin') {
        this.currentUserId = '00000000-0000-0000-0000-000000000001';
      } else if (role === 'driver') {
        this.currentUserId = '00000000-0000-0000-0000-000000000010';
      } else {
        this.currentUserId = '00000000-0000-0000-0000-000000000002';
      }
    }
    localStorage.setItem('zuvio_role', this.currentRole);
    localStorage.setItem('zuvio_user_id', this.currentUserId);
  }

  public static getSession() {
    const savedRole = localStorage.getItem('zuvio_role') as Role | null;
    const savedUserId = localStorage.getItem('zuvio_user_id');
    if (savedRole) this.currentRole = savedRole;
    if (savedUserId) this.currentUserId = savedUserId;
    return { role: this.currentRole, userId: this.currentUserId };
  }

  private static async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const headers = new Headers(options.headers || {});
    headers.set('Content-Type', 'application/json');
    headers.set('X-User-Role', this.currentRole);
    headers.set('X-User-Id', this.currentUserId);
    headers.set('Authorization', `Bearer demo-token-${this.currentRole}`);

    const res = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers,
    });

    if (!res.ok) {
      let errorMsg = `Error ${res.status}`;
      try {
        const data = await res.json();
        errorMsg = data.detail || data.error || errorMsg;
      } catch (_) {
        // ignore
      }
      throw new Error(errorMsg);
    }

    return res.json();
  }

  // Market Config
  public static async getMarketConfig(): Promise<MarketConfig> {
    return this.request<MarketConfig>('/config/market');
  }

  // Profile
  public static async getProfile(): Promise<{ user: User; driver_profile?: DriverProfile; vehicle?: Vehicle }> {
    return this.request<{ user: User; driver_profile?: DriverProfile; vehicle?: Vehicle }>('/profile');
  }

  // Rides
  public static async estimateRide(payload: {
    origin_address: string;
    origin_commune: string;
    origin_lat: number;
    origin_lng: number;
    destination_address: string;
    destination_commune: string;
    destination_lat: number;
    destination_lng: number;
  }): Promise<FareEstimate> {
    return this.request<FareEstimate>('/rides/estimate', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
  }

  public static async createRide(payload: {
    origin_address: string;
    origin_commune: string;
    origin_lat: number;
    origin_lng: number;
    destination_address: string;
    destination_commune: string;
    destination_lat: number;
    destination_lng: number;
  }): Promise<Ride> {
    return this.request<Ride>('/rides', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
  }

  public static async getActiveRide(): Promise<{ ride: Ride | null; driver?: User; vehicle?: Vehicle }> {
    return this.request<{ ride: Ride | null; driver?: User; vehicle?: Vehicle }>('/rides/active');
  }

  public static async listRides(): Promise<Ride[]> {
    return this.request<Ride[]>('/rides');
  }

  public static async acceptRide(rideId: string): Promise<{ message: string; ride: Ride }> {
    return this.request<{ message: string; ride: Ride }>(`/rides/${rideId}/accept`, {
      method: 'POST',
    });
  }

  public static async updateRideStatus(rideId: string, status: string): Promise<{ message: string; ride: Ride }> {
    return this.request<{ message: string; ride: Ride }>(`/rides/${rideId}/status`, {
      method: 'POST',
      body: JSON.stringify({ status }),
    });
  }

  public static async cancelRide(rideId: string, reason?: string): Promise<{ message: string; ride: Ride }> {
    return this.request<{ message: string; ride: Ride }>(`/rides/${rideId}/cancel`, {
      method: 'POST',
      body: JSON.stringify({ reason }),
    });
  }

  public static async rateRide(rideId: string, score: number, comment?: string): Promise<any> {
    return this.request(`/rides/${rideId}/rate`, {
      method: 'POST',
      body: JSON.stringify({ score, comment }),
    });
  }

  // Driver
  public static async getDriverProfile(): Promise<{ profile: DriverProfile | null; vehicle: Vehicle | null }> {
    return this.request<{ profile: DriverProfile | null; vehicle: Vehicle | null }>('/drivers/me');
  }

  public static async registerEV(payload: {
    brand: string;
    model: string;
    year: number;
    license_plate: string;
    propulsion: 'BEV';
    battery_capacity_kwh: number;
    passenger_capacity: number;
  }): Promise<{ message: string; vehicle: Vehicle }> {
    return this.request<{ message: string; vehicle: Vehicle }>('/drivers/register-ev', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
  }

  public static async toggleAvailability(isOnline: boolean): Promise<{ message: string; is_online: boolean }> {
    return this.request<{ message: string; is_online: boolean }>('/drivers/availability', {
      method: 'POST',
      body: JSON.stringify({ is_online: isOnline }),
    });
  }

  public static async updateCoverageRadius(radiusKm: number): Promise<{ message: string; coverage_radius_km: number }> {
    return this.request<{ message: string; coverage_radius_km: number }>('/drivers/coverage-radius', {
      method: 'POST',
      body: JSON.stringify({ radius_km: radiusKm }),
    });
  }

  public static async getNearbyDrivers(lat: number, lng: number, radiusKm?: number): Promise<NearbyDriver[]> {
    const q = new URLSearchParams({
      lat: lat.toString(),
      lng: lng.toString(),
      ...(radiusKm ? { radius_km: radiusKm.toString() } : {}),
    });
    return this.request<NearbyDriver[]>(`/drivers/nearby?${q.toString()}`);
  }

  public static async getAvailableRides(): Promise<Ride[]> {
    return this.request<Ride[]>('/drivers/available-rides');
  }

  // Admin
  public static async listPendingDrivers(): Promise<Array<{ profile: DriverProfile; user: User; vehicle: Vehicle | null }>> {
    return this.request<Array<{ profile: DriverProfile; user: User; vehicle: Vehicle | null }>>('/admin/pending-drivers');
  }

  public static async reviewDriver(driverId: string, status: 'approved' | 'rejected', notes?: string): Promise<any> {
    return this.request(`/admin/drivers/${driverId}/review`, {
      method: 'POST',
      body: JSON.stringify({ status, notes }),
    });
  }

  public static async listAuditLogs(): Promise<AuditLog[]> {
    return this.request<AuditLog[]>('/admin/audit-logs');
  }

  // Support
  public static async createTicket(payload: { subject: string; message: string; ride_id?: string }): Promise<{ message: string; ticket: SupportTicket }> {
    return this.request<{ message: string; ticket: SupportTicket }>('/support/tickets', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
  }

  public static async listTickets(): Promise<SupportTicket[]> {
    return this.request<SupportTicket[]>('/support/tickets');
  }

  public static async resolveTicket(ticketId: string, resolution_notes: string): Promise<any> {
    return this.request(`/admin/tickets/${ticketId}/resolve`, {
      method: 'POST',
      body: JSON.stringify({ resolution_notes }),
    });
  }
}
