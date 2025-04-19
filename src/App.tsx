import React from "react";
import Dashboard from "./Dashboard";
import "./App.css";

export default function App() {
  return (
    <div className="app-wrapper">
      <h1 className="title">System Monitor</h1>
      <Dashboard />
    </div>
  );
}