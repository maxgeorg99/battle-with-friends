import React from 'react';
import { useAuth } from 'react-oidc-context';
import GameWrapper from './components/GameWrapper';

const App: React.FC = () => {
  const { isAuthenticated, signinRedirect } = useAuth();

  if (!isAuthenticated) {
    return (
      <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100vh' }}>
        <h1>Battle with Friends</h1>
        <button
          onClick={() => signinRedirect()}
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
        </button>
      </div>
    );
  }

  return <GameWrapper />;
};

export default App;
