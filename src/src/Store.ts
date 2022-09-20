import { makeAutoObservable, action } from 'mobx';

import { listen } from '@tauri-apps/api/event';
import { State } from './generated/src-tauri/src/state';

// @ts-ignore
const { invoke } = window.__TAURI__.tauri;

class Store {
  state?: State;
  constructor() {
    makeAutoObservable(this);

    listen<Uint8Array>(
      'rs2js',
      action((e) => {
        console.log(e);
        this.state = State.deserialize(e.payload);
      })
    );
  }

  event(eventName: string, data = '') {
    invoke('handle_event', { eventName, data });
  }
}

export default Store;
