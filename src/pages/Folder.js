import React, { useState, useEffect } from "react";
import { useHistory, useLocation } from "react-router-dom";
import { promisified } from "tauri/api/tauri";
import { Button as TextButton, List } from "antd";
import {
  FileTwoTone,
  ArrowLeftOutlined,
  SyncOutlined,
  ShareAltOutlined
} from "@ant-design/icons";

import BlockLayout from "../components/BlockLayout";

const Actions = ({ path, onSyncMetadata }) => {
  const syncMetadata = async () => {
    const response = await promisified({
      cmd: "sync",
      path
    });

    onSyncMetadata(response);
  };

  return (
    <div className="ml-auto">
      <TextButton type="text" onClick={() => console.log("Share")}>
        <ShareAltOutlined className="text-lg" />
        <span className="text-md">Share</span>
      </TextButton>

      <TextButton type="text" onClick={() => syncMetadata()}>
        <SyncOutlined className="text-lg" />
        <span className="text-md">Sync</span>
      </TextButton>
    </div>
  );
};

const Folder = () => {
  const history = useHistory();
  const location = useLocation();
  const [metadata, setMetadata] = useState([]);
  const { folder, path } = location.state.params;

  useEffect(() => getMetadata(), []);

  const getMetadata = async () => {
    try {
      const response = await promisified({
        cmd: "getMetadata",
        path
      });
      setMetadata(response);
    } catch (error) {}
  };

  return (
    <BlockLayout>
      <div className="flex w-full">
        <ArrowLeftOutlined
          className="mr-auto mb-4 text-2xl"
          onClick={() => history.push("/folders")}
        />
        <Actions
          onSyncMetadata={metadata => setMetadata(metadata)}
          path={path}
        />
      </div>
      <List
        className="w-full h-screen"
        header={
          <div>
            <h3 className="text-lg">{folder}</h3>
          </div>
        }
        bordered
        dataSource={metadata}
        renderItem={file => (
          <List.Item>
            <div className="flex align-center justify-center flex-row ">
              <FileTwoTone className="text-xl mr-4" />
              {file.name_extension}
            </div>
          </List.Item>
        )}
      />
    </BlockLayout>
  );
};

export default Folder;
