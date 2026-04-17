ALTER TABLE workspaces
    DROP COLUMN IF EXISTS workspace_icon,
    DROP COLUMN IF EXISTS theme_preset,
    DROP COLUMN IF EXISTS bg_color,
    DROP COLUMN IF EXISTS fg_color,
    DROP COLUMN IF EXISTS accent_color,
    DROP COLUMN IF EXISTS panel_color,
    DROP COLUMN IF EXISTS background_image_url;
