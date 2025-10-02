import React from 'react';
import { useOidc, useOidcAccessToken } from '@axa-fr/react-oidc';
import GameWrapper from './components/GameWrapper';

const App: React.FC = () => {
  const { isAuthenticated, login } = useOidc();

  if (!isAuthenticated) {
    return (
      
        Battle with Friends
        <button
          onClick={() => login()}
          style={{
            padding: '12px 24px',
            fontSize: '16px',
            cursor: 'pointer',
            background: '#4CAF50',
            color: 'white',
            border: 'none',
            borderRadius: '4px'
          }}
        >
          Login to Play
        
      
    );
  }

  return ;
};

export default App;