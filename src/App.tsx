import { FileOpen, LastPage, Save } from "@mui/icons-material";
import "./App.css";
import KButton from "./components/Button";

function App() {
  return (
    <main className="bg-gray-50">
      <div className="grid grid-rows-[1fr_8fr_2fr] gap-y-1 h-screen">
        <div className="border-b-1 border-[#3f3f3f55] rounded-[5px] w-full flex flex-row p-2 bg-gray-300">
          <KButton icon={<FileOpen />} label="Open"></KButton>
          <KButton icon={<Save />} label="Save"></KButton>
          <KButton icon={<LastPage />} label="Run"></KButton>
        </div>

        <div className="grid grid-cols-[1fr_4fr_1fr] gap-1 w-screen">
          <div className="border-r-1 border-t-1 border-b-1 border-[#3f3f3f55] rounded-[5px] w-full bg-gray-200">
          </div>
          <div className="border-1 border-[#3f3f3f55] rounded-[5px] w-full bg-gray-200"></div>
          <div className="border-l-1 border-t-1 border-b-1 border-[#3f3f3f55] rounded-[5px] w-full bg-gray-200"></div>
        </div>

        <div className="border-t-1 border-[#3f3f3f55] rounded-[5px] w-full bg-gray-200">
          
        </div>
      </div>
    </main>
  );
}

export default App;
