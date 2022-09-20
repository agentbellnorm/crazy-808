import { observer } from 'mobx-react-lite';
import { useStore } from '../StoreContext';

const InstrumentSelect = () => {
  const store = useStore();

  return (
    <select
      value={store.state?.selected_instrument ?? 'bd'}
      onChange={(e) => store.event('instrument-selected', e.target.value)}
    >
      <option value="bd">bd</option>
      <option value="sd">sd</option>
      <option value="lt">lt</option>
      <option value="mt">mt</option>
      <option value="ht">ht</option>
      <option value="lc">lc</option>
      <option value="mc">mc</option>
      <option value="hc">hc</option>
      <option value="rs">rs</option>
      <option value="cl">cl</option>
      <option value="cp">cp</option>
      <option value="ma">ma</option>
      <option value="cb">cb</option>
      <option value="cy">cy</option>
      <option value="oh">oh</option>
      <option value="ch">ch</option>
    </select>
  );
};

export default observer(InstrumentSelect);
