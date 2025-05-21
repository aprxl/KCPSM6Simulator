import Registers from "./Registers";

export default function Sidebar() {
  return (
    <div className="w-64 bg-gray-800 p-4 space-y-4">
        <h3 className="text-lg font-bold mb-2">Registradores</h3>
        <Registers />
    </div>
  );
}