


#[cfg(test)]
mod tests {
    use rocket_taskify::interfaces::task_priority::TaskPriorityLevel;
    use rocket_taskify::interfaces::new_task::NewTask;

    const INITIAL_TIMESTAMP: i64 = 1738859415; // Used to test Priority logic

    fn setup_new_task() -> NewTask {
        NewTask {
            title: "title".to_string(),
            description: "description".to_string(),
            due_date: 1738859415,
            is_completed: false,
            is_critical: false,
        }
    }

    #[test]
    fn test_priority_low_should_be_valid() {
        let task_due_date = 1739200000;

        let priority = TaskPriorityLevel::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Low, priority);
    }

    #[test]
    fn test_priority_medium_should_be_valid() {
        let task_due_date = 1739100000; // Medium

        let priority = TaskPriorityLevel::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Medium, priority);
    }

    #[test]
    fn test_priority_high_should_be_valid() {
        let task_due_date = 1739000000; // High

        let priority = TaskPriorityLevel::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::High, priority);
    }

    #[test]
    fn test_priority_immediate_should_be_valid() {
        let task_due_date = 1738859999; // Immediate

        let priority = TaskPriorityLevel::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Immediate, priority);
    }

    #[test]
    fn test_priority_expired_should_be_valid() {
        let task_due_date = 1738858415; // Expired

        let priority = TaskPriorityLevel::calculate_priority_based_on_due_date(task_due_date, INITIAL_TIMESTAMP);

        assert_eq!(TaskPriorityLevel::Expired, priority);
    }

    #[test]
    fn test_priority_when_is_completed_returns_low_priority() {
        let mut task_model = setup_new_task();

        task_model.is_completed = true;

        let priority = TaskPriorityLevel::get_priority(&task_model);

        assert_eq!("low".to_string(), priority);
    }

    #[test]
    fn test_priority_when_is_critical_returns_immediate_priority() {
        let mut task_model = setup_new_task();

        task_model.is_critical = true;

        let priority = TaskPriorityLevel::get_priority(&task_model);

        assert_eq!("immediate".to_string(), priority);
    }

    #[test]
    fn test_priority_when_is_completed_and_is_critial_both_returns_low_priority() {
        let mut task_model = setup_new_task();

        task_model.is_critical = true;
        task_model.is_completed = true;

        let priority = TaskPriorityLevel::get_priority(&task_model);

        assert_eq!("low".to_string(), priority);
    }
}