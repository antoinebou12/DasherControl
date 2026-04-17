ALTER TABLE tenants
  ADD COLUMN oauth_provider VARCHAR(32),
  ADD COLUMN oauth_subject VARCHAR(255);

CREATE UNIQUE INDEX tenants_oauth_unique
  ON tenants (oauth_provider, oauth_subject)
  WHERE oauth_provider IS NOT NULL AND oauth_subject IS NOT NULL;
