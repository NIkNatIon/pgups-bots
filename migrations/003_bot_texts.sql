CREATE TABLE IF NOT EXISTS bot_texts (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT INTO bot_texts (key, value) VALUES
('btn.back', '⬅ Назад'),
('btn.home', '🏠 В начало'),
('btn.schedule_today', 'На сегодня'),
('btn.schedule_tomorrow', 'На завтра'),
('btn.schedule_this_week', 'Текущая неделя'),
('btn.schedule_next_week', 'Следующая неделя'),
('btn.schedule_change_group', '🔄 Сменить группу'),
('msg.select_section', 'Выберите раздел:'),
('msg.ask_group', 'Введите номер вашей группы (например, ИВБ-211):'),
('msg.schedule_header', E'Расписание для группы {group}\n\nСегодня: {weekday}, {parity} неделя'),
('msg.schedule_no_lessons', 'Пар нет.'),
('msg.schedule_error', 'Ошибка получения расписания.'),
('msg.schedule_server_down', 'Сервер расписания не отвечает.'),
('msg.schedule_connection_error', 'Не удалось подключиться к серверу расписания.'),
('msg.schedule_week_error', 'Не удалось получить расписание на неделю.'),
('msg.schedule_invalid_group', 'Группа не найдена. Проверьте правильность ввода.')
ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value;