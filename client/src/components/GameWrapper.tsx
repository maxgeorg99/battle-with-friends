import React, { useEffect, useState } from 'react';
import { useOidcAccessToken } from '@axa-fr/react-oidc';
import { useSpacetimeDB, useTable } from '@spacetimedb/sdk';
import { DbConnection, Player } from '../autobindings';
import PhaserGame from '../game/PhaserGame';
import { GameBridge } from '../game/GameBridge';

const GameWrapper: React.FC = () => {
  const { accessToken, accessTokenPayload } = useOidcAccessToken();
  const conn = useSpacetimeDB();
  const { rows: players } = useTable('player');
  const [gameBridge] = useState(() => new GameBridge());
  const [registered, setRegistered] = useState(false);

  // Connect with auth token
  useEffect(() => {
    if (accessToken && conn) {
      conn.connect({ token: accessToken }).then(() => {
        console.log('Connected to SpacetimeDB');
      });
    }
  }, [accessToken, conn]);

  // Register player once connected
  useEffect(() => {
    if (conn?.connected && !registered && accessTokenPayload?.name) {
      conn.db.register_player(accessTokenPayload.name);
      setRegistered(true);
    }
  }, [conn?.connected, registered, accessTokenPayload]);

  // Sync players to game via bridge
  useEffect(() => {
    if (players && conn?.identity) {
      gameBridge.updatePlayers(players, conn.identity.toHexString());
    }
  }, [players, conn?.identity, gameBridge]);

  // Setup position update callback from Phaser
  useEffect(() => {
    if (conn) {
      gameBridge.onPositionUpdate = (x: number, y: number) => {
        conn.db.update_position(x, y);
      };
    }
  }, [conn, gameBridge]);

  if (!conn?.connected) {
    return (
      
        Connecting to game server...
      
    );
  }

  return (
    
  );
};

export default GameWrapper;