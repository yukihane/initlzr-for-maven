import { fs, invoke } from "@tauri-apps/api";
import { save } from "@tauri-apps/api/dialog";
import React from "react";

export const App = () => {
  const saveContent = (contents: string) => {
    save().then((path) => {
      if (path === null) {
        return;
      }
      fs.writeFile({
        contents,
        path,
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
