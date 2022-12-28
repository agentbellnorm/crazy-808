import {createContext, PropsWithChildren, useContext} from 'react';
import Store from './Store';

const StoreContext = createContext<Store | null>(null);

type StoreProviderProps = {value: Store} & PropsWithChildren
export const StoreProvider = ({children, value}: StoreProviderProps) => <StoreContext.Provider value={value}>{children}</StoreContext.Provider>

export const useStore = (): Store => {
  const v = useContext(StoreContext);
  if (!v) {
    throw new Error("no value in context");
  }

  return v;
};