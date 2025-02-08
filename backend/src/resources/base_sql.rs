
pub fn get_base_sql() -> String {
    r#"
        SELECT 
            id,
            title,
            description,
            priority,
            due_date,
            is_completed,
            is_critical,
            CASE
                WHEN priority ILIKE 'expired' Then 1
                WHEN priority ILIKE 'low' Then 2
                WHEN priority ILIKE 'medium' Then 3
                WHEN priority ILIKE 'high' then 4
                WHEN priority ILIKE 'immediate' then 5
                ELSE 6
            END as priority_order

        FROM task

    "#.to_string()
}
