import React from "react";
import { promisified } from "tauri/api/tauri";
import { open } from "tauri/api/dialog";

import Button from "./Button";

const NewFolderButton = ({ text, size, onResponse, className }) => {
  const handleInitFolder = async () => {
    try {
      const folder = await getFolderPath();
      const response = await promisified({
        cmd: "init",
        folder
      });

      onResponse(response);
    } catch (error) {
      console.log(error);
    }
  };

  const getFolderPath = async () => {
    const path = await open({ directory: true });
    return path;
  };

  return (
    <Button
      className={className}
      size={size}
      onClick={() => handleInitFolder()}
      text={text}
    />
  );
};

export default NewFolderButton;
