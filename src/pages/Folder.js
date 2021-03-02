import React, { useState, useEffect } from "react";
import { useHistory, useLocation } from "react-router-dom";
import { promisified } from "tauri/api/tauri";

const Folder = () => {
  const history = useHistory();
  const location = useLocation();
  const [metadata, setMetadata] = useState([]);
  const { path } = location.state;

  useEffect(() => getMetadata(), []);

  const getMetadata = async () => {
    try {
      const response = await promisified({
        cmd: "getMetadata",
        path
      });

      console.log("THE METADATA =>>>>>>>>>>>>>>>>> ", response);
      setMetadata(response);
    } catch (error) {}
  };

  return (
    <div>
      <button onClick={() => history.push("/folders")}>Back</button>
      <h1>This is the folder page {path}</h1>
    </div>
  );
};

export default Folder;
