use chrono::Local;

use crate::NewTask;



#[derive(Clone, Copy)]
pub enum TaskPriorityLevel {
    Low,
    Medium,
    High,
    Immediate
}

impl TaskPriorityLevel {
    pub fn get_priority(model: &NewTask) -> String {
        // Get Priority Level STRING
        let priority_level: TaskPriorityLevel = Self::get_priority_level(model); 
        let priority = priority_level.level_string_representation().to_string(); //Priority Level STRING

        priority
    }

    fn level_string_representation(&self) -> &'static str {
        match self {
            TaskPriorityLevel::Low => "Low",
            TaskPriorityLevel::Medium => "Medium",
            TaskPriorityLevel::High => "High",
            TaskPriorityLevel::Immediate => "Immediate",
        }
    }

    fn get_priority_level(model: &NewTask) -> TaskPriorityLevel {
        // Get Priority Level Status
        if model.is_completed {
            TaskPriorityLevel::Low
        } else if model.is_critical {
            TaskPriorityLevel::Immediate
        } else {
            Self::calculate_priority_based_on_due_date(model.due_date)
        }

    }

    fn calculate_priority_based_on_due_date(due_date: i32) -> TaskPriorityLevel {
        let now_timestamp: i64 = Local::now().timestamp();
        let due_date_timestamp: i64 = due_date as i64;
    
        let difference: i64 = due_date_timestamp - now_timestamp;
    
        const ONE_DAY_IN_SECONDS: i64 = 86400;
    
        const IMMEDIATE_THRESHOLD: i64 = ONE_DAY_IN_SECONDS;  // 1 Day
        const HIGH_THRESHOLD: i64 = ONE_DAY_IN_SECONDS * 2;     // 2 Days
        const MEDIUM_THRESHOLD: i64 = ONE_DAY_IN_SECONDS * 3; // 3 Days
        // Else is LOW
    
        // Assign priority based on time difference
        if difference <= IMMEDIATE_THRESHOLD {
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
