# Task Management API Documentation

This API manages tasks with basic CRUD operations.

## Table of Contents
- [Tech Stack](#tech-stack)
- [Rocket Taskify Installation Guide](#rocket-taskify-installation-guide)
  - [Install Rust](#1-install-rust)
  - [Install PostgreSQL](#2-install-postgresql)
  - [Create the Database](#3-create-the-database)
  - [Configure Database Connection](#4-configure-database-connection)
  - [Apply Migrations](#5-apply-migrations)
  - [Run the Project](#6-run-the-project)
- [Endpoints](#endpoints)
  - [GET /tasks](#get-tasks)
  - [POST /tasks](#post-tasks)
  - [DELETE /tasks/id](#delete-tasksid)
  - [PUT /tasks](#put-tasks)
  - [GET /tasks?filter=isCompleted&value=true](#get-tasksfilteriscompletedvaluetrue)
  - [Task Priority Levels](#task-priority-levels)
- [Frontend Implementation](#frontend-implementation)
- [Future Improvements](#future-improvements)

$~~~~~~~~~~~$
$~~~~~~~~~~~$
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
# Rocket Taskify Installation Guide

This guide will help you set up and run the Rocket Taskify project using the Rocket framework.

## Prerequisites
Before setting up the project, ensure you have the following installed:

### 1. Install Rust
Rust is required to run this project. Install Rust using `rustup`:

```sh
Make sure you are using rustc 1.84.1 version
```

After installation, restart your terminal and verify the installation:

```sh
rustc --version
```

### 2. Install PostgreSQL
This project requires PostgreSQL as the database. Install it based on your operating system:

- **Ubuntu/Debian:**
  ```sh
  sudo apt update
  sudo apt install postgresql postgresql-contrib
  ```
- **MacOS (Homebrew):**
  ```sh
  brew install postgresql
  ```
- **Windows (Chocolatey):**
  ```sh
  choco install postgresql
  ```

### 3. Create the Database
Start the PostgreSQL service and create a database named `rocket_taskify`:

```sh
psql -U postgres
CREATE DATABASE rocket_taskify;
\q
```

Alternatively, use a single command:

```sh
createdb -U postgres rocket_taskify
```

### 4. Configure Database Connection
Navigate to the backend directory of the project:

```sh
cd ./backend/src
```

Edit `setup.rs` to include the correct database URL:

```rust
const DATABASE_URL: &str = "postgresql://<username>:<password>@localhost:5432/rocket_taskify";
```

Replace `<username>` and `<password>` with your actual PostgreSQL credentials.

### 5. Apply Migrations

To apply the database migrations, follow these steps:

1. **Navigate to the backend folder**:
   ```bash
   cd ./backend

2. Run the migration command: Replace <username> and <password> with your PostgreSQL credentials:

```sh
sea-orm-cli migrate up -u postgresql://<username>:<password>@localhost:5432/rocket_taskify
```
Example:
```sh
sea-orm-cli migrate up -u postgresql://admin:mysecurepassword@localhost:5432/rocket_taskify

```

### 6. Run the Project
Finally, start the Rocket server by running:

```sh
cargo run
```

Your Rocket project should now be up and running!

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
- *To create a task my application use this data object*
```ru
pub struct NewTask {
    pub title: String,
    pub description: String,
    pub due_date: i64, // i use UNIX-Timestamp
    pub is_completed: bool,
    pub is_critical: bool,
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
- *To update a task the application is using this data object
```ru
pub struct TaskDTO {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub due_date: String, // i use UNIX-Timestamp
    pub is_completed: bool,
    pub is_critical: bool, // If user update
    pub due_date_timestamp: i64,
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


# Future Improvements
I want to implement frontend to consume the backend with authentication:
- i will try using solana smart contracts using anchor for education purposes.
- i will upload simple version with front and backend version to use it as a showcase project. I will implement CD/CI git action to build, test, deploy to render.

