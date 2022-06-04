import { fs, invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";
import React from "react";
import { path } from "path";

export const App = () => {
  const saveContent = (contents: string) => {
    open({ directory: true }).then((directory) => {
      if (directory === null) {
        return;
      }

      const x = Array.isArray(directory) ? directory[0] : directory;
      path.join(x, "pom.xml");
      fs.writeFile({
        contents,
        path: x,
      });
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
