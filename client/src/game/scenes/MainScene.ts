import Phaser from 'phaser';
import { DbConnection, Player } from '../../autobindings';

export default class MainScene extends Phaser.Scene {
  private connection!: DbConnection;
  private localIdentity!: string;
  private username!: string;
  private localPlayer!: Phaser.GameObjects.Rectangle;
  private localPlayerText!: Phaser.GameObjects.Text;
  private otherPlayers: Map<string, { sprite: Phaser.GameObjects.Rectangle, text: Phaser.GameObjects.Text }> = new Map();
  private cursors!: Phaser.Types.Input.Keyboard.CursorKeys;
  private lastUpdateTime: number = 0;
  private updateThrottle: number = 50; // ms between updates

  constructor() {
    super('MainScene');
  }

  init() {
    this.connection = this.registry.get('connection');
    this.username = this.registry.get('username');
    this.localIdentity = this.registry.get('localIdentity');

    console.log('🎮 MainScene initialized', {
      hasConnection: !!this.connection,
      username: this.username,
      localIdentity: this.localIdentity
    });
  }

  create() {
    console.log('🎮 Creating game scene...');

    // Background
    this.add.rectangle(400, 300, 800, 600, 0x1a1a1a);

    // Create local player
    this.localPlayer = this.add.rectangle(400, 300, 32, 32, 0x00ff00);
    this.localPlayerText = this.add.text(400, 280, this.username || 'You', {
      fontSize: '12px',
      color: '#ffffff'
    }).setOrigin(0.5);

    // Setup controls
    this.cursors = this.input.keyboard!.createCursorKeys();

    // Subscribe to player table updates from SpacetimeDB
    if (this.connection) {
      const playerTable = this.connection.db.player;

      const syncPlayers = () => {
        this.syncPlayers();
      };

      playerTable.onInsert((player) => {
        console.log('➕ Player inserted:', player.name);
        syncPlayers();
      });

      playerTable.onUpdate((oldPlayer, newPlayer) => {
        console.log('🔄 Player updated:', newPlayer.name);
        syncPlayers();
      });

      playerTable.onDelete((player) => {
        console.log('➖ Player deleted:', player.name);
        syncPlayers();
      });

      // Initial sync
      syncPlayers();
    } else {
      console.error('❌ No connection available!');
    }
  }

  syncPlayers() {
    if (!this.connection) return;

    const players = Array.from(this.connection.db.player.iter());
    console.log('👥 Syncing players, total:', players.length);

    const currentIdentities = new Set<string>();

    players.forEach(player => {
      const identity = player.identity.toHexString();
      currentIdentities.add(identity);

      if (identity === this.localIdentity) {
        // Update local player name from server
        this.localPlayerText.setText(player.name);
        // Optionally sync position from server (for server authority)
        // this.localPlayer.setPosition(player.x, player.y);
        return;
      }

      if (!this.otherPlayers.has(identity)) {
        // Create new player sprite
        console.log('👤 Adding other player:', player.name, 'at', player.x, player.y);
        const sprite = this.add.rectangle(player.x, player.y, 32, 32, 0xff0000);
        const text = this.add.text(player.x, player.y - 20, player.name, {
          fontSize: '12px',
          color: '#ffffff'
        }).setOrigin(0.5);

        this.otherPlayers.set(identity, { sprite, text });
      } else {
        // Update existing player
        const playerObj = this.otherPlayers.get(identity)!;
        playerObj.sprite.setPosition(player.x, player.y);
        playerObj.text.setPosition(player.x, player.y - 20);
        playerObj.text.setText(player.name);

        // Update opacity based on online status
        const alpha = player.online ? 1 : 0.3;
        playerObj.sprite.setAlpha(alpha);
        playerObj.text.setAlpha(alpha);
      }
    });

    // Remove players that left
    this.otherPlayers.forEach((playerObj, identity) => {
      if (!currentIdentities.has(identity)) {
        console.log('👋 Removing player:', identity);
        playerObj.sprite.destroy();
        playerObj.text.destroy();
        this.otherPlayers.delete(identity);
      }
    });
  }

  update(time: number) {
    const speed = 200;
    let moved = false;

    if (this.cursors.left.isDown) {
      this.localPlayer.x -= speed * this.game.loop.delta / 1000;
      moved = true;
    } else if (this.cursors.right.isDown) {
      this.localPlayer.x += speed * this.game.loop.delta / 1000;
      moved = true;
    }

    if (this.cursors.up.isDown) {
      this.localPlayer.y -= speed * this.game.loop.delta / 1000;
      moved = true;
    } else if (this.cursors.down.isDown) {
      this.localPlayer.y += speed * this.game.loop.delta / 1000;
      moved = true;
    }

    // Update text position
    this.localPlayerText.setPosition(this.localPlayer.x, this.localPlayer.y - 20);

    // Throttled position updates to server
    if (moved && time - this.lastUpdateTime > this.updateThrottle) {
      if (this.connection) {
        this.connection.reducers.updatePosition(this.localPlayer.x, this.localPlayer.y);
      }
      this.lastUpdateTime = time;
    }
  }
}
