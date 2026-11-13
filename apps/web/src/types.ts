export type Role = 'passenger' | 'driver' | 'admin';

export interface User {
  id: string;
  email: string;
  full_name: string;
  phone?: string;
  role: Role;
  avatar_url?: string;
  is_active: boolean;
}

export type DriverStatus = 'pending_review' | 'approved' | 'rejected' | 'suspended';

export interface DriverProfile {
  id: string;
  user_id: string;
  status: DriverStatus;
  license_number: string;
  license_class: string;
  background_check_status: string;
  rating_avg: number;
  total_trips: number;
  is_online: boolean;
  coverage_radius_km: number;
  current_lat?: number;
  current_lng?: number;
  reviewed_at?: string;
  review_notes?: string;
}

export interface NearbyDriver {
  driver_id: string;
  user_id: string;
  full_name: string;
  rating_avg: number;
  total_trips: number;
  latitude: number;
  longitude: number;
  coverage_radius_km: number;
  distance_km: number;
  estimated_arrival_minutes: number;
  vehicle_brand: string;
  vehicle_model: string;
  license_plate: string;
  battery_kwh: number;
}

export type PropulsionType = 'BEV';
export type VehicleStatus = 'pending_review' | 'approved' | 'rejected' | 'inactive';

export interface Vehicle {
  id: string;
  driver_id: string;
  brand: string;
  model: string;
  year: number;
  license_plate: string;
  propulsion: PropulsionType;
  battery_capacity_kwh: number;
  passenger_capacity: number;
  status: VehicleStatus;
  is_active: boolean;
  verified_at?: string;
}

export type RideStatus =
  | 'requested'
  | 'matching'
  | 'accepted'
  | 'driver_en_route'
  | 'arrived'
  | 'in_progress'
  | 'completed'
  | 'cancelled'
  | 'expired';

export interface Ride {
  id: string;
  passenger_id: string;
  driver_id?: string;
  vehicle_id?: string;
  status: RideStatus;
  origin_address: string;
  origin_commune: string;
  origin_lat: number;
  origin_lng: number;
  destination_address: string;
  destination_commune: string;
  destination_lat: number;
  destination_lng: number;
  distance_km: number;
  duration_minutes: number;
  estimated_fare_clp: number;
  final_fare_clp?: number;
  co2_saved_kg: number;
  cancellation_reason?: string;
  created_at: string;
  updated_at: string;
  accepted_at?: string;
  driver_arrived_at?: string;
  started_at?: string;
  completed_at?: string;
}

export interface FareEstimate {
  distance_km: number;
  duration_minutes: number;
  base_fare_clp: number;
  distance_fare_clp: number;
  time_fare_clp: number;
  total_fare_clp: number;
  currency: string;
  co2_saved_kg: number;
  is_minimum_applied: boolean;
  provider_note: string;
}

export interface SupportTicket {
  id: string;
  user_id: string;
  ride_id?: string;
  subject: string;
  message: string;
  status: 'open' | 'in_progress' | 'resolved' | 'closed';
  resolution_notes?: string;
  created_at: string;
}

export interface AuditLog {
  id: string;
  actor_id?: string;
  action: string;
  target_entity: string;
  target_id?: string;
  details: Record<string, unknown>;
  created_at: string;
}

export interface MarketConfig {
  market: string;
  currency: string;
  timezone: string;
  pricing: {
    base_fare_clp: number;
    per_km_clp: number;
    per_minute_clp: number;
    minimum_fare_clp: number;
    cancellation_penalty_clp: number;
  };
  service_communes: string[];
  safety_guideline: string;
}
