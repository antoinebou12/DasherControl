ALTER TABLE workspaces
    ADD COLUMN workspace_icon VARCHAR(64) NOT NULL DEFAULT '',
    ADD COLUMN theme_preset VARCHAR(32) NOT NULL DEFAULT 'dracula',
    ADD COLUMN bg_color VARCHAR(16),
    ADD COLUMN fg_color VARCHAR(16),
    ADD COLUMN accent_color VARCHAR(16),
    ADD COLUMN panel_color VARCHAR(16),
    ADD COLUMN background_image_url TEXT;
