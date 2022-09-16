import React, { useState } from "react";
import logo from "./logo.svg";
import "./App.css";

// @ts-ignore
const { invoke } = window.__TAURI__.tauri;
// async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     greetMsgEl.textContent = await invoke("greet", {name: greetInputEl.value});
// }
//
function App() {
  const [greeting, setGreeting] = useState<string>("sd");
  const [greetingBack, setGreetingBack] = useState<string>("");
  return (
    <div className="App">
      <header className="App-header">
        <div>
          <input
            id="greet-input"
            value={greeting}
            onChange={(e) => setGreeting(e.target.value)}
            placeholder="Enter a name..."
          />
          <button
            type="button"
            onClick={() => {
              invoke("handle_event", {
                eventName: "set_drum",
                data: greeting,
              }).then(setGreetingBack);
            }}
          >
            Greet
          </button>
        </div>
        <div>{greetingBack}</div>
      </header>
    </div>
  );
}

export default App;
