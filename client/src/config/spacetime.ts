const SPACETIME_DB_LIVE = "battle-with-friends";
const SPACETIME_DB_DEV = "battle-with-friends";

const LOCAL_SPACETIMEDB_URI = "ws://localhost:3000";
const REMOTE_SPACETIMEDB_URI = "wss://maincloud.spacetimedb.com";
const STAGING_SPACETIMEDB_URI = "wss://staging.spacetimedb.com";

export enum Environment {
  Local = "local",
  Staging = "staging",
  Production = "production",
}

/**
 * Detect the current environment based on hostname and port
 */
export function detectEnvironment(): Environment {
  if (typeof window === 'undefined') {
    return Environment.Local;
  }

  const hostname = window.location.hostname;
  const port = window.location.port;

  // Local development
  if (hostname === 'localhost' || hostname === '127.0.0.1' || port === '3000') {
    return Environment.Local;
  }

  // Staging environment (customize based on your setup)
  if (hostname.includes('staging') || hostname.includes('dev')) {
    return Environment.Staging;
  }

  // Production
  return Environment.Production;
}

/**
 * Get the appropriate SpacetimeDB configuration based on environment
 */
export function getSpacetimeConfig() {
  const env = detectEnvironment();

  switch (env) {
    case Environment.Local:
      return {
        uri: LOCAL_SPACETIMEDB_URI,
        moduleName: SPACETIME_DB_DEV,
        tokenKey: 'local_spacetime_token',
        environment: env,
      };

    case Environment.Staging:
      return {
        uri: STAGING_SPACETIMEDB_URI,
        moduleName: SPACETIME_DB_DEV,
        tokenKey: 'staging_spacetime_token',
        environment: env,
      };

    case Environment.Production:
      return {
        uri: REMOTE_SPACETIMEDB_URI,
        moduleName: SPACETIME_DB_LIVE,
        tokenKey: 'spacetime_token',
        environment: env,
      };
  }
}

/**
 * Get stored auth token for current environment
 */
export function getStoredToken(): string | null {
  const config = getSpacetimeConfig();
  return localStorage.getItem(config.tokenKey);
}

/**
 * Store auth token for current environment
 */
export function storeToken(token: string): void {
  const config = getSpacetimeConfig();
  localStorage.setItem(config.tokenKey, token);
}

/**
 * Clear auth token for current environment
 */
export function clearToken(): void {
  const config = getSpacetimeConfig();
  localStorage.removeItem(config.tokenKey);
}

// Log current configuration on import (dev only)
if (import.meta.env.DEV) {
  const config = getSpacetimeConfig();
  console.log('🚀 SpacetimeDB Config:', {
    environment: config.environment,
    uri: config.uri,
    module: config.moduleName,
  });
}