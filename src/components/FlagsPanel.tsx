import { useState } from "react";

export default function FlagsPanel() {
  const [flags, setFlags] = useState({
    zero: false,
    carry: false,
    enable: false,
    steady: false,
    edge: false,
    timer: false,
    interval: 50,
  });

  const toggle = (key: keyof typeof flags) => {
    setFlags((prev) => ({
      ...prev,
      [key]: typeof prev[key] === "boolean" ? !prev[key] : prev[key],
    }));
  };

  const updateInterval = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFlags((prev) => ({ ...prev, interval: Number(e.target.value) }));
  };

  return (
    <div className="bg-gray-800 text-white p-2 rounded shadow w-40 space-y-3 text-sm border border-gray-700">
      <div>
        <h3 className="font-semibold mb-1 text-gray-300">Status</h3>
        {["zero", "carry", "enable"].map((flag) => (
          <label
            key={flag}
            className="flex items-center justify-between text-gray-300 text-sm"
          >
            {flag.charAt(0).toUpperCase() + flag.slice(1)}
            <input
              type="checkbox"
              checked={flags[flag as keyof typeof flags] as boolean}
              onChange={() => toggle(flag as keyof typeof flags)}
              className="appearance-none w-5 h-5 bg-gray-700 border border-gray-500 rounded-sm checked:bg-green-500 checked:border-green-300 transition-colors"
            />
          </label>
        ))}
      </div>

      <div>
        <h3 className="font-semibold mb-1 text-gray-300">Interrupt</h3>
        {["steady", "edge", "timer"].map((flag) => (
          <label
            key={flag}
            className="flex items-center justify-between text-gray-300 text-sm"
          >
            {flag.charAt(0).toUpperCase() + flag.slice(1)}
            <input
              type="checkbox"
              checked={flags[flag as keyof typeof flags] as boolean}
              onChange={() => toggle(flag as keyof typeof flags)}
              className="appearance-none w-5 h-5 bg-gray-700 border border-gray-500 rounded-sm checked:bg-green-500 checked:border-green-300 transition-colors"
            />
          </label>
        ))}

        <input
          type="number"
          value={flags.interval}
          onChange={updateInterval}
          min="0"
          max="1000"
          className="mt-2 w-full text-gray-400 hover:text-gray-200 transition-colors rounded px-1 py-0.5 bg-gray-700 border border-gray-500 hover:border-blue-200 text-sm"
        />
      </div>
    </div>
  );
}
