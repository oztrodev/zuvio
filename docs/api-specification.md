# Especificación Técnica de la API Zuvio (REST)

Backend desarrollado en **Rust con Axum y Tokio**.
Prefijo base: `/api/v1`

---

## 1. Endpoints de Salud y Configuración

### `GET /api/v1/health`
Verifica el estado del servicio, versión y modo de operación.
- **Respuesta 200 OK**:
  ```json
  {
    "status": "healthy",
    "version": "0.1.0",
    "service": "zuvio-backend",
    "demo_mode": true,
    "market": "Santiago de Chile",
    "fleet": "100% Electric (BEV)",
    "timestamp": "2026-11-13T01:30:00Z"
  }
  ```

### `GET /api/v1/config/market`
Retorna parámetros del mercado de Santiago de Chile (tarifas demo, comunas activas, advertencias legales).
- **Respuesta 200 OK**:
  ```json
  {
    "currency": "CLP",
    "timezone": "America/Santiago",
    "base_fare_clp": 1400,
    "per_km_clp": 650,
    "per_minute_clp": 150,
    "minimum_fare_clp": 3000,
    "service_communes": [
      "Santiago Centro",
      "Providencia",
      "Las Condes",
      "Vitacura",
      "Ñuñoa",
      "Lo Barnechea",
      "La Reina"
    ],
    "safety_notice": "Tolerancia Cero: Prohibido conducir bajo la influencia del alcohol o sustancias. Prohibido fumar en vehículos."
  }
  ```

---

## 2. Endpoints de Autenticación y Perfil

### `POST /api/v1/auth/demo-session`
Permite alternar sesiones de prueba rápidamente en el frontend (Pasajero, Conductor, Admin).
- **Body**:
  ```json
  {
    "role": "passenger" // "passenger" | "driver" | "admin"
  }
  ```
- **Respuesta 200 OK**:
  ```json
  {
    "user": {
      "id": "00000000-0000-0000-0000-000000000002",
      "email": "pasajero@zuvio.cl",
      "full_name": "Valentina Silva",
      "role": "passenger"
    },
    "token": "demo-token-passenger"
  }
  ```

### `GET /api/v1/profile`
Header: `Authorization: Bearer <token>`
Retorna el perfil del usuario autenticado.

---

## 3. Endpoints de Viajes (Rides)

### `POST /api/v1/rides/estimate`
Calcula distancia, tiempo estimado y tarifa en pesos chilenos (CLP) para una ruta en Santiago.
- **Body**:
  ```json
  {
    "origin_address": "Av. Providencia 1234",
    "origin_commune": "Providencia",
    "origin_lat": -33.4285,
    "origin_lng": -70.6189,
    "destination_address": "Av. Apoquindo 4500",
    "destination_commune": "Las Condes",
    "destination_lat": -33.4150,
    "destination_lng": -70.5790
  }
  ```
- **Respuesta 200 OK**:
  ```json
  {
    "distance_km": 5.4,
    "duration_minutes": 16,
    "estimated_fare_clp": 5200,
    "currency": "CLP",
    "vehicle_propulsion": "BEV",
    "co2_saved_kg": 0.86,
    "is_estimate": true
  }
  ```

### `POST /api/v1/rides`
Crea una solicitud de viaje.
- **Body**: Mismos campos que la estimación + `notes` opcional.
- **Respuesta 201 Created**: Retorna el objeto `Ride` con estado `matching`.

### `GET /api/v1/rides`
Retorna el historial de viajes del usuario (o todos los viajes si el rol es Admin).

### `GET /api/v1/rides/active`
Retorna el viaje activo actual del usuario autenticado (si existe).

### `POST /api/v1/rides/:id/accept`
**Exclusivo para Conductor Aprobado con Vehículo Eléctrico Activo**.
Acepta el viaje de forma atómica evitando doble asignación.

### `POST /api/v1/rides/:id/status`
Actualiza el estado del viaje (`driver_en_route`, `arrived`, `in_progress`, `completed`).
- **Body**:
  ```json
  {
    "status": "driver_en_route"
  }
  ```

### `POST /api/v1/rides/:id/cancel`
Cancela el viaje registrando el motivo.
- **Body**:
  ```json
  {
    "cancellation_reason": "Cambio de planes"
  }
  ```

### `POST /api/v1/rides/:id/rate`
Permite calificar el viaje completado (1 a 5 estrellas) con comentarios.

---

## 4. Endpoints de Conductores

### `POST /api/v1/drivers/register-ev`
Registra un nuevo vehículo eléctrico para el conductor.
- **Body**:
  ```json
  {
    "brand": "Tesla",
    "model": "Model 3",
    "year": 2023,
    "license_plate": "LW-72-91",
    "propulsion": "BEV", // Requerido: Solo BEV
    "battery_capacity_kwh": 60.0,
    "passenger_capacity": 4
  }
  ```
- **Validación**: Rechaza cualquier vehículo con `propulsion != 'BEV'` con código `422 Unprocessable Entity`.

### `POST /api/v1/drivers/availability`
Activa o desactiva la disponibilidad del conductor.
- Requiere que el conductor esté en estado `approved` y con vehículo eléctrico activo.

---

## 5. Endpoints de Administración y Soporte

### `GET /api/v1/admin/pending-drivers`
Lista conductores y vehículos pendientes de verificación manual.

### `POST /api/v1/admin/drivers/:id/review`
Aprueba o rechaza a un conductor con justificación y registro en `audit_logs`.

### `GET /api/v1/support/tickets`
Lista tickets de soporte del usuario (o todos si es admin).

### `POST /api/v1/support/tickets`
Crea un nuevo ticket de soporte.

### `POST /api/v1/admin/tickets/:id/resolve`
Resuelve un ticket de soporte con notas de resolución auditadas.

