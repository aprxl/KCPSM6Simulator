import { readTextFile, writeFile } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";

export async function abrirArquivo() {
  const path = await open({ filters: [{ name: "Assembly", extensions: ["asm"] }] });
  if (!path || Array.isArray(path)) return null;
  return await readTextFile(path);
}

export async function salvarArquivo(nome: string, conteudo: string) {
  await writeFile(nome, new TextEncoder().encode(conteudo));
}
