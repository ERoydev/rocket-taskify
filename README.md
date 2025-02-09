# Task Management API Documentation

This API manages tasks with basic CRUD operations.

# Tech Stack

## 🚀 Rust
- Systems programming language for performance and safety.
- Guarantees memory safety and concurrency without data races.

## ⚡ Rocket Framework
- Web framework for Rust, ideal for building fast, secure web applications.
- Supports asynchronous programming with `async/await`.
- Type-safe routing and easy request handling (JSON, forms, etc.).

## 🌊 SeaORM
- Asynchronous ORM for Rust, built for SQL databases.
- Supports PostgreSQL, MySQL, and SQLite.
- Provides type-safe, efficient database interactions with migrations.

$~~~~~~~~~~~$
$~~~~~~~~~~~$
# Endpoints

### GET /tasks
Fetch all tasks, optionally sorted by priority.

**Query Parameters:**
- `sort`: Filter tasks by priority (`high`, `medium`, `low`, `expired`, `immediate`).

- `/tasks?sort=high` => returns all tasks sorted by high priority

**Response:**
- `200 OK`: Array of tasks.


$~~~~~~~~~~~$
### POST /tasks
Create a new task.

**Request Body:**
```json
{
    "title": "Some title",
    "description": "Some description",
    "due_date": 1738971697, // Unix Timestamp 
    "is_completed": false,
    "is_critical": false
}
```

$~~~~~~~~~~~$
### DELETE /tasks/id
Delete a task


$~~~~~~~~~~~$
### PUT /tasks
Update a task with new fields

**Request Body:**
```json
{
    "id": 12,
    "title": "Sometitle",
    "description": "Some description",
    "priority": "expired",
    "due_date": "07-02-25",
    "is_completed": false,
    "is_critical": false,
    "due_date_timestamp": 1738971697
}
```

$~~~~~~~~~~~$
### GET /tasks?filter=isCompleted&value=true
*Get tasks by completion status provide value either true or false.*


$~~~~~~~~~~~$

### Task Priority Levels
```rs
enum TaskPriorityLevel {
    Low,
    Medium,
    High,
    Immediate,
    Expired,
}
```
$~~~~~~~~~~~$
$~~~~~~~~~~~$
$~~~~~~~~~~~$
---
<table>
  <thead>
    <tr>
      <th>HTTP Method</th>
      <th>Endpoint</th>
      <th>Description</th>
    </tr>
  </thead>
  <tbody>
      <tr>
      <td>GET</td>
      <td>/tasks</td>
      <td>
        Retrieves all tasks sorted by priority (DESC) and due date (ASC) by default.
      </td>
    </tr>
    <tr>
      <td>GET</td>
      <td>/tasks?sort=<em>priority</em></td>
      <td>
        Retrieves all tasks sorted by priority (DESC) and due date (ASC). If a sort query parameter is provided, it filters tasks by the specified priority level (e.g. "high").
      </td>
    </tr>
    <tr>
      <td>GET</td>
      <td>/tasks?filter=isCompleted&amp;value=<em>true/false</em></td>
      <td>
        Retrieves tasks filtered by their completion status. Use <code>filter=isCompleted</code> with <code>value=true</code> or <code>value=false</code>.
      </td>
    </tr>
    <tr>
      <td>GET</td>
      <td>/tasks/&lt;id&gt;</td>
      <td>Retrieves the task with the specified ID.</td>
    </tr>
    <tr>
      <td>POST</td>
      <td>/tasks</td>
      <td>
        Creates a new task. Expects task details in JSON (title, description, due_date, is_completed, is_critical).
      </td>
    </tr>
    <tr>
      <td>PUT</td>
      <td>/tasks</td>
      <td>Updates an existing task. Expects updated task data in JSON.</td>
    </tr>
    <tr>
      <td>DELETE</td>
      <td>/tasks/&lt;id&gt;</td>
      <td>Deletes the task with the specified ID.</td>
    </tr>
    <tr>
      <td>POST</td>
      <td>/tasks/complete/&lt;id&gt;</td>
      <td>
        Marks the task with the specified ID as complete (and resets the critical flag) and updates its priority accordingly.
      </td>
    </tr>
    <tr>
      <td>POST</td>
      <td>/tasks/critical/&lt;id&gt;</td>
      <td>
        Marks the task with the specified ID as critical and updates its priority accordingly.
      </td>
    </tr>
    <tr>
      <td>POST</td>
      <td>/tasks/update_priority</td>
      <td>
        Triggers an update of task priorities for all tasks based on the defined business logic.
      </td>
    </tr>
  </tbody>
</table>

$~~~~~~~~~~~$
# Frontend Implementation
*I have implemented frontend just for me to experiment.*
- It's not fully working thats why i don't provide instructions for it, anyway my task was for backend.

![Screenshot 2025-02-09 023127](https://github.com/user-attachments/assets/a9236cfa-bfc9-43e5-931d-8193cca25b61)


