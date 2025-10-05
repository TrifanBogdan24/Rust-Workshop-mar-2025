import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

const editor = document.querySelector<HTMLTextAreaElement>("#editor")!;
const fileTree = document.querySelector<HTMLUListElement>("#file-tree")!;
const openFolderBtn = document.querySelector<HTMLButtonElement>("#open-folder")!;
const saveBtn = document.querySelector<HTMLButtonElement>("#save")!;

let currentFile: string | null = null;

async function renderFolder(path: string, parent: HTMLElement) {
  try {
    const entries = await invoke<[string, boolean][]>("get_child_paths", { path });
    const ul = document.createElement("ul");
    for (const [childPath, isDir] of entries) {
      const li = document.createElement("li");
      li.textContent = childPath.split("/").pop()!;
      li.dataset.path = childPath;
      li.classList.add(isDir ? "folder" : "file");

      if (isDir) {
        li.addEventListener("click", async (e) => {
          e.stopPropagation();
          if (li.classList.contains("collapsed")) {
            li.classList.remove("collapsed");
            await renderFolder(childPath, li);
          } else {
            li.classList.add("collapsed");
            const childUl = li.querySelector("ul");
            if (childUl) li.removeChild(childUl);
          }
        });
      } else {
        li.addEventListener("click", async (e) => {
          e.stopPropagation();
          currentFile = childPath;
          const content = await invoke<string>("read_file", { path: childPath });
          editor.value = content;
        });
      }

      ul.appendChild(li);
    }
    parent.appendChild(ul);
  } catch (err) {
    console.error("Failed to read directory:", err);
  }
}

openFolderBtn.addEventListener("click", async () => {
  const selected = await open({ directory: true, multiple: false });
  if (selected && typeof selected === "string") {
    fileTree.innerHTML = "";
    await renderFolder(selected, fileTree);
  }
});

saveBtn.addEventListener("click", async () => {
  if (currentFile) {
    await invoke("write_file", { path: currentFile, contents: editor.value });
    alert("File saved!");
  } else {
    alert("No file selected.");
  }
});
