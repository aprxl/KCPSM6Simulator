export default function Tabs({
  tabs,
  activeTab,
  onSelect,
  onClose,
  onAdd,
}: {
  tabs: { id: number; name: string }[];
  activeTab: number;
  onSelect: (id: number) => void;
  onClose: (id: number) => void;
  onAdd: () => void;
}) {
  return (
    <div className="flex items-center bg-gray-700 p-2">
      {tabs.map((tab) => (
        <div
          key={tab.id}
          className={`px-4 py-1 rounded-t-md cursor-pointer ${
            tab.id === activeTab ? "bg-gray-900 text-white" : "bg-gray-600 text-gray-300"
          }`}
          onClick={() => onSelect(tab.id)}
        >
          {tab.name}
          <button
            onClick={(e) => {
              e.stopPropagation();
              onClose(tab.id);
            }}
            className="ml-2 text-sm"
          >
            ×
          </button>
        </div>
      ))}
      <button onClick={onAdd} className="ml-4 bg-blue-600 px-2 rounded text-white">
        +
      </button>
    </div>
  );
}