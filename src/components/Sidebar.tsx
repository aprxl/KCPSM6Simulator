import Registers from "./Registers";
import FlagsPanel from "./FlagsPanel";

export default function Sidebar() {
  return (
    <div className="w-64 bg-gray-800 p-4 space-y-6">
      <div>
        <h3 className="text-lg font-bold mb-2">Registradores</h3>
        <Registers />
      </div>
      <div>
        <h3 className="text-lg font-bold mb-2">Flags</h3>
        <FlagsPanel />
      </div>
    </div>
  );
}
