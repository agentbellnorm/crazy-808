import { useStore } from '../StoreContext';

const ChannelIndicators = () => {
  const { state } = useStore();
  return (
    <div style={{ display: 'flex' }}>
      {[...Array(16).keys()].map((i) => (
        <div
          key={`indicator-${i}`}
          style={{
            backgroundColor: i === Number(state?.bar) ? 'green' : 'red',
            opacity: i === Number(state?.bar) ? 1.0 : 0.0,
            width: '50px',
            height: '50px',
            borderRadius: '50%',
          }}
        ></div>
      ))}
    </div>
  );
};

export default ChannelIndicators;
