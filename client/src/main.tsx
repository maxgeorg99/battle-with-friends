import React from 'react';
import ReactDOM from 'react-dom/client';
import { AuthProvider } from '@axa-fr/react-oidc';
import { SpacetimeDBProvider } from '@spacetimedb/sdk';
import { DbConnection } from './autobindings';
import { getSpacetimeConfig, getStoredToken } from './config/spacetime';
import App from './App';

// Get environment-specific SpacetimeDB configuration
const spacetimeConfig = getSpacetimeConfig();

const oidcConfig = {
  authority: 'https://spacetimeauth.staging.spacetimedb.com/oidc',
  client_id: 'client_031CSnBZhPFgz5oj5Alo0a',
  redirect_uri: `${window.location.origin}/callback`,
  scope: 'openid profile email',
  response_type: 'code',
  automaticSilentRenew: true,
};

function onSigninCallback() {
  window.history.replaceState({}, document.title, window.location.pathname);
}

// SpacetimeDB connection builder with environment-specific config
const connectionBuilder = DbConnection.builder()
  .withUri(spacetimeConfig.uri)
  .withModuleName(spacetimeConfig.moduleName)
  .withToken(getStoredToken() || '');

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  
    
      
        
      
    
  
);