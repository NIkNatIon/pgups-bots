SET client_encoding TO 'UTF8';

DELETE FROM translations;

-- ========================================
-- Русский
-- ========================================
INSERT INTO translations (lang, content) VALUES ('ru', '
# Кнопки навигации
btn-back = ⬅ Назад
btn-home = 🏠 В начало
btn-language = 🌐 Язык

# Кнопки расписания
btn-schedule-today = На сегодня
btn-schedule-tomorrow = На завтра
btn-schedule-this-week = Текущая неделя
btn-schedule-next-week = Следующая неделя
btn-schedule-change-group = 🔄 Сменить группу

# Системные сообщения
msg-select-section = Выберите раздел:
msg-ask-group = Введите номер вашей группы (например, ИВБ-211):
msg-schedule-header = Расписание для группы { $group }
    Сегодня: { $weekday }, { $parity } неделя
msg-schedule-day-off = Выходной день, пар нет.
msg-schedule-error = Ошибка получения расписания.
msg-schedule-server-down = Сервер расписания не отвечает.
msg-schedule-connection-error = Не удалось подключиться к серверу расписания.
msg-schedule-week-error = Не удалось получить расписание на неделю.
msg-schedule-invalid-group = Группа не найдена. Проверьте правильность ввода.
msg-select-language = Выберите язык:
msg-language-changed = Язык изменён.

# Расписание
schedule-no-lessons = Пар нет.
schedule-pair = пара
schedule-room = Ауд
schedule-weekday-monday = Понедельник
schedule-weekday-tuesday = Вторник
schedule-weekday-wednesday = Среда
schedule-weekday-thursday = Четверг
schedule-weekday-friday = Пятница
schedule-weekday-saturday = Суббота
schedule-weekday-sunday = Воскресенье
schedule-parity-odd = нечётная
schedule-parity-even = чётная

# Меню: корень
menu-start-title = 🏠 Главное меню
menu-start-content = Добро пожаловать! Выберите интересующий раздел:

# Меню: уровень 1
menu-info-title = ℹ️ Информация
menu-info-content = Здесь Вы можете узнать различную информацию об университете. Выберите интересующую Вас категорию:
menu-about-title = 📌 О проекте
menu-about-content = Доступный ПГУПС — проект, разрабатываемый в рамках студенческого общества факультета «Автоматизация и Интеллектуальные Технологии».

# Меню: категории
menu-abit-title = 🎓 Абитуриентам
menu-abit-content = Выберите интересующую Вас информацию:
menu-stud-title = 👨‍🎓 Студентам
menu-stud-content = Выберите интересующую Вас информацию:
menu-prof-title = 👩‍🏫 Преподавателям
menu-prof-content = Выберите интересующую Вас информацию:

# Меню: абитуриентам
menu-docs-title = 📋 Документы
menu-docs-content =
    Вы можете подать документы дистанционно через личный кабинет абитуриента или госуслуги или очно, придя в университет.

    📎 Для подачи через личный кабинет зарегистрируйтесь на сайте https://abit.pgups.ru/

    📎 Для подачи через госуслуги заполните заявление и укажите предметы ЕГЭ.

    📎 Для подачи очно пройдите в приёмную комиссию.
    Режим работы: пн–пт 9:00–17:00, сб 9:00–16:00.

    ⚠️ Для зачисления необходимо предоставить оригинал документа об образовании и согласие на зачисление.

menu-dorm-title = 🏢 Общежития
menu-dorm-content =
    Наш университет предоставляет общежития для обучающихся. Всего 9 общежитий, более 3400 мест.

    Выберите номер общежития в меню.

    📄 Для заселения необходимы:
    • Документ, удостоверяющий личность
    • Медицинская справка от дерматолога
    • 4 фотографии 3х4
    • Согласие родителей (для несовершеннолетних)

    🔗 Подробнее:
    https://www.pgups.ru/struct/upravlenie-zhilishchnogo-fonda/predostavlenie-obshchezhitiya
    https://priem.pgups.ru/hostels

menu-calendar-title = 📅 Календарь поступления
menu-calendar-content =
    📅 Календарь поступления:

    📌 19 июня — Начало приёма документов
    📌 14 июля — Завершение приёма (поступающие по ВИ)
    📌 17–25 июля — Вступительные экзамены на бюджет
    📌 26 июля – 28 августа — ВИ на коммерческую основу
    📌 25 июля — Завершение приёма (по результатам ЕГЭ)
    📌 28 июля — Завершение приёма оригиналов (приоритетный этап)
    📌 29–30 июля — Приказы о зачислении (приоритетный этап)
    📌 3 августа — Завершение приёма оригиналов (основной этап)
    📌 4–9 августа — Приказы о зачислении (основной этап)
    📌 19 августа — Завершение приёма на платное обучение
    📌 29 августа — Завершение заключения договоров

menu-contacts-title = 📞 Контакты
menu-contacts-content =
    📞 Контакты приёмной комиссии:

    🇷🇺 Гражданам РФ (бакалавриат, специалитет, магистратура):
    📞 8(800)200-97-90 или 8(812)457-82-42
    ✉️ primkom@pgups.ru

    🌍 Гражданам других стран:
    📞 8(812)310-17-27 или 8(812)315-40-59
    ✉️ edusupport@pgups.ru

    🎓 Поступающим в аспирантуру:
    📞 8(812)457-80-97
    ✉️ asp@pgups.ru

menu-exam-title = 📝 Вступительные экзамены
menu-exam-content =
    В ПГУПС могут поступать по ВИ следующие категории:

    • лица, закончившие СПО
    • лица, имеющие инвалидность
    • иностранные граждане
    • студенты, поступающие в магистратуру

    📍 Экзамен проводится очно с применением дистанционных технологий.

    💡 Если Вы сдавали ВИ на бюджет, но не прошли — результаты переносятся на конкурс на коммерцию. Рекомендуем всегда сдавать ВИ на бюджет.

menu-rules-title = 📖 Правила приёма
menu-rules-content =
    📖 С правилами приёма можно ознакомиться на сайте приёмной комиссии:
    https://priem.pgups.ru/info

menu-3d-title = 🖥 3D-тур
menu-3d-content =
    Вы можете виртуально прогуляться по нашему университету, используя 3D-тур! 🏛

    http://3d.pgups.ru/

# Общежития для абитуриентов
menu-d1-title = Общежитие №1
menu-d1-content =
    🏢 Общежитие №1 — 644 места

    Спортивный зал, спортивная площадка, читальный зал, интернет.

    📍 190005, г. Санкт-Петербург, ул. 7-я Красноармейская, д. 12
    🚇 ст. м. «Технологический институт», «Фрунзенская»
    ✉️ hostel1@pgups.ru
    📞 +7(812)712-66-15, +7(812)713-02-54

menu-d2-title = Общежитие №2
menu-d2-content =
    🏢 Общежитие №2 — 288 мест (квартирного типа)

    Интернет, учебные классы.

    📍 197046, г. Санкт-Петербург, Кронверкский пр., д.9
    📞 +7(812)233-72-07, +7(812)232-70-87
    ✉️ hostel2@pgups.ru

menu-d3-title = Общежитие №3
menu-d3-content =
    🏢 Общежитие №3 — 178 мест

    Для обучающихся заочной формы. Рядом есть кафе.

    📍 190031, г. Санкт-Петербург, Столярный пер., д.6
    📞 +7(812)571-02-29
    ✉️ hostel3@pgups.ru

menu-d4-title = Общежитие №4
menu-d4-content =
    🏢 Общежитие №4 — 262 места

    Комната отдыха, спортивная и учебная комнаты, прачечная, интернет.

    📍 190031, г. Санкт-Петербург, наб. р. Фонтанки, д.113
    📞 +7(812)436-98-66
    ✉️ hostel4@pgups.ru

menu-d5-title = Общежитие №5
menu-d5-content =
    🏢 Общежитие №5

    Комната отдыха, спортивная площадка, прачечная, интернет.

    📍 197046, г. Санкт-Петербург, ул. Малая Посадская, д. 22-24
    📞 +7(812)233-54-30, +7(812)232-35-76
    ✉️ hostel5@pgups.ru

menu-d5k3-title = Общежитие №5 к.3
menu-d5k3-content =
    🏢 Общежитие №5 к.3

    📍 197046, г. Санкт-Петербург, ул. Малая Посадская, д. 22-24
    📞 +7(812)233-89-25
    ✉️ hostel5k3@pgups.ru

menu-d6-title = Общежитие №6
menu-d6-content =
    🏢 Общежитие №6 — 287 мест (блочного типа)

    Учебный класс, прачечная, интернет.

    📍 190121, г. Санкт-Петербург, ул. Перевозная, д.2
    📞 +7(812)714-82-65
    ✉️ hostel6@pgups.ru

menu-d7a-title = Общежитие №7а
menu-d7a-content =
    🏢 Общежитие №7а

    📍 197046, г. Санкт-Петербург, ул. Малая Посадская, д. 22-24
    📞 +7(812)232-14-86, +7(812)232-40-68
    ✉️ hostel7a@pgups.ru

menu-d8-title = Общежитие №8
menu-d8-content =
    🏢 Общежитие №8 — 804 места (квартирного типа)

    г. Пушкин. Спортивная площадка, тренажёрный зал, прачечная, интернет.

    📍 189620, г. Пушкин, ул. Оранжерейная, д.73
    📞 +7(812)465-90-50, +7(812)476-17-33, +7(812)465-99-85, +7(812)465-94-86
    ✉️ hostel8@pgups.ru

# Студентам
menu-schedule-title = 📅 Расписание
menu-facs-title = 🏫 Факультеты
menu-facs-content = В нашем университете 7 факультетов. Выберите название в меню:
menu-stud_dorm-title = 🏠 Общежития
menu-stud_dorm-content =
    Наш университет предоставляет общежития для обучающихся. Всего 9 общежитий, более 3400 мест.

    Выберите номер общежития в меню.

    📄 Для заселения необходимы:
    • Документ, удостоверяющий личность
    • Медицинская справка от дерматолога
    • 4 фотографии 3х4
    • Согласие родителей (для несовершеннолетних)

    🔗 Подробнее:
    https://www.pgups.ru/struct/upravlenie-zhilishchnogo-fonda/predostavlenie-obshchezhitiya
    https://priem.pgups.ru/hostels

menu-infrastructure-title = 🏗 Инфраструктура
menu-infrastructure-content = Выберите интересующий Вас объект инфраструктуры:
menu-scholarship-title = 💰 Стипендии
menu-scholarship-content =
    💰 Стипендии и материальная помощь:

    1. Государственная академическая стипендия
    2. Государственная социальная стипендия
    3. Стипендия Президента РФ
    4. Стипендия Правительства РФ
    5. Повышенная академическая стипендия за особые достижения
    6. Повышенная социальная стипендия (1–2 курс, «хор.» и «отл.»)
    7. Увеличенная стипендия для талантливых абитуриентов
    8. Именные стипендии

    Выдвижение кандидатур производится факультетами.

    Также студентам очной формы за счёт федерального бюджета выплачивается материальная помощь.

menu-portfolio-title = 📂 Портфолио
menu-portfolio-content =
    📂 Правила заполнения портфолио:
    https://vk.com/wall-102162446_2965

menu-struct-title = 🏛 Структуры
menu-struct-content =
    🏛 Структуры:

    1. Студенческие советы факультетов
    2. Волонтерское направление
    3. Студенческие советы общежитий
    4. Культурно-досуговый центр
    5. Студенческое научное общество
    6. TERRA — экологический кружок
    7. СМИ
    8. Факультетская профсоюзная организация
    9. Патриотический центр
    10. Дружина
    11. Спортивный отряд
    12. Студенческие отряды

menu-map-title = 🗺 Карта университета
menu-map-content = 🗺 Карта университета

# Факультеты
menu-ait-title = АИТ
menu-ait-content =
    🏫 Факультет «Автоматизация и Интеллектуальные Технологии»
    https://www.pgups.ru/struct/avtomatizatsiya-i-intellektualnye-tekhnologii/

    👤 Декан: Степанская Ольга Андреевна
    📞 +7(812)570-52-37
    📍 Московский пр., д. 9, ауд. 7-420

    📚 Кафедры:
    • Автоматика и телемеханика на ж/д
    • Высшая математика
    • Информатика и информационная безопасность
    • Информационные и вычислительные системы
    • Электрическая связь
    • Электроснабжение железных дорог

menu-bfo-title = БФО
menu-bfo-content =
    🏫 Факультет «Безотрывные Формы Обучения»
    https://www.pgups.ru/struct/fakultet_bezotryvnykh_form_obucheniya/

    👤 Декан: Куценко Сергей Михайлович
    ✉️ fbfo@pgups.ru
    📞 +7(812)457-81-17, +7(812)436-92-66
    📍 Московский пр., д. 9, ауд. 7-524

menu-eim-title = ЭиМ
menu-eim-content =
    🏫 Факультет «Экономика и менеджмент»
    https://www.pgups.ru/struct/ekonomika-i-menedzhment/

    👤 Декан: Кабанов Александр Васильевич
    ✉️ eim@pgups.ru
    📞 +7(812)407-40-39, +7(812)457-85-46
    📍 наб. р. Фонтанки, 115 (Юсуповский дворец), ауд. 9-124

    📚 Кафедры:
    • Экономика транспорта
    • Экономика и менеджмент в строительстве
    • Прикладная психология
    • Бухгалтерский учет и аудит
    • История, философия, политология и социология
    • Менеджмент и маркетинг

menu-pgs-title = ПГС
menu-pgs-content =
    🏫 Факультет «Промышленное и Гражданское Строительство»
    https://www.pgups.ru/struct/promyshlennoe_i_grazhdanskoe_stroitelstvo/

    👤 Декан: Абу-Хасан Махмуд Самиевич
    ✉️ pgs@pgups.ru
    📞 +7(812)315-13-32, +7(812)457-81-09
    📍 Московский пр., д. 9, ауд. 2-224

    📚 Кафедры:
    • Водоснабжение, водоотведение и гидравлика
    • Инженерная химия и естествознание
    • Основания и фундаменты
    • Строительные конструкции, здания и сооружения
    • Строительные материалы и технологии
    • Техносферная и экологическая безопасность
    • Физика

menu-tes-title = ТЭС
menu-tes-content =
    🏫 Факультет «Транспортные и Энергетические Системы»
    https://www.pgups.ru/struct/transportnye-i-energeticheskie-sistemy/

    👤 Декан: Чуян Сергей Николаевич
    ✉️ tes@pgups.ru
    📞 +7(812)315-40-53, +7(812)457-81-14
    📍 Московский пр., д. 9, ауд. 4-205

    📚 Кафедры:
    • Вагоны и вагонное хозяйство
    • Локомотивы и локомотивное хозяйство
    • Наземные транспортно-технологические комплексы
    • Электрическая тяга
    • Электротехника и теплоэнергетика

menu-ts-title = ТС
menu-ts-content =
    🏫 Факультет «Транспортное Строительство»
    https://www.pgups.ru/struct/transportnoe_stroitelstvo/

    👤 Декан: Бушуев Николай Сергеевич
    📞 +7(812)315-18-54
    📍 Московский пр., д. 9, ауд. 1-310, 1-407

    📚 Кафедры:
    • Железнодорожный путь
    • Изыскания и проектирование ж/д
    • Инженерная геодезия
    • Мосты
    • Начертательная геометрия и графика
    • Механика и прочность материалов
    • Строительство дорог транспортного комплекса
    • Тоннели и метрополитены

menu-upl-title = УПЛ
menu-upl-content =
    🏫 Факультет «Управление Перевозками и Логистика»
    https://www.pgups.ru/struct/upravlenie-perevozkami-i-logistika/

    👤 Декан: Бадецкий Александр Петрович
    ✉️ upl@pgups.ru
    📞 +7(812)310-65-75, +7(812)457-81-10
    📍 Московский пр., д. 9, ауд. 7-326

    📚 Кафедры:
    • Управление эксплуатационной работой
    • Железнодорожные станции и узлы
    • Логистика и коммерческая работа
    • Русский и иностранные языки
    • Физическая культура

# Общежития для студентов
menu-sd1-title = Общежитие №1
menu-sd1-content =
    🏢 Общежитие №1

    👤 Заведующий: Микушева Роза Вакильевна
    📞 +7(812)713-02-54
    ✉️ hostel1@pgups.ru
    📍 ул. 7-я Красноармейская, д.12
    🔗 https://vk.com/pgupshostel1

menu-sd2-title = Общежитие №2
menu-sd2-content =
    🏢 Общежитие №2

    👤 Заведующий: Чжан Фёдор Владимирович
    📞 +7(812)233-72-07
    ✉️ hostel2@pgups.ru
    📍 Кронверкский пр., д.9
    🔗 https://vk.com/hostel2_pgups_spb

menu-sd3-title = Общежитие №3
menu-sd3-content =
    🏢 Общежитие №3

    👤 Заведующий: Ковалев Игорь Викторович
    ✉️ hostel3@pgups.ru
    📞 +7(812)571-02-29, +7(812)314-96-19
    📍 Столярный пер., д.6

menu-sd4-title = Общежитие №4
menu-sd4-content =
    🏢 Общежитие №4

    👤 Заведующий: Никитина Ольга Юрьевна
    ✉️ hostel4@pgups.ru
    📞 +7(812)436-98-66, +7(812)457-89-05
    📍 наб. р. Фонтанки, д.113
    🔗 https://vk.com/sso4_pgups

menu-sd5-title = Общежитие №5
menu-sd5-content =
    🏢 Общежитие №5

    👤 Заведующий: Дуркина Светлана Александровна
    ✉️ hostel5@pgups.ru
    📞 +7(812)233-54-30, +7(812)232-35-76, +7(812)233-11-74
    📍 ул. Малая Посадская, д. 22-24
    🔗 https://vk.com/dormitory_5

menu-sd5k3-title = Общежитие №5 к.3
menu-sd5k3-content =
    🏢 Общежитие №5 к.3

    👤 Заведующий: Савицкий Павел Павлович
    ✉️ hostel5k3@pgups.ru
    📞 +7(812)233-89-25, +7(812)233-27-03
    📍 ул. Малая Посадская, д. 22-24
    🔗 https://vk.com/obschaga5k3pgups

menu-sd6-title = Общежитие №6
menu-sd6-content =
    🏢 Общежитие №6

    👤 Заведующий: Кучук Дмитрий Иосифович
    📞 +7(812)714-82-65
    ✉️ hostel6@pgups.ru
    📍 ул. Перевозная, д.2
    🔗 https://vk.com/hostelnumbersix

menu-sd7a-title = Общежитие №7а
menu-sd7a-content =
    🏢 Общежитие №7а

    👤 Заведующий: Савицкий Павел Павлович
    ✉️ hostel7a@pgups.ru
    📞 +7(812)232-14-86, +7(812)232-40-68, +7(812)232-76-84
    📍 ул. Малая Посадская, д. 22-24
    🔗 https://vk.com/club217049829

menu-sd8-title = Общежитие №8
menu-sd8-content =
    🏢 Общежитие №8

    👤 Заведующий: Кожевников Александр Анатольевич
    ✉️ hostel8@pgups.ru
    📞 +7(812)465-90-50, +7(812)476-17-33, +7(812)465-99-85, +7(812)465-94-86
    📍 г. Пушкин, ул. Оранжерейная, д.73
    🔗 https://vk.com/hostel_8_pgups

# Инфраструктура
menu-canteens-title = 🍽 Столовые
menu-canteens-content =
    🍽 Столовые в университете:

    • ауд. 7-205
    • ауд. 7-206
    • ауд. 1-219
    • ауд. 3-208

menu-libraries-title = 📚 Библиотеки
menu-libraries-content =
    📚 Библиотеки университета:

    • Основная библиотека — ауд. 3-207
    • Научно-техническая — ауд. 6-211
    • Общественно-политической литературы — ауд. 1-309
    • Художественной литературы — ауд. 1-314

    ⚠️ Читательский билет оформляется в первый месяц обучения в ауд. 3-207.

menu-hospital-title = 🏥 Поликлиника
menu-hospital-content =
    🏥 Поликлиника университета:

    📍 г. Санкт-Петербург, наб. реки Фонтанки, д.115

# Преподавателям
menu-prof_docs-title = 🔗 Полезные ссылки
menu-prof_docs-content =
    🔗 Полезные ссылки:

    📄 Документы
    https://www.pgups.ru/sveden/document/

    📢 Объявление конкурсов
    https://www.pgups.ru/university/general-information/ads/

    👥 Социальный отдел
    https://www.pgups.ru/struct/sotsialnyy_otdel/

    ✉️ Корпоративная почта
    https://www.pgups.ru/struct/upravlenie_informatizatsii/korporativnaya-pochta

    📝 Заявление в Управление информатизации
    https://www.pgups.ru/struct/upravlenie_informatizatsii/zayavka-v-upravlenie-informatizatsii

    🤝 Профком
    https://www.pgups.ru/employees/the-trade-union/

    🏡 Отдел загородных объектов
    https://www.pgups.ru/struct/zagorodnye_obekty/geobaza-universiteta-korpusa-dlya-otdykha

# Онбординг
msg-onboarding-welcome = Добро пожаловать! Давайте настроим бота для вас.
msg-onboarding-ask-lang = Выберите язык:
msg-onboarding-ask-role = Выберите вашу роль:
msg-onboarding-ask-group = Введите номер вашей группы (например, ИВБ-211):
msg-onboarding-skip-group = Вы можете пропустить этот шаг.
msg-onboarding-complete = Настройка завершена!

# Роли
role-applicant = 🎓 Абитуриент
role-student = 👨‍🎓 Студент
role-teacher = 👩‍🏫 Преподаватель
role-guest = 👤 Гость

# Настройки
msg-settings = Настройки
msg-settings-lang = Язык: { $lang }
msg-settings-role = Роль: { $role }
msg-settings-group = Группа: { $group }
btn-settings = ⚙️ Настройки
btn-settings-change-lang = Сменить язык
btn-settings-change-role = Сменить роль
btn-settings-change-group = Сменить группу
btn-skip = Пропустить
');

-- ========================================
-- English
-- ========================================
INSERT INTO translations (lang, content) VALUES ('en', '
# Navigation buttons
btn-back = ⬅ Back
btn-home = 🏠 Home
btn-language = 🌐 Language

# Schedule buttons
btn-schedule-today = Today
btn-schedule-tomorrow = Tomorrow
btn-schedule-this-week = This week
btn-schedule-next-week = Next week
btn-schedule-change-group = 🔄 Change group

# System messages
msg-select-section = Select a section:
msg-ask-group = Enter your group number (e.g. IVB-211):
msg-schedule-header = Schedule for group { $group }
    Today: { $weekday }, { $parity } week
msg-schedule-day-off = Day off, no classes.
msg-schedule-error = Error getting schedule.
msg-schedule-server-down = Schedule server is not responding.
msg-schedule-connection-error = Could not connect to schedule server.
msg-schedule-week-error = Could not get weekly schedule.
msg-schedule-invalid-group = Group not found. Please check your input.
msg-select-language = Select language:
msg-language-changed = Language changed.

# Schedule
schedule-no-lessons = No classes.
schedule-pair = class
schedule-room = Room
schedule-weekday-monday = Monday
schedule-weekday-tuesday = Tuesday
schedule-weekday-wednesday = Wednesday
schedule-weekday-thursday = Thursday
schedule-weekday-friday = Friday
schedule-weekday-saturday = Saturday
schedule-weekday-sunday = Sunday
schedule-parity-odd = odd
schedule-parity-even = even

# Menu: root
menu-start-title = 🏠 Main Menu
menu-start-content = Welcome! Select a section:

# Menu: level 1
menu-info-title = ℹ️ Information
menu-info-content = Here you can find various information about the university. Select a category:
menu-about-title = 📌 About
menu-about-content = Accessible PGUPS — a project developed by the student society of the Faculty of Automation and Intelligent Technologies.

# Menu: categories
menu-abit-title = 🎓 Applicants
menu-abit-content = Select information:
menu-stud-title = 👨‍🎓 Students
menu-stud-content = Select information:
menu-prof-title = 👩‍🏫 Teachers
menu-prof-content = Select information:

# Menu: applicants
menu-docs-title = 📋 Documents
menu-docs-content =
    You can submit documents remotely via the applicant personal account or government services, or in person.

    📎 To submit via personal account, register at https://abit.pgups.ru/

    📎 To submit via government services, fill out the application and specify your exam subjects.

    📎 To submit in person, visit the admissions office.
    Working hours: Mon–Fri 9:00–17:00, Sat 9:00–16:00.

    ⚠️ For enrollment, you must provide the original education document and consent for enrollment.

menu-dorm-title = 🏢 Dormitories
menu-dorm-content =
    Our university provides dormitories for students. There are 9 dormitories with over 3,400 places.

    Select a dormitory number from the menu.

    📄 Required for check-in:
    • Identity document
    • Medical certificate from a dermatologist
    • 4 photos 3x4
    • Parental consent (for minors)

    🔗 More info:
    https://www.pgups.ru/struct/upravlenie-zhilishchnogo-fonda/predostavlenie-obshchezhitiya
    https://priem.pgups.ru/hostels

menu-calendar-title = 📅 Admission Calendar
menu-calendar-content =
    📅 Admission Calendar:

    📌 June 19 — Start of document acceptance
    📌 July 14 — End of acceptance (entrance exams)
    📌 July 17–25 — Entrance exams for budget
    📌 July 26 – August 28 — Entrance exams for paid education
    📌 July 25 — End of acceptance (USE results)
    📌 July 28 — Deadline for originals (priority stage)
    📌 July 29–30 — Enrollment orders (priority stage)
    📌 August 3 — Deadline for originals (main stage)
    📌 August 4–9 — Enrollment orders (main stage)
    📌 August 19 — End of paid education acceptance
    📌 August 29 — End of contract signing

menu-contacts-title = 📞 Contacts
menu-contacts-content =
    📞 Admissions Office Contacts:

    🇷🇺 Russian citizens (bachelor, specialist, master):
    📞 8(800)200-97-90 or 8(812)457-82-42
    ✉️ primkom@pgups.ru

    🌍 International students:
    📞 8(812)310-17-27 or 8(812)315-40-59
    ✉️ edusupport@pgups.ru

    🎓 PhD applicants:
    📞 8(812)457-80-97
    ✉️ asp@pgups.ru

menu-exam-title = 📝 Entrance Exams
menu-exam-content =
    The following categories can take entrance exams at PGUPS:

    • Graduates of secondary vocational education
    • People with disabilities
    • International citizens
    • Master degree applicants

    📍 Exams are held in person with distance technologies.

    💡 If you took entrance exams for budget but did not pass, results transfer to the paid competition.

menu-rules-title = 📖 Admission Rules
menu-rules-content =
    📖 Admission rules are available on the admissions website:
    https://priem.pgups.ru/info

menu-3d-title = 🖥 3D Tour
menu-3d-content =
    Take a virtual tour of our university! 🏛

    http://3d.pgups.ru/

# Dormitories for applicants
menu-d1-title = Dormitory #1
menu-d1-content =
    🏢 Dormitory #1 — 644 places

    Gym, sports ground, reading room, internet.

    📍 190005, St. Petersburg, 7th Krasnoarmeyskaya st., 12
    ✉️ hostel1@pgups.ru
    📞 +7(812)712-66-15, +7(812)713-02-54

menu-d2-title = Dormitory #2
menu-d2-content =
    🏢 Dormitory #2 — 288 places (apartment type)

    Internet, study rooms.

    📍 197046, St. Petersburg, Kronverksky pr., 9
    📞 +7(812)233-72-07, +7(812)232-70-87
    ✉️ hostel2@pgups.ru

menu-d3-title = Dormitory #3
menu-d3-content =
    🏢 Dormitory #3 — 178 places

    For part-time students. Nearby cafe.

    📍 190031, St. Petersburg, Stolyarny per., 6
    📞 +7(812)571-02-29
    ✉️ hostel3@pgups.ru

menu-d4-title = Dormitory #4
menu-d4-content =
    🏢 Dormitory #4 — 262 places

    Lounge, sports room, study room, laundry, internet.

    📍 190031, St. Petersburg, Fontanka emb., 113
    📞 +7(812)436-98-66
    ✉️ hostel4@pgups.ru

menu-d5-title = Dormitory #5
menu-d5-content =
    🏢 Dormitory #5

    Lounge, sports ground, laundry, internet.

    📍 197046, St. Petersburg, Malaya Posadskaya st., 22-24
    📞 +7(812)233-54-30, +7(812)232-35-76
    ✉️ hostel5@pgups.ru

menu-d5k3-title = Dormitory #5 b.3
menu-d5k3-content =
    🏢 Dormitory #5 building 3

    📍 197046, St. Petersburg, Malaya Posadskaya st., 22-24
    📞 +7(812)233-89-25
    ✉️ hostel5k3@pgups.ru

menu-d6-title = Dormitory #6
menu-d6-content =
    🏢 Dormitory #6 — 287 places (block type)

    Study room, laundry, internet.

    📍 190121, St. Petersburg, Perevoznaya st., 2
    📞 +7(812)714-82-65
    ✉️ hostel6@pgups.ru

menu-d7a-title = Dormitory #7a
menu-d7a-content =
    🏢 Dormitory #7a

    📍 197046, St. Petersburg, Malaya Posadskaya st., 22-24
    📞 +7(812)232-14-86, +7(812)232-40-68
    ✉️ hostel7a@pgups.ru

menu-d8-title = Dormitory #8
menu-d8-content =
    🏢 Dormitory #8 — 804 places (apartment type)

    Pushkin. Sports ground, gym, laundry, internet.

    📍 189620, Pushkin, Oranzhereinaya st., 73
    📞 +7(812)465-90-50, +7(812)476-17-33, +7(812)465-99-85, +7(812)465-94-86
    ✉️ hostel8@pgups.ru

# Students
menu-schedule-title = 📅 Schedule
menu-facs-title = 🏫 Faculties
menu-facs-content = Our university has 7 faculties. Select from the menu:
menu-stud_dorm-title = 🏠 Dormitories
menu-stud_dorm-content =
    Our university provides dormitories for students. 9 dormitories, over 3,400 places.

    Select a dormitory from the menu.

menu-infrastructure-title = 🏗 Infrastructure
menu-infrastructure-content = Select an infrastructure facility:
menu-scholarship-title = 💰 Scholarships
menu-scholarship-content =
    💰 Scholarships and financial aid:

    1. State academic scholarship
    2. State social scholarship
    3. Presidential scholarship
    4. Government scholarship
    5. Enhanced academic scholarship for achievements
    6. Enhanced social scholarship (1st–2nd year)
    7. Increased scholarship for talented applicants
    8. Named scholarships

    Nominations are made by faculties.

menu-portfolio-title = 📂 Portfolio
menu-portfolio-content =
    📂 Portfolio guidelines:
    https://vk.com/wall-102162446_2965

menu-struct-title = 🏛 Organizations
menu-struct-content =
    🏛 Student organizations:

    1. Faculty student councils
    2. Volunteer programs
    3. Dormitory student councils
    4. Cultural center
    5. Student scientific society
    6. TERRA — eco club
    7. Media
    8. Faculty trade union
    9. Patriotic center
    10. Student patrol
    11. Sports team
    12. Student brigades

menu-map-title = 🗺 Campus Map
menu-map-content = 🗺 Campus Map

# Faculties
menu-ait-title = AIT
menu-ait-content =
    🏫 Faculty of Automation and Intelligent Technologies
    https://www.pgups.ru/struct/avtomatizatsiya-i-intellektualnye-tekhnologii/

    👤 Dean: Stepanskaya Olga Andreevna
    📞 +7(812)570-52-37
    📍 Moskovsky pr., 9, room 7-420

menu-bfo-title = BFO
menu-bfo-content =
    🏫 Faculty of Part-Time Education
    https://www.pgups.ru/struct/fakultet_bezotryvnykh_form_obucheniya/

    👤 Dean: Kutsenko Sergey Mikhailovich
    ✉️ fbfo@pgups.ru
    📞 +7(812)457-81-17, +7(812)436-92-66
    📍 Moskovsky pr., 9, room 7-524

menu-eim-title = E&M
menu-eim-content =
    🏫 Faculty of Economics and Management
    https://www.pgups.ru/struct/ekonomika-i-menedzhment/

    👤 Dean: Kabanov Alexander Vasilievich
    ✉️ eim@pgups.ru
    📞 +7(812)407-40-39, +7(812)457-85-46
    📍 Fontanka emb., 115, room 9-124

menu-pgs-title = PGS
menu-pgs-content =
    🏫 Faculty of Industrial and Civil Construction
    https://www.pgups.ru/struct/promyshlennoe_i_grazhdanskoe_stroitelstvo/

    👤 Dean: Abu-Khasan Mahmud Samievich
    ✉️ pgs@pgups.ru
    📞 +7(812)315-13-32, +7(812)457-81-09
    📍 Moskovsky pr., 9, room 2-224

menu-tes-title = TES
menu-tes-content =
    🏫 Faculty of Transport and Energy Systems
    https://www.pgups.ru/struct/transportnye-i-energeticheskie-sistemy/

    👤 Dean: Chuyan Sergey Nikolaevich
    ✉️ tes@pgups.ru
    📞 +7(812)315-40-53, +7(812)457-81-14
    📍 Moskovsky pr., 9, room 4-205

menu-ts-title = TS
menu-ts-content =
    🏫 Faculty of Transport Construction
    https://www.pgups.ru/struct/transportnoe_stroitelstvo/

    👤 Dean: Bushuev Nikolay Sergeevich
    📞 +7(812)315-18-54
    📍 Moskovsky pr., 9, room 1-310, 1-407

menu-upl-title = UPL
menu-upl-content =
    🏫 Faculty of Transportation Management and Logistics
    https://www.pgups.ru/struct/upravlenie-perevozkami-i-logistika/

    👤 Dean: Badetsky Alexander Petrovich
    ✉️ upl@pgups.ru
    📞 +7(812)310-65-75, +7(812)457-81-10
    📍 Moskovsky pr., 9, room 7-326

# Dormitories for students
menu-sd1-title = Dormitory #1
menu-sd1-content =
    🏢 Dormitory #1

    👤 Manager: Mikusheva Roza Vakilievna
    📞 +7(812)713-02-54
    ✉️ hostel1@pgups.ru
    📍 7th Krasnoarmeyskaya st., 12

menu-sd2-title = Dormitory #2
menu-sd2-content =
    🏢 Dormitory #2

    👤 Manager: Zhang Fyodor Vladimirovich
    📞 +7(812)233-72-07
    ✉️ hostel2@pgups.ru
    📍 Kronverksky pr., 9

menu-sd3-title = Dormitory #3
menu-sd3-content =
    🏢 Dormitory #3

    👤 Manager: Kovalev Igor Viktorovich
    ✉️ hostel3@pgups.ru
    📞 +7(812)571-02-29, +7(812)314-96-19
    📍 Stolyarny per., 6

menu-sd4-title = Dormitory #4
menu-sd4-content =
    🏢 Dormitory #4

    👤 Manager: Nikitina Olga Yurievna
    ✉️ hostel4@pgups.ru
    📞 +7(812)436-98-66, +7(812)457-89-05
    📍 Fontanka emb., 113

menu-sd5-title = Dormitory #5
menu-sd5-content =
    🏢 Dormitory #5

    👤 Manager: Durkina Svetlana Alexandrovna
    ✉️ hostel5@pgups.ru
    📞 +7(812)233-54-30, +7(812)232-35-76, +7(812)233-11-74
    📍 Malaya Posadskaya st., 22-24

menu-sd5k3-title = Dormitory #5 b.3
menu-sd5k3-content =
    🏢 Dormitory #5 building 3

    👤 Manager: Savitsky Pavel Pavlovich
    ✉️ hostel5k3@pgups.ru
    📞 +7(812)233-89-25, +7(812)233-27-03
    📍 Malaya Posadskaya st., 22-24

menu-sd6-title = Dormitory #6
menu-sd6-content =
    🏢 Dormitory #6

    👤 Manager: Kuchuk Dmitry Iosifovich
    📞 +7(812)714-82-65
    ✉️ hostel6@pgups.ru
    📍 Perevoznaya st., 2

menu-sd7a-title = Dormitory #7a
menu-sd7a-content =
    🏢 Dormitory #7a

    👤 Manager: Savitsky Pavel Pavlovich
    ✉️ hostel7a@pgups.ru
    📞 +7(812)232-14-86, +7(812)232-40-68, +7(812)232-76-84
    📍 Malaya Posadskaya st., 22-24

menu-sd8-title = Dormitory #8
menu-sd8-content =
    🏢 Dormitory #8

    👤 Manager: Kozhevnikov Alexander Anatolievich
    ✉️ hostel8@pgups.ru
    📞 +7(812)465-90-50, +7(812)476-17-33, +7(812)465-99-85, +7(812)465-94-86
    📍 Pushkin, Oranzhereinaya st., 73

# Infrastructure
menu-canteens-title = 🍽 Canteens
menu-canteens-content =
    🍽 University canteens:

    • Room 7-205
    • Room 7-206
    • Room 1-219
    • Room 3-208

menu-libraries-title = 📚 Libraries
menu-libraries-content =
    📚 University libraries:

    • Main library — room 3-207
    • Scientific-technical — room 6-211
    • Social-political literature — room 1-309
    • Fiction — room 1-314

    ⚠️ Library card is issued in the first month of study in room 3-207.

menu-hospital-title = 🏥 Clinic
menu-hospital-content =
    🏥 University clinic:

    📍 St. Petersburg, Fontanka emb., 115

# Teachers
menu-prof_docs-title = 🔗 Useful Links
menu-prof_docs-content =
    🔗 Useful links:

    📄 Documents
    https://www.pgups.ru/sveden/document/

    📢 Competitions
    https://www.pgups.ru/university/general-information/ads/

    ✉️ Corporate email
    https://www.pgups.ru/struct/upravlenie_informatizatsii/korporativnaya-pochta

    🤝 Trade union
    https://www.pgups.ru/employees/the-trade-union/

# Onboarding
msg-onboarding-welcome = Welcome! Let us set up the bot for you.
msg-onboarding-ask-lang = Select language:
msg-onboarding-ask-role = Select your role:
msg-onboarding-ask-group = Enter your group number (e.g. IVB-211):
msg-onboarding-skip-group = You can skip this step.
msg-onboarding-complete = Setup complete!

# Roles
role-applicant = 🎓 Applicant
role-student = 👨‍🎓 Student
role-teacher = 👩‍🏫 Teacher
role-guest = 👤 Guest

# Settings
msg-settings = Settings
msg-settings-lang = Language: { $lang }
msg-settings-role = Role: { $role }
msg-settings-group = Group: { $group }
btn-settings = ⚙️ Settings
btn-settings-change-lang = Change language
btn-settings-change-role = Change role
btn-settings-change-group = Change group
btn-skip = Skip
');

-- ========================================
-- 中文
-- ========================================
INSERT INTO translations (lang, content) VALUES ('zh', '
# 导航按钮
btn-back = ⬅ 返回
btn-home = 🏠 首页
btn-language = 🌐 语言

# 课表按钮
btn-schedule-today = 今天
btn-schedule-tomorrow = 明天
btn-schedule-this-week = 本周
btn-schedule-next-week = 下周
btn-schedule-change-group = 🔄 更换班级

# 系统消息
msg-select-section = 请选择：
msg-ask-group = 请输入您的班级编号（例如 ИВБ-211）：
msg-schedule-header = { $group } 班级课表
    今天：{ $weekday }，{ $parity }周
msg-schedule-day-off = 休息日，没有课程。
msg-schedule-error = 获取课表时出错。
msg-schedule-server-down = 课表服务器无响应。
msg-schedule-connection-error = 无法连接到课表服务器。
msg-schedule-week-error = 无法获取本周课表。
msg-schedule-invalid-group = 未找到该班级，请检查输入。
msg-select-language = 请选择语言：
msg-language-changed = 语言已更改。

# 课表
schedule-no-lessons = 没有课程。
schedule-pair = 节课
schedule-room = 教室
schedule-weekday-monday = 星期一
schedule-weekday-tuesday = 星期二
schedule-weekday-wednesday = 星期三
schedule-weekday-thursday = 星期四
schedule-weekday-friday = 星期五
schedule-weekday-saturday = 星期六
schedule-weekday-sunday = 星期日
schedule-parity-odd = 单
schedule-parity-even = 双

# 菜单：根
menu-start-title = 🏠 主菜单
menu-start-content = 欢迎！请选择：

# 菜单：一级
menu-info-title = ℹ️ 信息
menu-info-content = 在这里您可以了解大学的各种信息。请选择类别：
menu-about-title = 📌 关于项目
menu-about-content = 便捷PGUPS——由自动化与智能技术学院学生社团开发的项目。

# 菜单：类别
menu-abit-title = 🎓 招生信息
menu-abit-content = 请选择信息：
menu-stud-title = 👨‍🎓 学生信息
menu-stud-content = 请选择信息：
menu-prof-title = 👩‍🏫 教师信息
menu-prof-content = 请选择信息：

# 招生
menu-docs-title = 📋 文件
menu-docs-content =
    您可以通过个人账户、政府服务或亲自提交文件。

    📎 通过个人账户提交，请在 https://abit.pgups.ru/ 注册

    📎 通过政府服务提交，填写申请并注明考试科目。

    📎 亲自提交，请前往招生办。
    工作时间：周一至周五 9:00–17:00，周六 9:00–16:00。

    ⚠️ 入学需提供教育文件原件和入学同意书。

menu-dorm-title = 🏢 宿舍
menu-dorm-content =
    我校为学生提供宿舍。共9栋宿舍，3400多个床位。

    请从菜单中选择宿舍编号。

menu-calendar-title = 📅 招生日历
menu-calendar-content =
    📅 招生日历：

    📌 6月19日 — 开始接收文件
    📌 7月25日 — 截止接收（统考成绩）
    📌 8月19日 — 自费截止
    📌 8月29日 — 合同签订截止

menu-contacts-title = 📞 联系方式
menu-contacts-content =
    📞 招生办联系方式：

    🌍 外国公民：
    📞 8(812)310-17-27 或 8(812)315-40-59
    ✉️ edusupport@pgups.ru

menu-exam-title = 📝 入学考试
menu-exam-content =
    以下类别可在PGUPS参加入学考试：

    • 中等职业教育毕业生
    • 残疾人
    • 外国公民
    • 硕士申请人

menu-rules-title = 📖 招生规则
menu-rules-content =
    📖 招生规则请参阅招生网站：
    https://priem.pgups.ru/info

menu-3d-title = 🖥 3D参观
menu-3d-content =
    虚拟参观我们的大学！🏛

    http://3d.pgups.ru/

# 宿舍（招生）
menu-d1-title = 1号宿舍
menu-d1-content =
    🏢 1号宿舍 — 644个床位
    📍 圣彼得堡，第七红军街12号
    ✉️ hostel1@pgups.ru
    📞 +7(812)712-66-15

menu-d2-title = 2号宿舍
menu-d2-content =
    🏢 2号宿舍 — 288个床位（公寓型）
    📍 圣彼得堡，克朗维尔克大街9号
    📞 +7(812)233-72-07

menu-d3-title = 3号宿舍
menu-d3-content =
    🏢 3号宿舍 — 178个床位
    📍 圣彼得堡，斯托利亚尔尼巷6号
    📞 +7(812)571-02-29

menu-d4-title = 4号宿舍
menu-d4-content =
    🏢 4号宿舍 — 262个床位
    📍 圣彼得堡，丰坦卡河堤113号
    📞 +7(812)436-98-66

menu-d5-title = 5号宿舍
menu-d5-content =
    🏢 5号宿舍
    📍 圣彼得堡，小波萨茨卡亚街22-24号
    📞 +7(812)233-54-30

menu-d5k3-title = 5号宿舍3栋
menu-d5k3-content =
    🏢 5号宿舍3栋
    📍 圣彼得堡，小波萨茨卡亚街22-24号
    📞 +7(812)233-89-25

menu-d6-title = 6号宿舍
menu-d6-content =
    🏢 6号宿舍 — 287个床位
    📍 圣彼得堡，佩列沃兹纳亚街2号
    📞 +7(812)714-82-65

menu-d7a-title = 7а号宿舍
menu-d7a-content =
    🏢 7а号宿舍
    📍 圣彼得堡，小波萨茨卡亚街22-24号
    📞 +7(812)232-14-86

menu-d8-title = 8号宿舍
menu-d8-content =
    🏢 8号宿舍 — 804个床位（公寓型）
    📍 普希金市，奥兰热列伊纳亚街73号
    📞 +7(812)465-90-50

# 学生
menu-schedule-title = 📅 课表
menu-facs-title = 🏫 学院
menu-facs-content = 我校共有7个学院。请从菜单中选择：
menu-stud_dorm-title = 🏠 宿舍
menu-stud_dorm-content = 我校为学生提供宿舍。共9栋宿舍，3400多个床位。
menu-infrastructure-title = 🏗 基础设施
menu-infrastructure-content = 请选择基础设施：
menu-scholarship-title = 💰 奖学金
menu-scholarship-content =
    💰 奖学金和经济援助：

    1. 国家学术奖学金
    2. 国家社会奖学金
    3. 总统奖学金
    4. 政府奖学金
    5. 优秀成就奖学金

menu-portfolio-title = 📂 作品集
menu-portfolio-content =
    📂 作品集填写指南：
    https://vk.com/wall-102162446_2965

menu-struct-title = 🏛 组织
menu-struct-content =
    🏛 学生组织：

    1. 学院学生会
    2. 志愿者项目
    3. 宿舍学生会
    4. 文化中心
    5. 学生科学协会

menu-map-title = 🗺 校园地图
menu-map-content = 🗺 校园地图

# 学院
menu-ait-title = 自动化与智能技术学院
menu-ait-content =
    🏫 自动化与智能技术学院
    👤 院长：斯捷潘斯卡娅·奥尔加·安德烈耶夫娜
    📞 +7(812)570-52-37

menu-bfo-title = 函授教育学院
menu-bfo-content =
    🏫 函授教育学院
    👤 院长：库岑科·谢尔盖·米哈伊洛维奇
    📞 +7(812)457-81-17

menu-eim-title = 经济与管理学院
menu-eim-content =
    🏫 经济与管理学院
    👤 院长：卡巴诺夫·亚历山大·瓦西里耶维奇
    📞 +7(812)407-40-39

menu-pgs-title = 工业与民用建筑学院
menu-pgs-content =
    🏫 工业与民用建筑学院
    👤 院长：阿布-哈桑·马赫穆德
    📞 +7(812)315-13-32

menu-tes-title = 运输与能源系统学院
menu-tes-content =
    🏫 运输与能源系统学院
    👤 院长：楚扬·谢尔盖
    📞 +7(812)315-40-53

menu-ts-title = 运输建设学院
menu-ts-content =
    🏫 运输建设学院
    👤 院长：布舒耶夫·尼古拉
    📞 +7(812)315-18-54

menu-upl-title = 运输管理与物流学院
menu-upl-content =
    🏫 运输管理与物流学院
    👤 院长：巴杰茨基·亚历山大
    📞 +7(812)310-65-75

# 学生宿舍
menu-sd1-title = 1号宿舍
menu-sd1-content =
    🏢 1号宿舍
    👤 管理员：米库舍娃·罗扎
    📞 +7(812)713-02-54

menu-sd2-title = 2号宿舍
menu-sd2-content =
    🏢 2号宿舍
    👤 管理员：张·费奥多尔
    📞 +7(812)233-72-07

menu-sd3-title = 3号宿舍
menu-sd3-content =
    🏢 3号宿舍
    👤 管理员：科瓦列夫·伊戈尔
    📞 +7(812)571-02-29

menu-sd4-title = 4号宿舍
menu-sd4-content =
    🏢 4号宿舍
    👤 管理员：尼基季娜·奥尔加
    📞 +7(812)436-98-66

menu-sd5-title = 5号宿舍
menu-sd5-content =
    🏢 5号宿舍
    👤 管理员：杜尔金娜·斯韦特兰娜
    📞 +7(812)233-54-30

menu-sd5k3-title = 5号宿舍3栋
menu-sd5k3-content =
    🏢 5号宿舍3栋
    👤 管理员：萨维茨基·帕维尔
    📞 +7(812)233-89-25

menu-sd6-title = 6号宿舍
menu-sd6-content =
    🏢 6号宿舍
    👤 管理员：库丘克·德米特里
    📞 +7(812)714-82-65

menu-sd7a-title = 7а号宿舍
menu-sd7a-content =
    🏢 7а号宿舍
    👤 管理员：萨维茨基·帕维尔
    📞 +7(812)232-14-86

menu-sd8-title = 8号宿舍
menu-sd8-content =
    🏢 8号宿舍
    👤 管理员：科热夫尼科夫·亚历山大
    📞 +7(812)465-90-50

# 基础设施
menu-canteens-title = 🍽 食堂
menu-canteens-content =
    🍽 大学食堂：
    • 7-205教室
    • 7-206教室
    • 1-219教室
    • 3-208教室

menu-libraries-title = 📚 图书馆
menu-libraries-content =
    📚 大学图书馆：
    • 主图书馆 — 3-207教室
    • 科技图书馆 — 6-211教室

menu-hospital-title = 🏥 诊所
menu-hospital-content =
    🏥 大学诊所：
    📍 圣彼得堡，丰坦卡河堤115号

# 教师
menu-prof_docs-title = 🔗 有用链接
menu-prof_docs-content =
    🔗 有用链接：

    📄 文件
    https://www.pgups.ru/sveden/document/

    ✉️ 企业邮箱
    https://www.pgups.ru/struct/upravlenie_informatizatsii/korporativnaya-pochta

# 入门引导
msg-onboarding-welcome = 欢迎！让我们为您设置机器人。
msg-onboarding-ask-lang = 请选择语言：
msg-onboarding-ask-role = 请选择您的角色：
msg-onboarding-ask-group = 请输入您的班级编号（例如 ИВБ-211）：
msg-onboarding-skip-group = 您可以跳过此步骤。
msg-onboarding-complete = 设置完成！

# 角色
role-applicant = 🎓 申请人
role-student = 👨‍🎓 学生
role-teacher = 👩‍🏫 教师
role-guest = 👤 访客

# 设置
msg-settings = 设置
msg-settings-lang = 语言：{ $lang }
msg-settings-role = 角色：{ $role }
msg-settings-group = 班级：{ $group }
btn-settings = ⚙️ 设置
btn-settings-change-lang = 更换语言
btn-settings-change-role = 更换角色
btn-settings-change-group = 更换班级
btn-skip = 跳过
');
