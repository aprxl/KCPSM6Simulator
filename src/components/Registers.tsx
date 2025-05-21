import { useState } from "react";

type RegisterProps = {
    label: string
};

function Register({label}: RegisterProps) {
    const [value, setValue] = useState(0);

  const handleChange = (event: { target: { value: string; }; }) => {
    const newValue = event.target.value;

    // Validate the input to ensure it's a number between 0 and 255
    if (newValue === '' || (Number(newValue) >= 0 && Number(newValue) <= 255)) {
      setValue(Number(newValue));
    }
  };

  return (
    <div className="flex items-start">
      <label className="mr-5">
        {label}
        <input
          type="number"
          value={value}
          onChange={handleChange}
          min="0"
          max="255"
          className="rounded ml-2 px-1 py-0.5 bg-gray-700 border border-gray-500 hover:border-blue-200 border-1"
        />
      </label>
    </div>
  );
}

export default function Registers() {
    const arr = Array.from({length: 16}, (_, index) => index);

    return (
        <div className="grid grid-cols-2 grid-rows-8 gap-y-2">
            {
                arr.map((i) => {
                    return (
                        <Register label={`S${i.toString(16).toUpperCase()}`} />
                    )
                })
            }
        </div>
    )
}