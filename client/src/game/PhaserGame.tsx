import React, { useEffect, useRef } from 'react';
import Phaser from 'phaser';
import ShipScene from './scenes/ShipScene';
import { useSpacetimeDBConnection } from '../components/SpacetimeDBProvider';

const PhaserGame: React.FC = () => {
  const gameRef = useRef<Phaser.Game | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const { connection } = useSpacetimeDBConnection();

  useEffect(() => {
    // Wait for both connection AND identity to be available
    if (!gameRef.current && containerRef.current && connection && connection.identity) {
      console.log('🎮 Creating Phaser game with identity:', connection.identity.toHexString());

      const config: Phaser.Types.Core.GameConfig = {
        type: Phaser.AUTO,
        width: 1024,
        height: 768,
        parent: containerRef.current,
        backgroundColor: '#4a90e2',
        scene: ShipScene,
      };

      gameRef.current = new Phaser.Game(config);

      // Pass connection to Phaser
      gameRef.current.registry.set('connection', connection);
      gameRef.current.registry.set('localIdentity', connection.identity);
    }

    return () => {
      if (gameRef.current) {
        gameRef.current.destroy(true);
        gameRef.current = null;
      }
    };
  }, [connection]);

  return (
    <div
      ref={containerRef}
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        width: '100vw',
        height: '100vh',
        backgroundColor: '#1a1a1a'
      }}
    />
  );
};

export default PhaserGame;