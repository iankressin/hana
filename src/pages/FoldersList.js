import React, { useEffect, useState } from "react";
import { useHistory } from "react-router-dom";
import { promisified } from "tauri/api/tauri";
import { Button as TextButton, List } from "antd";
import { FolderTwoTone } from "@ant-design/icons";

import NewFolderButton from "../components/NewFolderButton";
import BlockLayout from "../components/BlockLayout";

const FoldersList = () => {
  const [folders, setFolders] = useState([]);
  const history = useHistory();

  useEffect(() => {
    getFolders();
  }, []);

  const getFolders = async () => {
    try {
      const folders = await promisified({
        cmd: "getFolders"
      });

      setFolders(folders);
    } catch (error) {
      console.log(error);
    }
  };

  const updateFolders = folder => setFolders([...folders, folder]);

  return (
    <BlockLayout>
      <NewFolderButton
        className="ml-auto mb-4"
        size="w-32"
        text="New"
        onResponse={updateFolders}
      />
      <List
        className="w-full h-screen"
        header={
          <div>
            <h3 className="text-lg">Folders</h3>
          </div>
        }
        bordered
        dataSource={folders}
        renderItem={([folder, path]) => (
          <List.Item>
            <div className="flex align-center justify-center flex-row ">
              <TextButton
                type="text"
                className="text-black-500"
                onClick={() => history.push("folder", { path })}
              >
                <FolderTwoTone className="text-xl mr-4" /> {folder}
              </TextButton>
            </div>
          </List.Item>
        )}
      />
    </BlockLayout>
  );
};

export default FoldersList;
