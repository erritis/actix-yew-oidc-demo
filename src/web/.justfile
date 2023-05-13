environment := "development"

serve:
  CLIEND_ID=web \
  ISSUER_URL=http://localhost:8585/realms/actix-yew-demo \
  BACKEND_URL=http://localhost:8080 \
  trunk serve