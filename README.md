# QuestionAPI

HTTP API на Rust для участия в GameJam.

## Конфигурация

Все основные настройки лежат в `config.yml`. Данные берутся из `questions.yml` и `items.yml`.

## Endpoints

Базовый префикс: `/api`

### Пользовательские

- `GET /api/can_start` - можно ли начать новую сессию (`true`/`false`).
- `GET /api/start` - стартует сессию, возвращает UUID-ключ (используется дальше в заголовке `Key`).
- `POST /api/send_answer` - принимает ответ пользователя и возвращает результат.
  - Заголовок: `Key: <uuid>`
  - Тело запроса: строка с ответом
  - Ответы:
    - `false` - ответ неверный
    - `true` - если дан верный ответ на последний вопрос
- `GET /api/inventory` - возвращает текущий инвентарь сессии в формате объектов из `items.yml`.
  - Заголовок: `Key: <uuid>`

### Админские

Базовый префикс: `/api/admin`

Все админские endpoint'ы требуют заголовок:

`Authorization: Bearer <admin_token>`

- `POST /api/admin/reset_session` - сбрасывает сессию и текущий индекс вопроса.
- `POST /api/admin/inventory/add` - добавляет предмет в инвентарь текущей сессии по id из `items.yml`.
  - Тело запроса: строка с одним числом (id)
- `GET /api/admin/questions` - возвращает полный список вопросов и ответов.
- `GET /api/admin/current_question` - возвращает индекс текущего вопроса.

