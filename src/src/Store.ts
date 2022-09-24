import { makeAutoObservable, action } from 'mobx';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';
import { Bar, State, Variation } from './generated/src-tauri/src/state';

// type Variation = 'a' | 'ab' | 'b';
const defaultBar: Bar = Bar.fromObject({
  bar: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
});
const defaultVariation: Variation = Variation.fromObject({
  instrument: Array(17).fill(defaultBar),
});

const initialState: State = State.fromObject({
  playing: false,
  current_variation: 'a',
  variation_a: defaultVariation,
  variation_b: defaultVariation,
  bar: 0,
  selected_instrument: 1,
});

class Store {
  state: State = initialState;
  constructor() {
    makeAutoObservable(this);

    listen<Uint8Array>(
      'rs2js',
      action((e) => {
        try {
          console.log('event');

          this.state = State.deserialize(e.payload);
        } catch (err) {
          console.warn(err);
        }
      })
    );

    setTimeout(() => this.event('get-state'), 0);
  }

  event(eventName: string, data = '') {
    invoke('handle_event', { eventName, data });
  }

  get selectedInstrumentBars(): number[] {
    return this.currentVariation[this.state.selected_instrument].bar;
  }

  get currentVariation(): Bar[] {
    switch (this.state.current_variation) {
      case 'a':
        return this.state.variation_a.instrument;
      case 'ab':
        return this.bar < 16
          ? this.state.variation_a.instrument
          : this.state.variation_b.instrument;
      case 'b':
        return this.state.variation_b.instrument;
      default:
        throw new Error(`What variation: ${this.state.current_variation}`);
    }
  }

  get bar(): number {
    return this.state.bar % (this.state.current_variation === 'ab' ? 32 : 16);
  }
}

export default Store;
