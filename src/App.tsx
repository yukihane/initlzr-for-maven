import { invoke } from "@tauri-apps/api";
import React from "react";

export const App = () => {
  const handleClicked = () => {
    console.log("click");
    invoke("generate")
      .then((res) => console.log("OK: " + res))
      .catch((e) => console.log("NG: " + e));
  };
  return (
    <div>
      <button onClick={() => handleClicked()}>生成</button>
    </div>
  );
};
