DROP INDEX IF EXISTS tenants_oauth_unique;

ALTER TABLE tenants
  DROP COLUMN IF EXISTS oauth_provider,
  DROP COLUMN IF EXISTS oauth_subject;
