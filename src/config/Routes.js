import React, { useState, useEffect } from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { message } from "antd";
import { promisified } from "tauri/api/tauri";

import New from "../pages/New";
import FoldersList from "../pages/FoldersList";
import Folder from "../pages/Folder";

const Routes = () => {
  const [hasDirs, setHasDirs] = useState(false);

  useEffect(() => checkDirs(), []);

  const checkDirs = async () => {
    try {
      const hasDirs = await promisified({
        cmd: "hasDirs"
      });
      console.log("Has dirs: ", hasDirs);
      setHasDirs(hasDirs);
    } catch (error) {
      message.error(error);
      console.log(error);
    }
  };

  return (
    <Router>
      <Switch>
        <Route path="/new">
          <New />
        </Route>

        <Route path="/folder">
          <Folder />
        </Route>

        <Route path="/folders">
          <FoldersList />
        </Route>

        <Route path="/">{hasDirs ? <FoldersList /> : <New />}</Route>
      </Switch>
    </Router>
  );
};

export default Routes;
