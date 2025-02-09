import React, { useEffect, useRef, useState } from "react";
import { getTaskById } from "../../api/TaskApi";

interface Task {
    id: number,
    title: string,
    description: string,
    priority: string,
    due_date: string,
    is_completed: boolean,
    is_critical:boolean,
    due_date_timestamp: number
}

export default function FormModal({ 
    isOpen, 
    closeModal, 
    taskId,
 }: {
    isOpen: boolean;
    closeModel: void;
    taskId?: number;
 }) {
  // Ref to detect clicks outside of modal content
    const modalRef = useRef<HTMLDivElement | null>(null);
    const backdropRef = useRef<HTMLDivElement | null>(null);
    const [task, setTask] = useState({} as Task);
  
    useEffect(() => {
        // Fetch task when modal opens and taskId changes
        if (isOpen && taskId) {
            getTaskById(taskId)
              .then(data => setTask(data)) // Convert response to JSON
              .catch(error => console.error("Error fetching task:", error));
          }
        
        // Close modal if clicked outside modal content
        const handleClickOutside = (event: MouseEvent) => {
            if (
                modalRef.current &&
                !modalRef.current.contains(event.target as Node) &&
                backdropRef.current &&
                !backdropRef.current.contains(event.target as Node)
            ) {
                closeModal(0);
            }
        };
        
        // Attach event listener
        document.addEventListener("mousedown", handleClickOutside);
        
        // Cleanup on unmount
        return () => {
            document.removeEventListener("mousedown", handleClickOutside);
        };
    }, [isOpen, taskId, closeModal]);
    
    return (
        <div className="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
            {/* Backdrop */}
            <div 
                ref={backdropRef} 
                className="fixed inset-0 bg-gray-500/75 transition-opacity" 
                aria-hidden="true"
                onClick={closeModal}
            ></div>

        <div className="fixed inset-0 z-10 w-screen overflow-y-auto">
            <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <div 
                ref={modalRef} 
                className="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg"
            >
                <div className="bg-white px-6 py-5 sm:p-6 rounded-lg shadow-md">
                    <div className="flex flex-col gap-4 justify-center">
                        <form>
                            <div className="w-full max-w-sm min-w-[200px]">
                                <label className="block mb-2 text-sm text-slate-600">Task Title</label>
                                <input
                                type="text"
                                id="task-title"
                                name="title"
                                className="w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
                                placeholder="Task Title"
                                />
                            </div>

                            <div className="w-full max-w-sm min-w-[200px]">
                                <label className="block mb-2 text-sm text-slate-600">Task Description</label>
                                <textarea
                                id="task-description"
                                name="description"
                                className="w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
                                placeholder="Task Description"
                                ></textarea>
                            </div>

                            <div className="w-full max-w-sm min-w-[200px]">
                                <label className="block mb-2 text-sm text-slate-600">Due Date</label>
                                <input
                                type="date"
                                id="task-due-date"
                                name="dueDate"
                                className="w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
                                />
                            </div>

                            <div className="w-full max-w-sm min-w-[200px]">
                                <label className="block mb-2 text-sm text-slate-600">Priority</label>
                                <select
                                id="task-priority"
                                name="priority"
                                className="w-full bg-transparent placeholder:text-slate-400 text-slate-700 text-sm border border-slate-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-slate-400 hover:border-slate-300 shadow-sm focus:shadow"
                                >
                                <option value="low">Low</option>
                                <option value="medium">Medium</option>
                                <option value="high">High</option>
                                </select>
                            </div>

                                {/* Checkboxes for Critical Status */}
                            <div className="flex items-center gap-4 py-5">

                                {/* Critical Checkbox */}
                                <label className="flex items-center gap-2">
                                    <input
                                    type="checkbox"
                                    checked={task.is_critical}
                                    readOnly
                                    className="h-5 w-5 accent-red-500 cursor-pointer"
                                    />
                                    <span className="text-sm font-medium text-gray-700">Critical</span>
                                </label>
                            </div>

                            <div className="mt-4">
                                <button
                                type="submit"
                                class="w-full rounded-md bg-slate-800 py-2 px-4 text-white transition-all hover:bg-slate-700 focus:outline-none"
                                >
                                Submit
                                </button>
                            </div>
                        </form>             
                    </div>
                </div>
            </div>
            </div>
        </div>
        </div>
    );
}
