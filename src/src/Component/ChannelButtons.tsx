import { observer } from 'mobx-react-lite';
import { useStore } from '../StoreContext';

const getColor = (bar: number, active: number, currentBar: number) => {
  if (bar === currentBar) {
    return 'green';
  }

  if (active) {
    return 'orange';
  }

  return 'grey';
};

const ChannelButtons = () => {
  const store = useStore();
  return (
    <div
      style={{ display: 'flex', justifyContent: 'space-between', width: '60%' }}
    >
      {store.selectedInstrumentBars.map((channel, index) => (
        <div
          key={`channelbutton-${index}`}
          style={{
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            width: '25px',
          }}
        >
          <div
            style={{
              width: '20px',
              height: '20px',
              backgroundColor: getColor(index, channel, store.bar),
            }}
          ></div>
          <button
            onMouseDown={() => store.event('channel-pressed', index.toString())}
          >
            {' '}
            *{' '}
          </button>
        </div>
      ))}
    </div>
  );
};

export default observer(ChannelButtons);
