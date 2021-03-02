import React from "react";

const Button = ({ size, text, onClick, className }) => {
  return (
    <button
      className={`${size} h-10 text-lg border rounded border-black ${className}`}
      onClick={onClick}
    >
      {text}
    </button>
  );
};

export default Button;
