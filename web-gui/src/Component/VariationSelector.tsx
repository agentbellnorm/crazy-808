import { observer } from 'mobx-react-lite';
import { useStore } from '../StoreContext';

const VariationSelector = () => {
  const store = useStore();

  return (
    <select
      value={store.state.current_variation}
      onChange={(e) => store.event('variation-changed', e.target.value)}
    >
      <option value="a">A</option>
      <option value="ab">AB</option>
      <option value="b">B</option>
    </select>
  );
};

export default observer(VariationSelector);
