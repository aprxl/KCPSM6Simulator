import { useState } from "react";
import BusLine from "./BusLine";

export default function IOVisualizer() {
  const [opA, setOpA] = useState(0b00000000);

  return (
    <div className="space-y-1 bg-gray-800 p-1 rounded shadow-md border border-gray-600">
      <BusLine label="opA" address="01" value={opA} editable onChange={setOpA} />
      <BusLine label="opB" address="02" value={0b10101010} />
      <BusLine label="opC" address="03" value={0b01010101} />
      <BusLine label="MSRes" address="04" value={0b11111111} />
      <BusLine label="LSRes" address="05" value={0b00000001} />
    </div>
  );
}
