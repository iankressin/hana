import React, { useState, useEffect } from "react";
import { useHistory, useLocation } from "react-router-dom";
import { invoke, promisified } from "tauri/api/tauri";
import { Button as TextButton, List, Checkbox, message } from "antd";
import {
  FileTwoTone,
  ArrowLeftOutlined,
  SyncOutlined,
  ShareAltOutlined,
  DownloadOutlined,
  LoadingOutlined,
  CloseOutlined,
  SendOutlined
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

const Actions = ({
  path,
  onSyncMetadata,
  setServerRunning,
  serverRunning,
  setSelectingFiles,
  selectingFiles,
  selectedFiles,
  setSelectedFiles
}) => {
  const syncMetadata = async () => {
    try {
      const response = await promisified({
        cmd: "sync",
        path
      });

      message.success("Metadata synced");
      onSyncMetadata(response);
    } catch (error) {
      message.error(error);
    }
  };

  const runServer = async () => {
    try {
      setServerRunning(true);
      await promisified({
        cmd: "runServer",
        path
      });

      message.success("Metadata synced");
      setServerRunning(false);
      syncMetadata();
    } catch (error) {
      setServerRunning(false);
      message.error(error);
      console.log(error);
    }
  };

  const stopServer = async () => {
    try {
      setServerRunning(false);
      await promisified({
        cmd: "sendFiles",
        path,
        files: []
      });
    } catch (error) {
      console.log(error);
    }
  };

  const sendFiles = async () => {
    try {
      await promisified({
        cmd: "sendFiles",
        path,
        files: [...selectedFiles]
      });

      message.success("Files sent!");
      setSelectingFiles(false);
      setSelectedFiles(new Set([]));
    } catch (error) {
      message.error(error);
      console.log(error);
    }
  };

  return (
    <div className="ml-auto">
      <TextButton type="text" onClick={() => syncMetadata()}>
        <SyncOutlined className="text-lg" />
        <span className="text-md">Sync</span>
      </TextButton>

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

      {selectingFiles ? (
        <>
          <TextButton type="text" onClick={() => sendFiles()}>
            <SendOutlined className="text-lg" />
            <span className="text-md">Send</span>
          </TextButton>

          <TextButton type="text" onClick={() => setSelectingFiles(false)}>
            <CloseOutlined className="text-lg" />
            <span className="text-md">Cancel</span>
          </TextButton>
        </>
      ) : (
        <TextButton type="text" onClick={() => setSelectingFiles(true)}>
          <ShareAltOutlined className="text-lg" />
          <span className="text-md">Share</span>
        </TextButton>
      )}
    </div>
  );
};

const Folder = () => {
  const history = useHistory();
  const location = useLocation();
  const [metadata, setMetadata] = useState([]);
  const [selectingFiles, setSelectingFiles] = useState(false);
  const [selectedFiles, setSelectedFiles] = useState(new Set([]));
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

  const handleCheckboxChange = event => {
    const metaIndex = event.target.name;
    const fileMeta = metadata[metaIndex];

    if (selectedFiles.has(fileMeta)) {
      const set = new Set([...selectedFiles]);
      set.delete(fileMeta);
      setSelectedFiles(set);
    } else {
      const set = new Set([...selectedFiles]);
      set.add(fileMeta);
      setSelectedFiles(set);
    }
  };

  const openFile = path => {
    console.log("Path: ", path);
    invoke({
      cmd: "openFile",
      path
    });
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
          setSelectingFiles={setSelectingFiles}
          selectingFiles={selectingFiles}
          selectedFiles={selectedFiles}
          setSelectedFiles={setSelectedFiles}
        />
      </div>
      {serverRunning ? (
        <ServerRunning />
      ) : (
        <List
          className="w-auto h-screen"
          header={
            <div>
              <h3 className="text-lg">{folder}</h3>
            </div>
          }
          bordered
          dataSource={metadata}
          renderItem={(file, index) => (
            <List.Item>
              <div className="flex align-center justify-center flex-row ">
                {selectingFiles ? (
                  <Checkbox
                    className="mr-10"
                    name={index}
                    onChange={handleCheckboxChange}
                  >
                    <label>{file.name_extension}</label>
                  </Checkbox>
                ) : (
                  <TextButton
                    type="text"
                    onClick={() => openFile(`${path}/${file.name_extension}`)}
                  >
                    <FileTwoTone className="text-xl mr-4" />
                    {file.name_extension}
                  </TextButton>
                )}
              </div>
            </List.Item>
          )}
        />
      )}
    </BlockLayout>
  );
};

export default Folder;
