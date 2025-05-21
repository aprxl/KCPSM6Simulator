export default function Sidebar() {
  return (
    <div className="w-64 bg-gray-800 p-4 space-y-4">
      <div>
        <h2 className="text-lg font-bold mb-2">Registradores</h2>
        <pre className="text-sm">R1: 0x00\nR2: 0x10\n...</pre>
      </div>
      <div>
        <h2 className="text-lg font-bold mb-2">Memória</h2>
        <pre className="text-sm">0x0000: 00\n0x0004: FF\n...</pre>
      </div>
    </div>
  );
}