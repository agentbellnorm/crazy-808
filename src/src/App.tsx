import './App.css';
import { observer } from 'mobx-react-lite';
import { useStore } from './StoreContext';
import InstrumentSelect from './Component/InstrumentSelect';
import ChannelButtons from './Component/ChannelButtons';

function App() {
  const store = useStore();
  return (
    <div className="App">
      <header className="App-header">
        <InstrumentSelect />
        <select
          onChange={(e) => store.event('variation-changed', e.target.value)}
        >
          <option value="a">A</option>
          <option value="ab" disabled>
            AB
          </option>
          <option value="b">B</option>
        </select>
        <ChannelButtons />
        <button onMouseDown={() => store.event('start-stop')}>
          Start / Stop
        </button>
      </header>
    </div>
  );
}

export default observer(App);
