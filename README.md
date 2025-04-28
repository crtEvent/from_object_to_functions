# From Objects to Functions in Rust

## 📕 Zettai API

### 📥 GET `/todo/{user_name}/{list_name}`

사용자의 특정 리스트에 있는 모든 To-Do 항목을 조회.

- **Method**: `GET`
- **Path Parameters**:
  - `user_name`: 사용자 이름 (string)
  - `list_name`: 리스트 이름 (string)
- **Response**:
  - `200 OK: 리스트 HTML 페이지 반환

### 📥 POST `/todo/{user_name}/{list_name}`

새로운 To-Do 항목을 사용자의 특정 리스트에 추가.

- **Method**: `POST`
- **Path Parameters**:
    - `user_name` (string): 사용자 이름
    - `list_name` (string): 리스트 이름
- **Content-Type**: `application/x-www-form-urlencoded`
- **Request Body**:
    - `item_name` (string): To-Do 항목
    - `due_date` (string): To-Do 항목의 마감일 (YYYY-MM-DD)
    - `status` (string): 항목의 상태 (Todo, InProgress, Done, Blocked)
- **Response**:
    - `303 See Other`: 항목 추가 성공 → 리스트 페이지로 Redirection
        - `Location` 헤더: `/todo/{user_name}/{list_name}`