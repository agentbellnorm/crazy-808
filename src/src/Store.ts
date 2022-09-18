import { makeAutoObservable, action } from 'mobx';

import { listen } from '@tauri-apps/api/event';

class Store {
  beat?: string = undefined;
  constructor() {
    makeAutoObservable(this);

    listen<{ bar: number }>(
      'rs2js',
      action((e) => {
        //@ts-ignore
        const state = JSON.parse(e.payload);
        this.beat = state.bar.toString();
      })
    );
  }
}

export default Store;
