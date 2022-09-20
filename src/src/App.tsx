import './App.css';
import { observer } from 'mobx-react-lite';
import { useStore } from './StoreContext';
import InstrumentSelect from './Component/InstrumentSelect';

function App() {
  const store = useStore();
  const bar = store.state?.bar ?? 0;
  return (
    <div className="App">
      <header className="App-header">
        <InstrumentSelect />
        <div style={{ display: 'flex' }}>
          {[...Array(16).keys()].map((i) => (
            <div
              key={`indicator-${i}`}
              style={{
                backgroundColor: i === Number(bar) ? 'green' : 'red',
                opacity: i === Number(bar) ? 1.0 : 0.0,
                width: '50px',
                height: '50px',
                borderRadius: '50%',
              }}
            ></div>
          ))}
        </div>
        <div>{bar}</div>
        <select
          onChange={(e) => store.event('variation-changed', e.target.value)}
        >
          <option value="a">A</option>
          <option value="ab" disabled>
            AB
          </option>
          <option value="b">B</option>
        </select>
        <button onClick={() => store.event('start-stop')}>Start / Stop</button>
      </header>
    </div>
  );
}

export default observer(App);
