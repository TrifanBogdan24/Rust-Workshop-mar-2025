import { open, save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

const editor = document.querySelector<HTMLTextAreaElement>("#editor")!;
const openBtn = document.querySelector<HTMLButtonElement>("#open")!;
const saveBtn = document.querySelector<HTMLButtonElement>("#save")!;

let currentPath: string | null = null;

// Open file
openBtn.addEventListener("click", async () => {
  const selected = await open({ multiple: false });
  if (selected && typeof selected === "string") {
    currentPath = selected;
    const contents = await invoke<string>("read_file", { path: currentPath });
    editor.value = contents;
  }
});

// Save file
saveBtn.addEventListener("click", async () => {
  if (!currentPath) {
    currentPath = await save({});
  }
  if (currentPath) {
    await invoke("write_file", {
      path: currentPath,
      contents: editor.value,
    });
    alert("File saved!");
  }
});
