export default interface Task {
    id: number,
    title: string,
    description: string,
    priority: string,
    due_date: string,
    is_completed: boolean,
    is_critical:boolean,
    due_date_timestamp: number
}