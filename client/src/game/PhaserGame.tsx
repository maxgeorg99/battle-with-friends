import React, { useEffect, useRef } from 'react';
import Phaser from 'phaser';
import MainScene from './scenes/MainScene';
import { GameBridge } from './GameBridge';

interface PhaserGameProps {
  bridge: GameBridge;
}

const PhaserGame: React.FC<PhaserGameProps> = ({ bridge }) => {
  const gameRef = useRef<Phaser.Game | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!gameRef.current && containerRef.current) {
      const config: Phaser.Types.Core.GameConfig = {
        type: Phaser.AUTO,
        width: 800,
        height: 600,
        parent: containerRef.current,
        backgroundColor: '#1a1a1a',
        physics: {
          default: 'arcade',
          arcade: {
            gravity: { y: 0 },
            debug: false,
          },
        },
        scene: MainScene,
      };

      gameRef.current = new Phaser.Game(config);

      // Pass bridge to Phaser
      gameRef.current.registry.set('gameBridge', bridge);
    }

    return () => {
      if (gameRef.current) {
        gameRef.current.destroy(true);
        gameRef.current = null;
      }
    };
  }, [bridge]);

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