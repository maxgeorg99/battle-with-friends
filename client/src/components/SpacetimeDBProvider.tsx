import React, { createContext, useContext, ReactNode, useState, useEffect } from 'react';
import { DbConnection } from '../autobindings';

interface SpacetimeDBContextType {
  connection: DbConnection;
  isConnected: boolean;
}

const SpacetimeDBContext = createContext<SpacetimeDBContextType | null>(null);

export function SpacetimeDBProvider({
  connection,
  children
}: {
  connection: DbConnection;
  children: ReactNode
}) {
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    // Check if already connected
    if (connection.identity) {
      setIsConnected(true);
    }

    // Poll for connection (SpacetimeDB doesn't have a good event for this)
    const interval = setInterval(() => {
      if (connection.identity && !isConnected) {
        console.log('🔗 SpacetimeDB connection ready');
        setIsConnected(true);
      }
    }, 100);

    return () => clearInterval(interval);
  }, [connection, isConnected]);

  return (
    <SpacetimeDBContext.Provider value={{ connection, isConnected }}>
      {children}
    </SpacetimeDBContext.Provider>
  );
}

export function useSpacetimeDB(): DbConnection {
  const context = useContext(SpacetimeDBContext);
  if (!context) {
    throw new Error('useSpacetimeDB must be used within SpacetimeDBProvider');
  }
  return context.connection;
}

export function useSpacetimeDBConnection(): SpacetimeDBContextType {
  const context = useContext(SpacetimeDBContext);
  if (!context) {
    throw new Error('useSpacetimeDBConnection must be used within SpacetimeDBProvider');
  }
  return context;
}
