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

export default function Modal({ 
    isOpen, 
    closeModal, 
    taskId,
    deleteTask
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
    
    const deleteTaskHandler = () => {
        deleteTask(taskId);
        closeModal();
    }
    
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
                    <div className="flex flex-col gap-4">
                        {/* Title */}
                        <h1 className="text-2xl font-bold text-gray-900">{task.title}</h1>

                        {/* Description */}
                        <p className="text-gray-600 text-sm sm:text-base">{task.description}</p>

                        {/* Priority */}
                        <p className="text-sm sm:text-base font-semibold text-red-500">
                        Priority: {task.priority}
                        </p>

                        {/* Due Date */}
                        <p className="text-gray-500 text-sm">
                        <span className="font-medium text-gray-700">Due Date:</span> {task.due_date}
                        </p>

                        {/* Checkboxes for Completion & Critical Status */}
                        <div className="flex items-center gap-4">
                            {/* Completed Checkbox */}
                            <label className="flex items-center gap-2">
                                <input
                                type="checkbox"
                                checked={task.is_completed}
                                readOnly
                                className="h-5 w-5 accent-green-500 cursor-pointer"
                                />
                                <span className="text-sm font-medium text-gray-700">Completed</span>
                            </label>

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
                    </div>
                </div>


                {/* Buttons */}
                <div className="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
                    <button 
                        type="button" 
                        className="hover:cursor-pointer inline-flex w-full justify-center rounded-md bg-green-600 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-black sm:ml-3 sm:w-auto"
                        onClick={closeModal}
                    >
                        Completed
                    </button>


                    <button 
                        type="button" 
                        className="hover:cursor-pointer mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 ring-1 shadow-xs ring-gray-300 ring-inset hover:bg-gray-50 sm:mt-0 sm:w-auto"
                        onClick={deleteTaskHandler}
                    >
                        Delete
                    </button>
                </div>
            </div>
            </div>
        </div>
        </div>
    );
}
