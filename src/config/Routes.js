import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";

import New from "../pages/New";
import FoldersList from "../pages/FoldersList";
import Folder from "../pages/Folder";

const Routes = () => {
  const hasFolderSet = localStorage.getItem("hasFolder");
  console.log(hasFolderSet);

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

        <Route path="/">{hasFolderSet ? <FoldersList /> : <New />}</Route>
      </Switch>
    </Router>
  );
};

export default Routes;
