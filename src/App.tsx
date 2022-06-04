import { fs, invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import React from "react";

export const App = () => {
  const saveContent = (content: string) => {
    open({ directory: true }).then((directory) => {
      if (directory === null) {
        return;
      }

      const path = Array.isArray(directory) ? directory[0] : directory;
      invoke<string>("save", { path, content }).then((msg) => console.log(msg));
    });
  };

  const handleClicked = () => {
    console.log("click");
    invoke<string>("generate")
      .then((res) => saveContent(res))
      .catch((e) => console.log("NG: " + e));
  };

  return (
    <div>
      <button onClick={() => handleClicked()}>生成</button>
    </div>
  );
};
