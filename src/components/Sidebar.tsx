import Registers from "./Registers";
import FlagsPanel from "./FlagsPanel";

export default function Sidebar() {
  return (
    <div className="w-40 bg-gray-800 p-2">
      <div>
        <h3 className="text-lg font-bold mb-2">Registradores</h3>
        <div className="w-full h-0.5 bg-gray-600"></div>
        <Registers />
      </div>
      <div>
        <h3 className="text-lg font-bold mb-2">Flags</h3>
        <div className="w-full h-0.5 mb-2 bg-gray-600"></div>
        <FlagsPanel />
      </div>
    </div>
  );
}
