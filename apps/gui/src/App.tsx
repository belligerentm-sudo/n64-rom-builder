import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [msg, setMsg] = useState("Ready");
  async function onBuild() {
    try { setMsg(await invoke<string>("build_project")); }
    catch (e:any) { setMsg("Build failed: " + e.toString()); }
  }
  return (
    <main style={{ padding: 24 }}>
      <h1>N64 ROM Builder</h1>
      <button onClick={onBuild}>Build</button>
      <pre>{msg}</pre>
    </main>
  );
}
