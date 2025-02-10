import Task from "../../interfaces/Task";




export default function TaskItem({
  task,
  modalHandler
}: {
  task: Task
  modalHandler: (id: number) => void;
}) {

    const getPriorityColor = (priority: string) => {
        switch (priority) {
          case "immediate":
            return "font-semibold text-red-500 text-xs ";
          case "high":
            return "font-semibold text-yellow-500 text-xs";
          case "medium":
            return "font-semibold text-orange-500 text-xs";
          case "low":
            return "font-semibold text-blue-500 text-xs";
          default:
            return "font-semibold text-gray-500 text-xs"; // Fallback color
        }
    };

    const ModalOpenClickHandler = () => {
        modalHandler(task.id);
    }
    return(
    <>   
    <div onClick={ModalOpenClickHandler} id="task" className="hover:cursor-pointer flex justify-between items-center border-b border-slate-200 py-3 px-2 border-l-4  border-l-transparent bg-gradient-to-r from-transparent to-transparent hover:from-slate-100 transition ease-linear duration-150">
        <div className="inline-flex items-center space-x-2">
            <div>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5" stroke="currentColor" className="w-6 h-6 text-slate-500 hover:text-indigo-600 hover:cursor-pointer">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                                        
            </div>
            <div>{task.title}</div>
        </div>


        <div className="flex items-center gap-4">
            <p className={getPriorityColor(task.priority)}>{task.priority}</p>
        </div>
    </div>
    </>

    );
}