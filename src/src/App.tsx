import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import Store from './Store';
import { observer } from 'mobx-react-lite';

// @ts-ignore
const { invoke } = window.__TAURI__.tauri;
// async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//     greetMsgEl.textContent = await invoke("greet", {name: greetInputEl.value});
// }
//
function App({ store }: { store: Store }) {
  const [greeting, setGreeting] = useState<string>('sd');
  const [greetingBack, setGreetingBack] = useState<string>('');
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
              invoke('handle_event', {
                eventName: 'set_drum',
                data: greeting,
              }).then(setGreetingBack);
            }}
          >
            Greet
          </button>
        </div>
        <div style={{ display: 'flex' }}>
          {[...Array(16).keys()].map((i) => (
            <div
              key={`indicator-${i}`}
              style={{
                backgroundColor: i === Number(store.beat) ? 'green' : 'red',
                width: '50px',
                height: '50px',
                borderRadius: '50%',
              }}
            ></div>
          ))}
        </div>
        <div>{store.beat}</div>
        <select
          onChange={(e) =>
            invoke('handle_event', {
              eventName: 'variation-changed',
              data: e.target.value,
            })
          }
        >
          <option value="a">A</option>
          <option value="ab">AB</option>
          <option value="b">B</option>
        </select>
      </header>
    </div>
  );
}

export default observer(App);
