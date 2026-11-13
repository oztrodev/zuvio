# Zuvio — “Muévete a tu manera”

Plataforma de solicitud de viajes urbanos (ride-hailing) nacida en **Santiago de Chile**, que opera de manera exclusiva con **vehículos 100% eléctricos (BEV - Battery Electric Vehicles)**.

---

## 1. Resumen de la Plataforma y Características

- **Flota 100% Eléctrica (BEV)**: Validación estricta en múltiples capas (Postgres CHECK, validadores de dominio y filtros de operación). No se admiten vehículos híbridos ni de combustión interna.
- **Mercado Inicial**: Región Metropolitana de Santiago de Chile (Providencia, Las Condes, Vitacura, Santiago Centro, Ñuñoa, Lo Barnechea).
- **Tarificación Transparente en CLP**: Algoritmo de cálculo en pesos chilenos ($1.400 base + $650/km + $150/min con redondeo a $100 y tarifa mínima de $3.000 CLP). Muestra de métricas ecológicas de ahorro de CO2 en kg.
- **Máquina de Estados Concurrente**: Previene condiciones de carrera y doble asignación mediante asignación atómica (`accept_ride_atomic`). Ciclo de vida: `requested` → `matching` → `accepted` → `driver_en_route` → `arrived` → `in_progress` → `completed` / `cancelled` / `expired`.
- **Tres Roles Integrados con Cambio Rápido**:
  1. **Pasajero**: Solicita viajes en Santiago, ve estimación de ruta y tarifa en CLP, monitorea conductor en vivo, cancela bajo política y califica viajes completados.
  2. **Conductor**: Onboarding y registro de auto eléctrico (validación de año ≥ 2018, batería ≥ 30 kWh y patente chilena), cambio de disponibilidad online/offline (bloqueado si no está aprobado), recepción y aceptación de viajes, actualización paso a paso del viaje.
  3. **Operaciones / Soporte (Admin)**: Cola de aprobación manual de conductores y autos eléctricos bajo normativa Ley EAT, resolución de tickets de soporte y visor de auditoría en tiempo real.
- **Seguridad Vial & Normativa**: Banner de Tolerancia Cero al volante (Ley N° 18.290 y Ley Emilia). Prohibición absoluta de fumar o consumir sustancias dentro del vehículo.
- **SEO & Geo para Santiago**: Metadatos geoespaciales (`geo.region: CL-RM`, `geo.placename: Santiago`, `geo.position: -33.4489;-70.6693`), OpenGraph, Twitter Cards, `robots.txt`, `sitemap.xml` y Schema.org JSON-LD para indexación en Chile.

---

## 2. Stack Tecnológico

| Capa | Tecnologías |
|---|---|
| **Frontend** | React 18, TypeScript, Vite, Lucide Icons, CSS Custom Properties (Dark graphite `#0D0F12`, acentos violeta eléctrico `#8B5CF6` y cian `#06B6D4`), PWA responsive mobile-first. |
| **Backend** | Rust 1.80+ / 1.96, Axum 0.7, Tokio (async runtime), Tower HTTP (CORS, Trace), Serde, UUID, Chrono. |
| **Persistencia** | Supabase Postgres (esquema SQL, restricciones, triggers de auditoría, políticas RLS en todas las tablas) + Repositorio en memoria thread-safe para desarrollo y modo demo sin dependencias externas. |
| **Comunicaciones** | Proveedor de correo transaccional Brevo (Sendinblue REST API v3) con reintentos y fallback a `MockEmailProvider` local estructurado. |
| **Geolocalización y Pagos** | Adaptador de mapas para Santiago de Chile y adaptador sandbox `MockPaymentProvider` (sin cobros bancarios reales). |

---

## 3. Estructura del Repositorio

```text
zuvio/
├── apps/
│   └── web/                   # Frontend React + TypeScript + Vite PWA
│       ├── public/            # robots.txt, sitemap.xml, manifest.json
│       ├── src/
│       │   ├── components/    # Header, SafetyBanner, PassengerView, DriverView, AdminView, SupportModal
│       │   ├── api.ts         # Cliente API con tipado fuerte y autenticación por rol
│       │   ├── types.ts       # Definiciones de dominio TypeScript
│       │   ├── index.css      # Sistema de diseño Zuvio (Mobile-First)
│       │   └── App.tsx        # Contenedor principal con alternador de roles
│       ├── index.html         # SEO Santiago de Chile, Geo tags, OpenGraph, JSON-LD
│       └── package.json
├── services/
│   └── api/                   # Backend Rust Axum & Tokio
│       ├── src/
│       │   ├── auth/          # Extractor de autenticación y AppState
│       │   ├── config.rs      # Variables de entorno y tarifas de Santiago
│       │   ├── domain/        # Modelos, máquina de estados, validador EV, cálculo CLP
│       │   ├── integrations/  # Brevo Email, Mock Email, MapProvider, MockPaymentProvider
│       │   ├── repository/    # Traits de persistencia, InMemoryRepository con datos demo
│       │   ├── routes/        # Rutas de viajes, conductores, admin, soporte, salud
│       │   ├── lib.rs         # Módulos exportados para pruebas
│       │   └── main.rs        # Servidor HTTP Axum con CORS y logging estructurado
│       ├── tests/             # Suite de pruebas de integración de Rust
│       └── Cargo.toml
├── supabase/
│   └── migrations/
│       ├── 00001_initial_schema.sql  # Tablas, enums, constraints BEV, triggers
│       ├── 00002_rls_policies.sql    # Políticas RLS (Pasajero, Conductor, Admin)
│       └── 00003_seed_demo_data.sql   # Datos de prueba para Santiago
├── docs/
│   ├── architecture.md               # Diagramas de arquitectura y modelo de amenazas
│   ├── legal-and-pending-decisions.md # Marco Ley EAT, electromovilidad y decisiones pendientes
│   └── api-specification.md          # Especificación de endpoints REST
├── .env.example                      # Plantilla de variables de entorno sin secretos
└── README.md
```

---

## 4. Guía de Ejecución Rápida en Desarrollo Local

### Requisitos Previos
- **Node.js**: v18+ o v22+
- **Rust y Cargo**: 1.75+ o superior

### 1. Iniciar el Backend en Rust
```bash
# Desde la raíz del proyecto
cd services/api

# Ejecutar el servidor (escuchará en http://127.0.0.1:4000)
cargo run
```
*En Windows con Visual Studio Build Tools, puedes ejecutar con el entorno MSVC inicializado (`vcvars64.bat && cargo run`).*

### 2. Iniciar el Frontend React
```bash
# En una nueva terminal
cd apps/web

# Instalar dependencias
npm install

# Iniciar servidor de desarrollo Vite (abrirá en http://localhost:5173)
npm run dev
```

### 3. Ejecutar las Pruebas Unitarias y de Integración
```bash
# Ejecutar la suite completa de pruebas del backend en Rust
cd services/api
cargo test
```
Pruebas incluidas y verificadas:
- `test_ev_validation_rules`: Valida que solo autos 100% eléctricos (BEV) año ≥ 2018 con batería ≥ 30 kWh sean admitidos.
- `test_pricing_calculation_santiago_clp`: Verifica cálculo de tarifas en CLP y ahorro de CO2.
- `test_state_machine_valid_transitions`: Ciclo de vida completo del viaje.
- `test_state_machine_invalid_skips_and_unauthorized`: Bloqueo de saltos de estado y usuarios no autorizados.
- `test_prevent_double_assignment`: Bloqueo de concurrencia cuando dos conductores intentan tomar la misma solicitud.
- `test_mock_email_provider`: Simulación de correos transaccionales sin credenciales externas.

---

## 5. Cuentas de Demostración Disponibles

| Rol | Correo | Perfil | Características |
|---|---|---|---|
| **Pasajero** | `pasajero@zuvio.cl` | Valentina Silva | Viajes activos en Providencia / Las Condes. Puede pedir y calificar viajes. |
| **Conductor** | `conductor1@zuvio.cl` | Rodrigo Araya | Conductor aprobado, Tesla Model 3 (60 kWh, Patente `LW-72-91`), en línea. |
| **Operaciones** | `admin@zuvio.cl` | Carlos Morales | Panel administrativo: aprueba/rechaza autos eléctricos y revisa auditoría. |

*Puedes alternar entre estos tres roles de forma instantánea usando la barra superior en la aplicación.*

---

## 6. Configuración de Servicios Externos

### Supabase
1. Crea un proyecto en [supabase.com](https://supabase.com).
2. Ejecuta secuencialmente los scripts en el SQL Editor de Supabase:
   - `supabase/migrations/00001_initial_schema.sql`
   - `supabase/migrations/00002_rls_policies.sql`
   - `supabase/migrations/00003_seed_demo_data.sql`
3. Copia tus variables a `.env` (`DATABASE_URL`, `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_JWT_SECRET`).

### Brevo (Sendinblue)
1. Obtén tu API Key v3 en Brevo.
2. Configura `BREVO_API_KEY=tu_api_key_aqui` en `.env`.
3. Si la clave se omite o está vacía, el backend activa automáticamente el `MockEmailProvider` registrando los envíos estructurados en consola.

---

## 7. Advertencias Legales y Decisiones Pendientes (Chile)

Consulte el documento exhaustivo en [`docs/legal-and-pending-decisions.md`](docs/legal-and-pending-decisions.md) para conocer los requisitos de la **Ley N° 21.553 (Ley EAT)**, licencias profesionales clase A2/A3, seguros de responsabilidad civil y las decisiones de negocio sujetas a confirmación del propietario.

