# From Objects to Functions in Rust

## 📕 Zettai API

### 📥 GET `/todo/{user_name}`

사용자의 모든 To-Do 리스트를 조회. 존재하지 않는 사용자 이름으로 요청할 경우, 해당 이름의 새 유저와 빈 To-Do 리스트 목록을 자동으로 생성한 뒤 반환

- **Method**: `GET`
- **Path Parameters**:
  - `user_name`: 사용자 이름
- **Response**:
  - `200 OK: To-Do 리스트 목록 HTML 페이지 반환

### 📥 POST `/todo/{user_name}`

새로운 To-Do 리스트 추가.

- **Method**: `POST`
- **Path Parameters**:
  - `user_name`: 사용자 이름
- **Content-Type**: `application/x-www-form-urlencoded`
- **Request Body**:
  - `list_name`: 리스트 이름
- **Response**:
  - `303 See Other`: 리스트 추가 성공 → To-Do 리스트 목록 페이지로 Redirection
    - `Location` 헤더: `/todo/{user_name}`

### 📥 GET `/todo/{user_name}/{list_name}`

사용자의 특정 To-Do 리스트에 있는 모든 항목을 조회.

- **Method**: `GET`
- **Path Parameters**:
  - `user_name`: 사용자 이름
  - `list_name`: 리스트 이름
- **Response**:
  - `200 OK: 특정 To-Do 리스트 HTML 페이지 반환

### 📥 POST `/todo/{user_name}/{list_name}`

새로운 To-Do 항목을 사용자의 특정 리스트에 추가.

- **Method**: `POST`
- **Path Parameters**:
    - `user_name`: 사용자 이름
    - `list_name`: 리스트 이름
- **Content-Type**: `application/x-www-form-urlencoded`
- **Request Body**:
    - `item_name`: To-Do 항목
    - `due_date`: To-Do 항목의 마감일 (YYYY-MM-DD)
    - `status`: 항목의 상태 (Todo, InProgress, Done, Blocked)
- **Response**:
    - `303 See Other`: 항목 추가 성공 → To-Do 리스트 페이지로 Redirection
        - `Location` 헤더: `/todo/{user_name}/{list_name}`