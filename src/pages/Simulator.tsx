import { useState } from "react";
import Sidebar from "../components/Sidebar";
import Tabs from "../components/TabManager";
import Editor from "../components/Editor";

import { open } from "@tauri-apps/plugin-dialog";
import { readTextFile, writeFile } from "@tauri-apps/plugin-fs";
import { invoke } from "@tauri-apps/api/core";

export default function Simulator() {
  const [tabs, setTabs] = useState([{ id: 1, name: "main.asm", content: "" }]);
  const [activeTab, setActiveTab] = useState(1);
  const [output, setOutput] = useState("");

  const currentTab = tabs.find((t) => t.id === activeTab);

  const updateTabContent = (value: string) => {
    setTabs((prev) =>
      prev.map((t) => (t.id === activeTab ? { ...t, content: value } : t))
    );
  };

  const addNewTab = () => {
    const id = Date.now();
    setTabs([...tabs, { id, name: `tab${tabs.length + 1}.asm`, content: "" }]);
    setActiveTab(id);
  };

  const closeTab = (id: number) => {
    if (tabs.length === 1) return;
    const newTabs = tabs.filter((t) => t.id !== id);
    setTabs(newTabs);
    if (activeTab === id) setActiveTab(newTabs[0].id);
  };

  return (
    <div className="flex h-screen bg-gray-900 text-white">
      <Sidebar />
      <div className="flex flex-col flex-1">
        <Tabs
          tabs={tabs.map((t) => ({ id: t.id, name: t.name }))}
          activeTab={activeTab}
          onSelect={setActiveTab}
          onClose={closeTab}
          onAdd={addNewTab}
        />
        <div className="flex-1 overflow-hidden">
          {currentTab && (
            <Editor value={currentTab.content} onChange={updateTabContent} />
          )}
        </div>
        <div className="bg-gray-800 p-2 text-sm h-24 overflow-y-auto">
          <h2 className="font-bold">Saída:</h2>
          <pre>{output}</pre>
          <div className="mt-2 space-x-2">
            <button
              //onClick={}
              className="bg-green-600 px-3 py-1 rounded"
            >
              Abrir
            </button>
            <button
              //onClick={salvarArquivo}
              className="bg-yellow-600 px-3 py-1 rounded"
            >
              Salvar
            </button>
            <button
              //onClick={executar}
              className="bg-red-600 px-3 py-1 rounded"
            >
              Executar
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
