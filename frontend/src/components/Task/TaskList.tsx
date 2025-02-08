import { useEffect, useState } from "react";
import CrossedTaskItem from "./CrossedTaskItem";
import TaskItem from "./TaskItem";
import { getTasks } from "../../api/TaskApi";


export default function TaskList() {
    const [tasks, setTasks] = useState<any[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        // Fetch tasks when the component mounts
        const fetchTasks = async () => {
          try {
            const fetchedTasks = await getTasks();  // Get tasks function
            setTasks(fetchedTasks);  // Set tasks in state
          } catch (err) {
            setError('Failed to fetch tasks');  // Handle error
          } finally {
            setLoading(false);  // Set loading state to false once done
          }
        };
    
        fetchTasks();  // Call the function to fetch tasks
      }, [])

    console.log(tasks)

    return(
        <div className="antialiased bg-white text-slate-700 p-20 pb-30" id="tasklist">
            <div className="max-w-[1400px] mx-auto my-10 bg-white p-8 rounded-xl shadow shadow-slate-300">
                <div className="flex flex-row justify-between items-center">
                    <div>
                        <h1 className="text-3xl font-medium">Tasks list</h1>
                    </div>
                    
                    {/* SIDE BUTTONS */}
                    <div className="inline-flex space-x-2 items-center">
                        <a href="#" className="p-2 border border-slate-200 rounded-md inline-flex space-x-1 items-center text-indigo-200 hover:text-white bg-indigo-600 hover:bg-indigo-500">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span className="text-sm font-medium hidden md:block">Urgent</span>                     
                        </a>
                        <a href="#" className="p-2 border border-slate-200 rounded-md inline-flex space-x-1 items-center hover:bg-slate-200">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                            </svg> 
                            <span className="text-sm hidden md:block">Latest</span>                    
                        </a>
                    </div>
                </div>
                <p className="text-slate-500">Hello, here are your latest tasks</p>


                <div id="tasks" className="my-5">
                    <CrossedTaskItem />
       
                    <TaskItem />
                </div>


                <p className="text-xs text-slate-500 text-center">Last updated 12 minutes ago</p>
            </div>
        </div>
    );
}