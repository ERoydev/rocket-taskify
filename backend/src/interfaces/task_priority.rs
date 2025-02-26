use chrono::Local;

/*
    This module is created to handle all priority logic for the tasks;
    It is designed to be encapsulated and used outside only with get_priority_level() function

    With test cases bellow

*/

pub fn get_priority_level(is_completed: bool, is_critical: bool, due_date: i64) -> String {
    let priority_instance: TaskPriority = TaskPriority::new(is_completed, is_critical, due_date);
    let priority: String = TaskPriority::get_priority_level(priority_instance);
    priority
}

// ENCAPSULATED LOGIC BELLOW
#[derive(Clone, Copy, Debug, PartialEq)]
struct TaskPriority {
    is_completed: bool,
    is_critical: bool,
    due_date: i64
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum TaskPriorityLevel {
    Low,
    Medium,
    High,
    Immediate,
    Expired,
}

impl TaskPriority {
    fn new(is_completed: bool, is_critical: bool, due_date: i64) -> TaskPriority  {
        let instance = TaskPriority {
            is_completed,
            is_critical,
            due_date
        };
        instance
    }

    fn get_priority_level(priority_instance: TaskPriority) -> String {
        let priority_level: TaskPriorityLevel;

        if priority_instance.is_completed {
            priority_level = TaskPriorityLevel::Low

        } else if priority_instance.is_critical {
            priority_level = TaskPriorityLevel::Immediate

        } else {
            let now_timestamp: i64 = Local::now().timestamp(); // Generate current time
            priority_level = Self::calculate_priority_based_on_due_date(priority_instance.due_date, now_timestamp)
        }

        let priority_string: String = Self::level_string_representation(priority_level).to_string();
        priority_string
    }

    fn level_string_representation(priority_level: TaskPriorityLevel) -> &'static str {
        match priority_level {
            TaskPriorityLevel::Low => "low",
            TaskPriorityLevel::Medium => "medium",
            TaskPriorityLevel::High => "high",
            TaskPriorityLevel::Immediate => "immediate",
            TaskPriorityLevel::Expired => "expired",
        }
    }

    pub fn calculate_priority_based_on_due_date(due_date: i64, now_timestamp: i64) -> TaskPriorityLevel {
        let difference: i64 = due_date - now_timestamp;
    
        const ONE_DAY_IN_SECONDS: i64 = 86400;
    
        const IMMEDIATE_THRESHOLD: i64 = ONE_DAY_IN_SECONDS;  // 1 Day
        const HIGH_THRESHOLD: i64 = ONE_DAY_IN_SECONDS * 2;     // 2 Days
        const MEDIUM_THRESHOLD: i64 = ONE_DAY_IN_SECONDS * 3; // 3 Days
        // Else is LOW
    
        // Assign priority based on time difference
        if difference <= 0 {
            return TaskPriorityLevel::Expired
        } else if difference <= IMMEDIATE_THRESHOLD {
            return TaskPriorityLevel::Immediate 
        } else if difference <= HIGH_THRESHOLD {
            return TaskPriorityLevel::High
        } else if difference <= MEDIUM_THRESHOLD {
            return TaskPriorityLevel::Medium
        } else {
            return TaskPriorityLevel::Low
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::NewTask;
    use super::*;

    const INITIAL_TIMESTAMP: i64 = 1738859415; // Used to test Priority logic

    fn setup_new_task() -> NewTask {
        NewTask {
            title: "title".to_string(),
            description: "description".to_string(),
            due_date: 1738859415, // "Current DateTime"
            is_completed: false,
            is_critical: false,
        }
    }

    #[test]
    fn test_priority_low_should_be_valid() {
        let task_due_date = 1739200000; // Low

        let priority = TaskPriority::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Low, priority);
    }

    #[test]
    fn test_priority_medium_should_be_valid() {
        let task_due_date = 1739100000; // Medium

        let priority = TaskPriority::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Medium, priority);
    }

    #[test]
    fn test_priority_high_should_be_valid() {
        let task_due_date = 1739000000; // High

        let priority = TaskPriority::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::High, priority);
    }

    #[test]
    fn test_priority_immediate_should_be_valid() {
        let task_due_date = 1738859999; // Immediate

        let priority = TaskPriority::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Immediate, priority);
    }

    #[test]
    fn test_priority_expired_should_be_valid() {
        let task_due_date = 1738858415; // Expired

        let priority = TaskPriority::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Expired, priority);
    }

    #[test]
    fn test_priority_when_is_completed_returns_low_priority() {
        let mut task_model = setup_new_task();

        task_model.is_completed = true;

        let priority = get_priority_level(task_model.is_completed, task_model.is_critical, task_model.due_date);

        assert_eq!("low".to_string(), priority);
    }

    #[test]
    fn test_priority_when_is_critical_returns_immediate_priority() {
        let mut task_model = setup_new_task();

        task_model.is_critical = true;

        let priority = get_priority_level(task_model.is_completed, task_model.is_critical, task_model.due_date);

        assert_eq!("immediate".to_string(), priority);
    }

    #[test]
    fn test_priority_when_is_completed_and_is_critial_both_returns_low_priority() {
        let mut task_model = setup_new_task();

        task_model.is_critical = true;
        task_model.is_completed = true;

        let priority = get_priority_level(task_model.is_completed, task_model.is_critical, task_model.due_date);

        assert_eq!("low".to_string(), priority);
    }
}