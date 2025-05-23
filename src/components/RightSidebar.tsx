
import IOVisualizer from "./IOVisualizer";

export default function RightSidebar() {
  return (
    <div className="w-128 bg-gray-800 p-1 space-y-1 border-l border-gray-700">
      <h3 className="text-lg font-bold mb-2 text-gray-200">Entradas/Saídas</h3>
      <IOVisualizer />
    </div>
  );
}
