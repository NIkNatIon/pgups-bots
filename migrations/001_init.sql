SET client_encoding TO 'UTF8';

DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS menu_nodes CASCADE;
DROP TABLE IF EXISTS translations CASCADE;

CREATE TABLE IF NOT EXISTS menu_nodes (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT REFERENCES menu_nodes(id),
    slug TEXT NOT NULL,
    image_url TEXT,
    sort_order INT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    platform TEXT NOT NULL,
    platform_user_id BIGINT NOT NULL,
    current_menu_node_id BIGINT REFERENCES menu_nodes(id),
    student_group TEXT,
    lang TEXT DEFAULT 'ru',
    UNIQUE(platform, platform_user_id)
);

CREATE TABLE IF NOT EXISTS translations (
    lang TEXT PRIMARY KEY,
    content TEXT NOT NULL
);

-- Корень
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(1, NULL, 'start', 0);

-- Уровень 1
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(2, 1, 'info', 0),
(3, 1, 'about', 1),
(4, 1, 'schedule', 2);

-- Уровень 2: Категории
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(10, 2, 'abit', 0),
(11, 2, 'stud', 1),
(12, 2, 'prof', 2);

-- Абитуриентам
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(20, 10, 'docs', 0),
(21, 10, 'dorm', 1),
(22, 10, 'calendar', 2),
(23, 10, 'contacts', 3),
(24, 10, 'exam', 4),
(25, 10, 'rules', 5),
(26, 10, '3d', 6);

-- Общежития для абитуриентов
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(30, 21, 'd1', 0),
(31, 21, 'd2', 1),
(32, 21, 'd3', 2),
(33, 21, 'd4', 3),
(34, 21, 'd5', 4),
(35, 21, 'd5k3', 5),
(36, 21, 'd6', 6),
(37, 21, 'd7a', 7),
(38, 21, 'd8', 8);

-- Студентам
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(40, 11, 'facs', 0),
(41, 11, 'stud_dorm', 1),
(42, 11, 'infrastructure', 2),
(43, 11, 'scholarship', 3),
(44, 11, 'portfolio', 4),
(45, 11, 'struct', 5),
(46, 11, 'map', 6);

-- Факультеты
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(50, 40, 'ait', 0),
(51, 40, 'bfo', 1),
(52, 40, 'eim', 2),
(53, 40, 'pgs', 3),
(54, 40, 'tes', 4),
(55, 40, 'ts', 5),
(56, 40, 'upl', 6);

-- Общежития для студентов
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(60, 41, 'sd1', 0),
(61, 41, 'sd2', 1),
(62, 41, 'sd3', 2),
(63, 41, 'sd4', 3),
(64, 41, 'sd5', 4),
(65, 41, 'sd5k3', 5),
(66, 41, 'sd6', 6),
(67, 41, 'sd7a', 7),
(68, 41, 'sd8', 8);

-- Инфраструктура
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(70, 42, 'canteens', 0),
(71, 42, 'libraries', 1),
(72, 42, 'hospital', 2);

-- Преподавателям
INSERT INTO menu_nodes (id, parent_id, slug, sort_order) VALUES
(80, 12, 'prof_docs', 0);

-- Обновляем sequence
SELECT setval('menu_nodes_id_seq', (SELECT MAX(id) FROM menu_nodes));
UPDATE menu_nodes SET image_url = 'karta_pgups.jpg' WHERE id = 46;
