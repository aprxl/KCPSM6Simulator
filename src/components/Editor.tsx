import CodeMirror from "@uiw/react-codemirror";
import { cpp } from "@codemirror/lang-cpp";
import { oneDark } from "@codemirror/theme-one-dark";

export default function Editor({ value, onChange }: { value: string; onChange: (val: string) => void }) {
  return (
    <CodeMirror
      value={value}
      height="100%"
      theme={oneDark}
      extensions={[cpp()]}
      onChange={onChange}
    />
  );
}