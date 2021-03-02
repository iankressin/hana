import React, { useState, useEffect } from "react";
import { useHistory, useLocation } from "react-router-dom";
import { promisified } from "tauri/api/tauri";
import { Button as TextButton, List } from "antd";
import {
  FileTwoTone,
  ArrowLeftOutlined,
  SyncOutlined,
  ShareAltOutlined,
  DownloadOutlined,
  LoadingOutlined,
  CloseOutlined
} from "@ant-design/icons";

import BlockLayout from "../components/BlockLayout";
import CenterLayout from "../components/CenterLayout";

const ServerRunning = () => {
  return (
    <CenterLayout>
      <div className="border w-full h-screen flex flex-col align-center justify-center">
        <LoadingOutlined className="text-3xl mb-10" />
        <h1 className="text-xl text-center">Waiting for files ...</h1>
      </div>
    </CenterLayout>
  );
};

const Actions = ({ path, onSyncMetadata, setServerRunning, serverRunning }) => {
  const syncMetadata = async () => {
    try {
      const response = await promisified({
        cmd: "sync",
        path
      });

      onSyncMetadata(response);
    } catch (error) {
      console.log("Error: ", error);
    }
  };

  const runServer = async () => {
    try {
      setServerRunning(true);
      await promisified({
        cmd: "runServer",
        path
      });

      setServerRunning(false);
    } catch (error) {
      setServerRunning(false);
      console.log(error);
    }
  };

  const stopServer = async () => {};

  return (
    <div className="ml-auto">
      {serverRunning ? (
        <TextButton type="text" onClick={() => stopServer()}>
          <CloseOutlined className="text-lg" />
          <span className="text-md">Stop Receiving</span>
        </TextButton>
      ) : (
        <TextButton type="text" onClick={() => runServer()}>
          <DownloadOutlined className="text-lg" />
          <span className="text-md">Receive</span>
        </TextButton>
      )}

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
  const [serverRunning, setServerRunning] = useState(false);
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
      <div className="flex w-full mb-4">
        <ArrowLeftOutlined
          className="mr-auto text-2xl"
          onClick={() => history.push("/folders")}
        />
        <Actions
          onSyncMetadata={metadata => setMetadata(metadata)}
          setServerRunning={setServerRunning}
          serverRunning={serverRunning}
          path={path}
        />
      </div>
      {serverRunning ? (
        <ServerRunning />
      ) : (
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
      )}
    </BlockLayout>
  );
};

export default Folder;
