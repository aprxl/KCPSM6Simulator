import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type FileContent = {
  content: string,
  size: number
};

function App() {
  let [content, setContent] = useState<FileContent>();

  async function read() {
    invoke<FileContent>('read_file').then(fc => {
      setContent(fc)
    }).catch(() => {
      setContent({
        content: "Unable to read file!",
        size: -1
      })
    })
  }

  return (
    <main className="container">
      <p>{content?.content}</p>
      <p> Length: {content?.size}</p>
      <button onClick={() => read()}> Read file </button>
    </main>
  );
}

export default App;
