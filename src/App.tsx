import "./App.css";

function App() {
  return (
    <main>
      <div className="grid grid-rows-[1fr_8fr_2fr] gap-y-1 h-screen">
        <div className="border-1 border-[#3f3f3f] rounded-[5px] w-full">
          Top
        </div>

        <div className="grid grid-cols-[1fr_4fr_1fr] gap-1 w-screen">
          <div className="border-1 border-[#3f3f3f] rounded-[5px] w-full">Left</div>
          <div className="border-1 border-[#3f3f3f] rounded-[5px] w-full">Middle</div>
          <div className="border-1 border-[#3f3f3f] rounded-[5px] w-full">Right</div>
        </div>

        <div className="border-1 border-[#3f3f3f] rounded-[5px] w-full">
          Bottom
        </div>
      </div>
    </main>
  );
}

export default App;
