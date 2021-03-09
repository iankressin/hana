import React, { useState } from "react";
import { useHistory } from "react-router-dom";

import NewFolderButton from "../components/NewFolderButton";
import CenterLayout from "../components/CenterLayout";

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
  const history = useHistory();

  const handleInitFolder = () => {
    setLoading(loadingState);

    localStorage.setItem("hasFolder", true);
    history.push("/folders");

    setLoading(defaultState);
  };

  return (
    <CenterLayout>
      <div className="text-center m-auto">
        <h1 className="text-8xl">🐶</h1>
        <h1 className="text-6xl"> Welcome to Hana!</h1>
        <h3 className="text-2xl mb-8">
          First, let's pick a location to initialize as a Hana folder
        </h3>
        <NewFolderButton
          size="w-80"
          onResponse={() => handleInitFolder()}
          text={loading.btn}
        />
      </div>
    </CenterLayout>
  );
};

export default New;
