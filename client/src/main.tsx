import React from 'react';
import ReactDOM from 'react-dom/client';
import { AuthProvider } from '@axa-fr/react-oidc';
import { SpacetimeDBProvider } from '@spacetimedb/sdk';
import { DbConnection } from './autobindings';
import { getSpacetimeConfig, getStoredToken } from './config/spacetime';
import App from './App';

// Get environment-specific SpacetimeDB configuration
const spacetimeConfig = getSpacetimeConfig();

// Configure your SpacetimeDB OIDC settings here
const oidcConfig = {
  client_id: 'your-client-id',
  redirect_uri: window.location.origin + '/authentication/callback',
  silent_redirect_uri: window.location.origin + '/authentication/silent-callback',
  scope: 'openid profile email',
  authority: 'https://staging.spacetimedb.com',
  service_worker_only: false,
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