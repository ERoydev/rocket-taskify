import { useEffect, useState } from "react";
import CrossedTaskItem from "./CrossedTaskItem";
import TaskItem from "./TaskItem";
import { deleteTask, getTasks } from "../../api/TaskApi";
import Modal from "./Modal";
import FormModal from "./FormModal";


export default function TaskList() {
    const [tasks, setTasks] = useState<any[]>([]);
    const [error, setError] = useState<string | null>(null);
    const [modalIsOpen, setModalIsOpen] = useState(false);
    const [formModalIsOpen, setFormModalIsOpen] = useState(false);
    const [currTaskId, setCurrTaskId] = useState(0);

    useEffect(() => {
        // Fetch tasks when the component mounts
        const fetchTasks = async () => {
          try {
            const fetchedTasks = await getTasks();  // Get tasks function
            setTasks(fetchedTasks);  // Set tasks in state
          } catch (err) {
            setError('Failed to fetch tasks');  // Handle error
          }
        };
    
        fetchTasks();  // Call the function to fetch tasks
      }, [])


    const formModalHandler = () => {
        if (formModalIsOpen) {
            setFormModalIsOpen(false)
        } else if (!formModalIsOpen) {
            setFormModalIsOpen(true)
        }
    }
    const modalHandler = (id: number) => {
        if (modalIsOpen) {
            setModalIsOpen(false)
        } else if (!modalIsOpen) {
            setCurrTaskId(id);
            setModalIsOpen(true)
        }
    }

    const deleteTaskHandler = async (id: number) => {
        await deleteTask(id);
        setTasks((prevTasks) => {
            const updatedTasks = prevTasks.filter(task => task.id !== id);
            console.log("After delte: ", updatedTasks)
            return updatedTasks;
        });
    }



    return(
        <>
        {error && <p>{error}</p>}
        {formModalIsOpen && 
            <FormModal
                isOpen={formModalIsOpen}    
                closeModal={formModalHandler}
                />
        }
        
        {modalIsOpen && 
            <Modal
                isOpen={modalIsOpen} 
                closeModal={modalHandler} 
                taskId={currTaskId}
                deleteTask={deleteTaskHandler}
            />
        }
        <div className="antialiased bg-white text-slate-700 p-20 pb-30" id="tasklist">
            <div className="max-w-[1400px] mx-auto my-10 bg-white p-8 rounded-xl shadow shadow-slate-300">
                <div className="flex flex-row justify-between items-center">
                    <div>
                        <h1 className="text-3xl font-medium">Tasks list</h1>
                    </div>
                    
                    {/* SIDE BUTTONS */}
                    <div className="inline-flex space-x-2 items-center">
                        <button onClick={() => setFormModalIsOpen(true)} className="hover:cursor-pointer p-2 border border-slate-200 rounded-md inline-flex space-x-1 items-center text-indigo-200 hover:text-white bg-indigo-600 hover:bg-indigo-500">
                        <svg xmlns="http://www.w3.org/2000/svg" className="w-4 h-4 fill-gray-200" viewBox="0 0 448 512"><path d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 144L48 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l144 0 0 144c0 17.7 14.3 32 32 32s32-14.3 32-32l0-144 144 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-144 0 0-144z"/></svg>
                            <span className="text-sm font-medium hidden md:block">Create New Task</span>                     
                        </button>

                        <button className="hover:cursor-pointer p-2 border border-slate-200 rounded-md inline-flex space-x-1 items-center hover:bg-slate-200">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                            </svg> 
                            <span className="text-sm hidden md:block">Sort By Priority</span>                    
                        </button>

                        <button className="hover:cursor-pointer p-2 border border-slate-200 rounded-md inline-flex space-x-1 items-center hover:bg-slate-200">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                            </svg> 
                            <span className="text-sm hidden md:block">Completed</span>                    
                        </button>

                        <button className="hover:cursor-pointer p-2 border border-slate-200 rounded-md inline-flex space-x-1 items-center hover:bg-slate-200">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" className="w-4 h-4">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
                            </svg> 
                            <span className="text-sm hidden md:block">Not Completed</span>                    
                        </button>
                    </div>
                </div>
                <p className="text-slate-500">Hello, here are your latest tasks</p>



                <div id="tasks" className="my-5">
                    <CrossedTaskItem />
                    {tasks.map(task => (
                        <TaskItem key={task.id} task={task} modalHandler={modalHandler}/>
                    )
                    
                )}
       
                </div>


                <p className="text-xs text-slate-500 text-center">Last updated 12 minutes ago</p>
            </div>
        </div>
    </>
    );
}