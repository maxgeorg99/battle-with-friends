import React, { createContext, useContext, ReactNode } from 'react';
import { DbConnection } from '../autobindings';

interface SpacetimeDBContextType {
  connection: DbConnection;
}

const SpacetimeDBContext = createContext<SpacetimeDBContextType | null>(null);

export function SpacetimeDBProvider({
  connection,
  children
}: {
  connection: DbConnection;
  children: ReactNode
}) {
  return (
    <SpacetimeDBContext.Provider value={{ connection }}>
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
