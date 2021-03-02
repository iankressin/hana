import React from "react";

const CenterLayout = ({ children }) => (
  <div className="flex align-center justify-center flex-col w-full h-screen p-8">
    {children}
  </div>
);

export default CenterLayout;
