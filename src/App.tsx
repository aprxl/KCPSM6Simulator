import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  let [text, setText] = useState("");

  async function read() {
    setText(await invoke<string>('read_file'));
  }

  return (
    <main className="container">
      <p>{text}</p>
      <button onClick={() => read()}> Read file </button>
    </main>
  );
}

export default App;
