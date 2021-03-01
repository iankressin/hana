import React, { useState } from "react";
import { promisified } from "tauri/api/tauri";
import { open, close } from "tauri/api/dialog";
import { Button, Input } from "antd";

const New = () => {
  const defaultState = {
    state: false,
    btn: "Select folder"
  };
  const loadingState = {
    state: true,
    btn: "Configuring folder ..."
  };
  const [loading, setLoading] = useState(defaultState);

  const handleInitFolder = async () => {
    setLoading(loadingState);
    try {
      const folder = await getFolderPath();
      const response = await promisified({
        cmd: "init",
        folder
      });
      console.log("Response from rust", response);
    } catch (error) {
      console.log("----->>> Something went wrong!!!!");
      console.log(error);
    }

    setLoading(defaultState);
  };

  const getFolderPath = async () => {
    const path = await open({ directory: true });
    return path;
  };

  return (
    <div>
      <h1>Hana</h1>
      <p>First, pick a valid folder path to initialize hazna</p>
      <Button loading={loading.state} onClick={() => handleInitFolder()}>
        {loading.btn}
      </Button>
    </div>
  );
};

export default New;
