{{- define "search-autocomplete.fullname" -}}
{{- default "search-autocomplete" .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}
