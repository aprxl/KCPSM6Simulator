import { useState } from "react";

type RegisterProps = {
    label: string,
    disabled: boolean
};

function RegCheckbox({label, disabled}: RegisterProps) {
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
    <div className="flex">
      <label className={`text-sm text-center ${disabled ? "text-gray-500" : "text-gray-300"}`}>
        <p className="translate-x-1">{label}</p>
        <input
          value={value}
          onChange={handleChange}
          disabled={disabled}
          min="0"
          max="255"
          className="w-12 text-gray-400 hover:text-gray-200 disabled:text-gray-600 transition-colors rounded ml-2 px-1 py-0.5 bg-gray-700 border border-gray-500 hover:border-blue-200 disabled:border-gray-600 border-1"
        />
      </label>
    </div>
  );
}

export default function Registers() {
    const arr = Array.from({length: 16}, (_, index) => index);

    return (
        <div className="grid w-[90%] grid-cols-2 grid-rows-8 m-2 gap-y-0.5 items-start justify-items-start">
            {
                arr.map((i) => {
                    const hexIndex = i.toString(16).toUpperCase();

                    return (
                        <RegCheckbox label={`S${hexIndex}`} disabled={false} />
                    )
                })
            }
        </div>
    )
}