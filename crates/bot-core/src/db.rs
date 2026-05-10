pub const CREATE_TABLES: &str = r#"
CREATE TABLE IF NOT EXISTS menu_nodes (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT REFERENCES menu_nodes(id),
    slug TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    image_url TEXT,
    sort_order INT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    platform TEXT NOT NULL,
    platform_user_id BIGINT NOT NULL,
    current_menu_node_id BIGINT REFERENCES menu_nodes(id),
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(platform, platform_user_id)
);
"#;

pub const SELECT_ALL_MENU_NODES: &str =
    "SELECT id, parent_id, slug, title, content, image_url, sort_order FROM menu_nodes ORDER BY sort_order";

pub const SELECT_USER: &str =
    "SELECT id, current_menu_node_id FROM users WHERE platform = $1 AND platform_user_id = $2";

pub const UPSERT_USER: &str = r#"
INSERT INTO users (platform, platform_user_id)
VALUES ($1, $2)
ON CONFLICT (platform, platform_user_id) DO NOTHING
RETURNING id, current_menu_node_id
"#;

pub const UPDATE_USER_NODE: &str =
    "UPDATE users SET current_menu_node_id = $1 WHERE id = $2";