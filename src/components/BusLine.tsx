type BusLineProps = {
  label: string;
  address: string;
  value: number; // 0–255
  editable?: boolean;
  onChange?: (newValue: number) => void;
};

export default function BusLine({ label, address, value, editable = false, onChange }: BusLineProps) {
  const bits = value.toString(2).padStart(8, "0").split("").map(Number);

  const toggleBit = (bitIndex: number) => {
    if (!editable || !onChange) return;

    const mask = 1 << (7 - bitIndex);
    const newValue = value ^ mask;
    onChange(newValue);
  };

  return (
    <div className="flex items-center space-x-2 text-sm text-gray-300">
      <span className="w-10 text-right font-mono">${address}</span>
      <span className="w-10">{label}</span>

      <div className="flex space-x-1">
        {bits.map((bit, index) => (
          <div
            key={index}
            onClick={() => toggleBit(index)}
            className={`w-4 h-4 border rounded ${
              editable
                ? bit
                  ? "bg-blue-400 border-blue-300 cursor-pointer"
                  : "bg-gray-800 border-gray-500 cursor-pointer hover:border-blue-200"
                : bit
                ? "bg-green-500 border-green-400"
                : "bg-gray-700 border-gray-600"
            }`}
            title={`Bit ${7 - index}`}
          ></div>
        ))}
      </div>

      <span className="w-10 text-right font-mono">${value.toString(16).padStart(2, "0").toUpperCase()}</span>
    </div>
  );
}
