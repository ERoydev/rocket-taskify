import axios from "axios";

// Set the base URL for your API
const API_BASE_URL = "http://localhost:8000";

// Axios instance for easier handling
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

// Get all tasks with optional sorting by priority
export const getTasks = async (sort?: string) => {
  try {
    const response = await api.get(`/tasks`, {
      params: { sort },
    });
    return response.data;
  } catch (error) {
    console.error("Error fetching tasks:", error);
    throw error;
  }
};

// Get tasks by completion status (true/false)
const getTasksByCompletionStatus = async (isCompleted: boolean) => {
  try {
    const response = await api.get(`/tasks`, {
      params: { filter: "isCompleted", value: isCompleted.toString() },
    });
    return response.data;
  } catch (error) {
    console.error("Error fetching tasks by completion status:", error);
    throw error;
  }
};

// Get task by ID
const getTaskById = async (id: number) => {
  try {
    const response = await api.get(`/tasks/${id}`);
    return response.data;
  } catch (error) {
    console.error("Error fetching task by ID:", error);
    throw error;
  }
};

// Create a new task
const createTask = async (newTask: any) => {
  try {
    const response = await api.post(`/tasks`, newTask);
    return response.data;
  } catch (error) {
    console.error("Error creating task:", error);
    throw error;
  }
};

// Update an existing task
const updateTask = async (updatedTask: any) => {
  try {
    const response = await api.put(`/tasks`, updatedTask);
    return response.data;
  } catch (error) {
    console.error("Error updating task:", error);
    throw error;
  }
};

// Delete a task by ID
const deleteTask = async (id: number) => {
  try {
    const response = await api.delete(`/tasks/${id}`);
    return response.data;
  } catch (error) {
    console.error("Error deleting task:", error);
    throw error;
  }
};

// Mark task as complete
const completeTask = async (id: number) => {
  try {
    const response = await api.post(`/tasks/complete/${id}`);
    return response.data;
  } catch (error) {
    console.error("Error completing task:", error);
    throw error;
  }
};

// Mark task as critical
const criticalTask = async (id: number) => {
  try {
    const response = await api.post(`/tasks/critical/${id}`);
    return response.data;
  } catch (error) {
    console.error("Error marking task as critical:", error);
    throw error;
  }
};

// Update task priorities
const updateTasksPriority = async () => {
  try {
    await api.post(`/tasks/update_priority`);
  } catch (error) {
    console.error("Error updating task priorities:", error);
    throw error;
  }
};
