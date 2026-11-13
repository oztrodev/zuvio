# Despliegue de Zuvio

Frontend en **Netlify** (`netlify.toml`) y backend Rust en un servicio que corra contenedores (Railway, Render o Fly.io).

> [!WARNING]
> El backend actual usa **repositorio en memoria y sesión demo** (cabeceras `X-User-Role` / `X-User-Id`). Cualquiera que llame a la API puede hacerse pasar por admin. Es válido solo para una **demo pública**. Antes de producción real falta conectar Supabase Auth (validación de JWT) y SQLx/Postgres.

## 1. Probar la imagen en local

```bash
docker compose up --build
curl http://localhost:4000/api/v1/health
```

O solo la imagen: `docker build -t zuvio-api services/api && docker run -p 4000:4000 zuvio-api`.

## 2. Desplegar el backend

Crear un servicio desde el repo de GitHub con:

| Ajuste | Valor |
|---|---|
| Root directory / contexto | `services/api` |
| Dockerfile | `services/api/Dockerfile` |
| Health check | `GET /api/v1/health` |
| Puerto | el plataforma inyecta `PORT` (la app lo respeta; escucha en `0.0.0.0`) |

Variables de entorno (en el panel del proveedor, nunca en Git):

| Variable | Valor |
|---|---|
| `CORS_ALLOWED_ORIGINS` | `https://zuvio.netlify.app` (lista separada por comas; `*` solo para demo) |
| `DEMO_MODE` | `true` mientras no exista autenticación real |
| `BREVO_API_KEY` | opcional; vacío = correo simulado |
| `BREVO_SENDER_EMAIL`, `BREVO_SENDER_NAME` | remitente verificado en Brevo |

## 3. Conectar el frontend

En Netlify → *Site settings → Environment variables* agregar:

```
VITE_API_URL = https://<url-publica-del-backend>      # sin /api/v1 al final
```

Luego *Deploys → Trigger deploy → Clear cache and deploy site*.
