import './App.css';
import { observer } from 'mobx-react-lite';
import InstrumentSelect from './Component/InstrumentSelect';
import ChannelButtons from './Component/ChannelButtons';
import VariationSelector from './Component/VariationSelector';
import StartStoppButton from './Component/StartStoppButton';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <InstrumentSelect />
        <VariationSelector />
        <ChannelButtons />
        <StartStoppButton />
      </header>
    </div>
  );
}

export default observer(App);
