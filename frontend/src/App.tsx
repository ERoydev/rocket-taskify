import "../src/input.css";
import Footer from "./components/Footer";
import Header from "./components/Header";
import Home from "./components/Home";
import TaskList from "./components/Task/TaskList";

function App() {

  return (
    <div className="bg-white dark:bg-gray-900">
      <Header />
      <Home />
      <TaskList />
      <Footer />
    </div>
  )
}

export default App
