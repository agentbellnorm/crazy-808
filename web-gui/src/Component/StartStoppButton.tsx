import { observer } from 'mobx-react-lite';
import { useStore } from '../StoreContext';

const StartStopButton = () => {
  const store = useStore();
  return (
    <button onMouseDown={() => store.event('start-stop')}>Start / Stop</button>
  );
};

export default observer(StartStopButton);
