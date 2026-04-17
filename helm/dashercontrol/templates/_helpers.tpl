{{/*
Expand the name of the chart.
*/}}
{{- define "dashercontrol.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "dashercontrol.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Chart label
*/}}
{{- define "dashercontrol.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "dashercontrol.labels" -}}
helm.sh/chart: {{ include "dashercontrol.chart" . }}
{{ include "dashercontrol.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels (app)
*/}}
{{- define "dashercontrol.selectorLabels" -}}
app.kubernetes.io/name: {{ include "dashercontrol.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Name of the Secret holding app (and optionally DB) credentials.
*/}}
{{- define "dashercontrol.secretName" -}}
{{- if .Values.existingSecret }}
{{- .Values.existingSecret }}
{{- else }}
{{- printf "%s-secret" (include "dashercontrol.fullname" .) }}
{{- end }}
{{- end }}

{{/*
PostgreSQL password: from Values when not using generated secret for postgres only.
When secret.create and postgresql.enabled, password is in the main secret as postgres-password.
*/}}
{{- define "dashercontrol.postgresPassword" -}}
{{- .Values.postgresql.auth.password }}
{{- end }}
