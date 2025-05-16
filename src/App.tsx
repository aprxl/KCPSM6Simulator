import "./App.css";
import Button from "./components/Button";

function App() {
  return (
    <main>
      <div className="grid grid-rows-[1fr_8fr_2fr] gap-y-1 h-screen">
        <div className="border-b-1 border-[#3f3f3f55] rounded-[5px] w-full flex flex-row">
          <Button icon="./src/assets/file.svg" name="Open file"></Button>
          <Button icon="./src/assets/file.svg" name="Save file"></Button>
        </div>

        <div className="grid grid-cols-[1fr_4fr_1fr] gap-1 w-screen">
          <div className="border-r-1 border-t-1 border-b-1 border-[#3f3f3f55] rounded-[5px] w-full"></div>
          <div className="border-1 border-[#3f3f3f55] rounded-[5px] w-full"></div>
          <div className="border-l-1 border-t-1 border-b-1 border-[#3f3f3f55] rounded-[5px] w-full"></div>
        </div>

        <div className="border-t-1 border-[#3f3f3f55] rounded-[5px] w-full">
          
        </div>
      </div>
    </main>
  );
}

export default App;
