import { makeAutoObservable, action } from 'mobx';

import { listen } from '@tauri-apps/api/event'

class Store {
  beat?: string = undefined;
  constructor() {
    makeAutoObservable(this);

    listen('rs2js', action((e) => {
      this.beat = e.payload as string;
    }));
  }
}

export default Store;