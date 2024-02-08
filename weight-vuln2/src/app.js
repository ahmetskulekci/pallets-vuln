import React, { useState } from 'react';
import PalletProvider from './PalletProvider';

const App = () => {
  const [something, setSomething] = useState('');

  const getSomething = async () => {
    const result = await api.query.myPallet.getSomething();
    setSomething(result.toString());
  };

  return (
    <div>
      <PalletProvider pallet={myPallet}>
        <button onClick={getSomething}>Get Something</button>
        <p>{something}</p>
      </PalletProvider>
    </div>
  );
};

export default App;
