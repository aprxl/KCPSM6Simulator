import { useState } from "react";

type RegisterProps = {
    label: string
};

function RegCheckbox({label}: RegisterProps) {
    const [value, setValue] = useState(0);

  const handleChange = (event: { target: { value: any; }; }) => {
    const newValue = event.target.value;

    // Allow the user to clear the input.
    if (newValue === '') {
        setValue(newValue);
        return;
    }

    // Validate the input to ensure it's a number between 0 and 255
    if (Number(newValue) >= 0 && Number(newValue) <= 255) {
      setValue(newValue);
    }
  };

  return (
    <div className="flex items-start">
      <label>
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
                        <RegCheckbox label={`S${i.toString(16).toUpperCase()}`} />
                    )
                })
            }
        </div>
    )
}