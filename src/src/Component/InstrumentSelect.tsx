import { observer } from 'mobx-react-lite';
import { useStore } from '../StoreContext';

const InstrumentSelect = () => {
  const store = useStore();

  return (
    <select
      value={`${store.state?.selected_instrument ?? 1}`}
      onChange={(e) => store.event('instrument-selected', e.target.value)}
    >
      <option value="0">ac</option>
      <option value="1">bd</option>
      <option value="2">sd</option>
      <option value="3">lt</option>
      <option value="4">mt</option>
      <option value="5">ht</option>
      <option value="6">lc</option>
      <option value="7">mc</option>
      <option value="8">hc</option>
      <option value="9">rs</option>
      <option value="10">cl</option>
      <option value="11">cp</option>
      <option value="12">ma</option>
      <option value="13">cb</option>
      <option value="14">cy</option>
      <option value="15">oh</option>
      <option value="16">ch</option>
    </select>
  );
};

export default observer(InstrumentSelect);
