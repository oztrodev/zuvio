# Zuvio — Marco Regulatorio en Chile y Decisiones Pendientes

> [!WARNING]
> **Descargo de Responsabilidad Legal**: Este documento no constituye asesoría legal ni tributaria formal. Todo el contenido debe ser revisado y validado por asesores legales especializados en derecho de transporte, laboral, tributario y de protección de datos en la República de Chile antes de cualquier lanzamiento comercial o captación de usuarios en producción.

---

## 1. Regulación de Transporte Remunerado de Pasajeros en Chile

### Ley N° 21.553 (Ley EAT - Empresas de Aplicaciones de Transportes)
En Chile, las plataformas de movilidad están reguladas por la Ley N° 21.553 y sus reglamentos dictados por el Ministerio de Transportes y Telecomunicaciones (MTT):

1. **Registro Nacional de EAT**:
   - Zuvio debe constituirse legalmente en Chile con giro correspondiente y registrarse formalmente en el Registro de Empresas de Aplicaciones de Transporte del MTT.
   - Tanto los conductores como los vehículos deben estar debidamente inscritos en el registro oficial.
2. **Requisitos de los Conductores**:
   - Poseer licencia de conducir profesional **Clase A2 o A3**.
   - Hoja de vida del conductor y certificado de antecedentes sin anotaciones penales o infracciones gravísimas a la Ley de Tránsito.
3. **Requisitos de los Vehículos**:
   - Antigüedad máxima: la normativa establece límites de antigüedad para el ingreso inicial y la operación continuada.
   - Estándares de seguridad: frenos ABS, airbags, control de estabilidad (ESP), cinturones de 3 puntas en todos los asientos.
4. **Seguros Obligatorios**:
   - Pólizas de seguros de responsabilidad civil por daños a pasajeros y terceros con coberturas mínimas estipuladas por el reglamento.

---

## 2. Marco de Electromovilidad en Chile

Chile cuenta con la **Estrategia Nacional de Electromovilidad**, que busca que el 100% del transporte público urbano y vehículos ligeros nuevos sean cero emisiones al 2035:
- **Homologación Técnica**: Todo vehículo eléctrico en Zuvio debe contar con homologación oficial del Centro de Control y Certificación Vehicular (3CV) del MTT.
- **Categorización Estricta**: Zuvio solo admite vehículos eléctricos de batería (**BEV - Battery Electric Vehicles**). Quedan excluidos expresamente los híbridos enchufables (PHEV), híbridos convencionales (HEV) y vehículos a combustión interna (ICE).
- **Incentivos y Patentes**: Beneficios en permiso de circulación para vehículos eléctricos vigentes según la Ley N° 21.499.

---

## 3. Política de Seguridad Vial y Normativa sobre Sustancias (Alcohol y Cannabis)

> [!CAUTION]
> **Tolerancia Cero y Seguridad Zuvio**:
> - En Chile rige la **Ley de Tolerancia Cero** (Ley N° 20.580) y la **Ley Emilia** (Ley N° 20.770), así como la Ley de Tránsito N° 18.290 y la Ley N° 20.000 (Sustancias Estupefacientes y Psicotrópicas).
> - **Queda terminantemente prohibido conducir bajo la influencia del alcohol, estupefacientes, cannabis o psicotrópicos.**
> - **Prohibición absoluta de fumar o consumir cannabis o tabaco dentro de los vehículos Zuvio.**
> - Zuvio jamás promoverá, facilitará ni tolerará el consumo o tráfico de sustancias ilícitas o controladas.
> - La noción de identidad "weed-friendly" **no implica consumo en servicio**: solo puede entenderse como una cultura de hospitalidad, respeto y no discriminación hacia estilos de vida urbanos, sujeta a confirmación explícita del propietario y visado legal.

---

## 4. Protección de Datos Personales (Ley N° 19.628 y Nueva Ley de Datos)

- **Consentimiento Expreso**: Separación estricta entre correos de servicio indispensables y comunicaciones comerciales u ofertas.
- **Minimización de Geolocalización**: Registro de geolocalización restringido únicamente a la ejecución del viaje.
- **Retención y Eliminación**: Plazo propuesto de 30 días para datos de telemetría de ruta fina y derecho de supresión de cuenta a solicitud del titular.

---

## 5. Régimen Tributario y Medios de Pago

- **IVA a Servicios Digitales**: Cumplimiento del artículo 8° letra n) de la Ley sobre Impuesto a las Ventas y Servicios (IVA servicios digitales) o facturación local según la estructura societaria de Zuvio en Chile.
- **Retención de Boletas / Honorarios a Conductores**: Integración con el Servicio de Impuestos Internos (SII) para la emisión o retención de documentos tributarios.
- **Procesador de Pagos**: La versión MVP implementa un adaptador `MockPaymentProvider` (sin cobro real). Para producción se evaluará la integración con pasarelas locales autorizadas (ej. Webpay Plus de Transbank, Mercado Pago o Stripe).

---

## 6. Matriz de Decisiones de Negocio Pendientes de Confirmación

| Parámetro / Decisión | Valor Propuesto en MVP | Estado | Responsable |
|---|---|---|---|
| **Estructura Tarifaria Gran Santiago** | Base $1.400 CLP, $650/km, $150/min | Pendiente Confirmación | Propietario / Finanzas |
| **Comisión de Plataforma** | 15% (Simulada, no descontada) | Pendiente Confirmación | Finanzas |
| **Tarifa de Cancelación Tardía** | $1.500 CLP si se cancela > 3 min tras aceptación | Pendiente Confirmación | Operaciones |
| **Zonas de Cobertura Inicial** | Providencia, Las Condes, Vitacura, Santiago Centro, Ñuñoa, Lo Barnechea | Pendiente Confirmación | Operaciones |
| **Capacidad Mínima de Batería EV** | 40 kWh (rango urbano de ~250 km) | Pendiente Confirmación | Operaciones Flota |
| **Antigüedad Máxima de Autos Eléctricos** | Año 2019 o superior | Pendiente Confirmación | Cumplimiento EAT |
| **Política de Marca "Weed-Friendly"** | Solo tono acogedor en comunicaciones de marca | Pendiente Confirmación | Dirección / Legal |
| **Pasarela de Pago Producción** | Transbank Webpay Plus / Mercado Pago | Pendiente Selección | CTO / Finanzas |

