import Phaser from 'phaser';
import { DbConnection } from '../../autobindings';
import { Identity } from 'spacetimedb';
import { World } from '../ecs/World';
import { DragDropSystem } from '../systems/DragDropSystem';
import { ComponentTypes, CrewData, ShopCrewData, GridPosition, Sprite, Draggable } from '../ecs/Components';

export default class ShipScene extends Phaser.Scene {
  private connection!: DbConnection;
  private localIdentity!: Identity;
  private username!: string;
  private world!: World;

  // UI elements
  private berriesText!: Phaser.GameObjects.Text;
  private bountyText!: Phaser.GameObjects.Text;
  private shipTypeText!: Phaser.GameObjects.Text;
  private shopContainer!: Phaser.GameObjects.Container;

  constructor() {
    super('ShipScene');
  }

  preload() {
    console.log('🔄 ShipScene preload started');

    const baseUrl = import.meta.env.BASE_URL || '/';

    // Load crew portraits for shop
    this.load.image('shop-luffy', `${baseUrl}assets/shop/icon/Luffy.JPG`);
    this.load.image('shop-lysop', `${baseUrl}assets/shop/icon/Lysop.JPG`);
    this.load.image('shop-nami', `${baseUrl}assets/shop/icon/Nami.JPG`);
    this.load.image('shop-zoro', `${baseUrl}assets/shop/icon/Zoro.JPG`);

    // Load unit sprites for field
    this.load.image('unit-luffy', `${baseUrl}assets/unit/Luffy.PNG`);
    this.load.image('unit-lysop', `${baseUrl}assets/unit/Lysop.PNG`);
    this.load.image('unit-nami', `${baseUrl}assets/unit/Nami.PNG`);
    this.load.image('unit-zoro', `${baseUrl}assets/unit/Zoro.PNG`);

    // Load trait icons
    this.load.image('trait-marines', `${baseUrl}assets/trait/Marines.png`);
    this.load.image('trait-logia', `${baseUrl}assets/trait/Logia.png`);
    this.load.image('trait-paramecia', `${baseUrl}assets/trait/Paramecia.png`);
    this.load.image('trait-zoan', `${baseUrl}assets/trait/Zoan.png`);

    // Add load event listeners
    this.load.on('complete', () => {
      console.log('✅ ShipScene: All assets loaded successfully');
    });

    this.load.on('loaderror', (fileObj: any) => {
      console.error('❌ ShipScene: Error loading asset:', fileObj.key, fileObj.url);
    });
  }

  init() {
    this.connection = this.registry.get('connection');
    this.username = this.registry.get('username');
    this.localIdentity = this.registry.get('localIdentity');

    console.log('🚢 ShipScene initialized', {
      hasConnection: !!this.connection,
      hasIdentity: !!this.localIdentity,
      identityValue: this.localIdentity,
      username: this.username
    });

    if (!this.connection || !this.localIdentity) {
      console.error('❌ Missing connection or identity!', {
        connection: this.connection,
        localIdentity: this.localIdentity
      });
      return;
    }

    // Initialize ECS
    this.world = new World();
    this.world.addSystem(new DragDropSystem(this));
  }

  create() {
    console.log('🎨 ShipScene create() started');

    // Guard against missing connection/identity
    if (!this.connection || !this.localIdentity) {
      console.error('❌ Cannot create scene without connection/identity');
      return;
    }

    // Background
    this.add.rectangle(512, 384, 1024, 768, 0x4a90e2); // Ocean blue

    // Create UI
    this.createUI();

    // Create ship grid
    this.createShipGrid();

    // Subscribe to SpacetimeDB updates
    this.subscribeToDatabase();

    // Listen for crew movement events
    this.events.on('crew-moved', this.onCrewMoved, this);

    // Initial data load
    this.loadPlayerData();
    this.loadCrewData();
    this.loadShopData();

    console.log('✅ ShipScene create() completed');
  }

  update(time: number, delta: number) {
    this.world.update(delta);
  }

  private createUI() {
    // Top bar - Player info
    const topBar = this.add.rectangle(512, 40, 1024, 80, 0xf4e4c1);

    this.berriesText = this.add.text(50, 20, 'Berries: 100', {
      fontSize: '24px',
      color: '#000',
      fontStyle: 'bold',
    });

    this.bountyText = this.add.text(250, 20, 'Bounty: 0', {
      fontSize: '24px',
      color: '#8b0000',
      fontStyle: 'bold',
    });

    this.shipTypeText = this.add.text(450, 20, 'Ship: Raft', {
      fontSize: '24px',
      color: '#000',
      fontStyle: 'bold',
    });

    // Battle button
    const battleBtn = this.add.rectangle(900, 40, 150, 50, 0xff4444)
      .setInteractive({ useHandCursor: true })
      .on('pointerdown', () => this.startBattle());

    this.add.text(900, 40, 'START BATTLE', {
      fontSize: '14px',
      color: '#fff',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Shop section (top)
    const shopBg = this.add.rectangle(512, 150, 900, 120, 0xf4e4c1);

    this.add.text(512, 100, 'RECRUIT', {
      fontSize: '32px',
      color: '#000',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    this.shopContainer = this.add.container(0, 0);

    // Refresh button
    const refreshBtn = this.add.rectangle(900, 600, 150, 50, 0x44ff44)
      .setInteractive({ useHandCursor: true })
      .on('pointerdown', () => this.refreshShop());

    this.add.text(900, 600, 'REFRESH', {
      fontSize: '14px',
      color: '#000',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    // Treasure Island button (bottom left)
    const treasureBtn = this.add.rectangle(100, 650, 150, 100, 0xf4e4c1)
      .setInteractive({ useHandCursor: true });

    this.add.text(100, 620, 'TREASURE', {
      fontSize: '16px',
      color: '#000',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    this.add.text(100, 650, 'ISLAND', {
      fontSize: '16px',
      color: '#000',
      fontStyle: 'bold',
    }).setOrigin(0.5);

    this.add.text(100, 680, '1400', {
      fontSize: '24px',
      color: '#000',
      fontStyle: 'bold',
    }).setOrigin(0.5);
  }

  private createShipGrid() {
    const gridX = 200;
    const gridY = 400;
    const cellSize = 64;

    // Draw ship grid (10 slots in a row)
    for (let i = 0; i < 10; i++) {
      const x = gridX + i * cellSize;
      const y = gridY;

      // Grid cell background
      this.add.rectangle(x, y, cellSize - 4, cellSize - 4, 0x8b6f47, 0.5)
        .setStrokeStyle(2, 0x000000);
    }

    // Ship label
    this.add.text(512, 300, 'RAFT', {
      fontSize: '32px',
      color: '#f4e4c1',
      fontStyle: 'bold',
      stroke: '#000',
      strokeThickness: 4,
    }).setOrigin(0.5);
  }

  private subscribeToDatabase() {
    if (!this.connection || !this.localIdentity) return;

    // Subscribe to player updates
    this.connection.db.player.onUpdate((oldPlayer, newPlayer) => {
      if (this.localIdentity && newPlayer.identity.isEqual(this.localIdentity)) {
        this.updatePlayerUI(newPlayer);
      }
    });

    // Subscribe to crew updates
    this.connection.db.crew.onInsert((crew) => {
      if (this.localIdentity && crew.owner.isEqual(this.localIdentity)) {
        this.addCrewEntity(crew);
      }
    });

    this.connection.db.crew.onUpdate((oldCrew, newCrew) => {
      if (this.localIdentity && newCrew.owner.isEqual(this.localIdentity)) {
        this.updateCrewEntity(newCrew);
      }
    });

    this.connection.db.crew.onDelete((crew) => {
      if (this.localIdentity && crew.owner.isEqual(this.localIdentity)) {
        this.removeCrewEntity(crew.id);
      }
    });

    // Subscribe to shop updates
    this.connection.db.shopCrew.onInsert((shopCrew) => {
      if (this.localIdentity && shopCrew.player.isEqual(this.localIdentity)) {
        this.addShopCrewCard(shopCrew);
      }
    });

    this.connection.db.shopCrew.onDelete((shopCrew) => {
      if (this.localIdentity && shopCrew.player.isEqual(this.localIdentity)) {
        this.removeShopCrewCard(shopCrew.id);
      }
    });
  }

  private loadPlayerData() {
    if (!this.connection || !this.localIdentity) return;

    const player = Array.from(this.connection.db.player.iter())
      .find(p => p.identity.isEqual(this.localIdentity));

    if (player) {
      this.updatePlayerUI(player);
    }
  }

  private loadCrewData() {
    if (!this.connection || !this.localIdentity) return;

    const crewList = Array.from(this.connection.db.crew.iter())
      .filter(c => c.owner.isEqual(this.localIdentity));

    for (const crew of crewList) {
      this.addCrewEntity(crew);
    }
  }

  private loadShopData() {
    if (!this.connection || !this.localIdentity) return;

    const shopList = Array.from(this.connection.db.shopCrew.iter())
      .filter(s => s.player.isEqual(this.localIdentity));

    for (const shopCrew of shopList) {
      this.addShopCrewCard(shopCrew);
    }
  }

  private updatePlayerUI(player: any) {
    this.berriesText.setText(`Berries: ${player.berries}`);
    this.bountyText.setText(`Bounty: ${player.bounty}`);
    this.shipTypeText.setText(`Ship: ${player.shipType}`);
  }

  private addCrewEntity(crew: any) {
    const entity = this.world.createEntity();

    // Add crew data component
    const crewData: CrewData = {
      id: Number(crew.id),
      name: crew.name,
      rarity: crew.rarity,
      trait1: crew.trait1,
      trait2: crew.trait2,
      maxHp: crew.maxHp,
      currentHp: crew.currentHp,
      attack: crew.attack,
      defense: crew.defense,
      level: crew.level,
    };
    entity.addComponent(ComponentTypes.CREW_DATA, crewData);

    // Create sprite
    const container = this.createCrewCard(crew, false);
    entity.addComponent(ComponentTypes.SPRITE, { gameObject: container });

    // Add grid position
    if (crew.slotIndex !== null && crew.slotIndex !== undefined) {
      const gridPos: GridPosition = {
        x: crew.slotIndex,
        y: 0,
        width: 1,
        height: 1,
      };
      entity.addComponent(ComponentTypes.GRID_POSITION, gridPos);

      // Position on grid
      const screenPos = this.gridToScreen(gridPos.x, gridPos.y);
      container.setPosition(screenPos.x, screenPos.y);
    }

    // Make draggable
    const draggable: Draggable = {
      isDragging: false,
      originalPosition: { x: container.x, y: container.y },
    };
    entity.addComponent(ComponentTypes.DRAGGABLE, draggable);
  }

  private createCrewCard(crew: any, isShopCard: boolean): Phaser.GameObjects.Container {
    const container = this.add.container(0, 0);

    // Map crew name to sprite key
    const getCrewSpriteKey = (name: string): string => {
      const nameLower = name.toLowerCase().replace(/\s+/g, '');
      if (nameLower.includes('luffy')) return isShopCard ? 'shop-luffy' : 'unit-luffy';
      if (nameLower.includes('zoro')) return isShopCard ? 'shop-zoro' : 'unit-zoro';
      if (nameLower.includes('nami')) return isShopCard ? 'shop-nami' : 'unit-nami';
      if (nameLower.includes('usopp') || nameLower.includes('lysop')) return isShopCard ? 'shop-lysop' : 'unit-lysop';
      return ''; // Fallback - will show colored rectangle
    };

    const spriteKey = getCrewSpriteKey(crew.name);

    // Card background based on rarity
    const rarityColors: Record<string, number> = {
      Common: 0xcccccc,
      Rare: 0x4169e1,
      Epic: 0x9370db,
      Legendary: 0xffd700,
    };

    if (spriteKey && this.textures.exists(spriteKey)) {
      // Use sprite image
      const sprite = this.add.image(0, 0, spriteKey);
      sprite.setDisplaySize(100, 120);
      container.add(sprite);

      // Add rarity border
      const border = this.add.rectangle(0, 0, 100, 120)
        .setStrokeStyle(3, rarityColors[crew.rarity] || 0xcccccc)
        .setFillStyle(0x000000, 0);
      container.add(border);
    } else {
      // Fallback colored rectangle
      const bg = this.add.rectangle(0, 0, 100, 120, rarityColors[crew.rarity] || 0xcccccc)
        .setStrokeStyle(2, 0x000000);
      container.add(bg);
    }

    // Crew name
    const nameText = this.add.text(0, -55, crew.name, {
      fontSize: '10px',
      color: '#fff',
      fontStyle: 'bold',
      align: 'center',
      wordWrap: { width: 90 },
      backgroundColor: '#000000',
      padding: { x: 4, y: 2 },
    }).setOrigin(0.5);
    container.add(nameText);

    // Stats overlay
    const statsText = this.add.text(0, 50,
      `HP: ${crew.currentHp || crew.maxHp} ATK: ${crew.attack} DEF: ${crew.defense}`,
      {
        fontSize: '8px',
        color: '#fff',
        align: 'center',
        backgroundColor: '#000000',
        padding: { x: 4, y: 2 },
      }
    ).setOrigin(0.5);
    container.add(statsText);

    // Cost (for shop cards)
    if (isShopCard) {
      const costText = this.add.text(0, 60, `⚡${crew.cost}`, {
        fontSize: '12px',
        color: '#ffd700',
        fontStyle: 'bold',
        backgroundColor: '#000000',
        padding: { x: 4, y: 2 },
      }).setOrigin(0.5);
      container.add(costText);
    }

    return container;
  }

  private addShopCrewCard(shopCrew: any) {
    // Create shop card
    const card = this.createCrewCard(shopCrew, true);

    // Position in shop
    const shopIndex = this.shopContainer.list.length;
    const x = 150 + shopIndex * 120;
    card.setPosition(x, 150);

    // Make clickable to buy - use setSize for Container hit area
    card.setSize(100, 120);
    card.setInteractive(new Phaser.Geom.Rectangle(-50, -60, 100, 120), Phaser.Geom.Rectangle.Contains);
    card.input!.cursor = 'pointer';
    card.on('pointerdown', () => this.buyCrewFromShop(shopCrew.id));

    this.shopContainer.add(card);
  }

  private removeShopCrewCard(shopCrewId: bigint) {
    // Remove from shop container
    this.shopContainer.removeAll(true);
  }

  private updateCrewEntity(crew: any) {
    // Find entity by crew ID and update it
    // For now, just recreate
    this.removeCrewEntity(crew.id);
    this.addCrewEntity(crew);
  }

  private removeCrewEntity(crewId: bigint) {
    const entities = this.world.queryEntities([ComponentTypes.CREW_DATA]);
    for (const entity of entities) {
      const crewData = entity.getComponent<CrewData>(ComponentTypes.CREW_DATA);
      if (crewData && crewData.id === Number(crewId)) {
        const sprite = entity.getComponent<Sprite>(ComponentTypes.SPRITE);
        if (sprite) {
          sprite.gameObject.destroy();
        }
        this.world.destroyEntity(entity.id);
        break;
      }
    }
  }

  private gridToScreen(gridX: number, gridY: number): { x: number; y: number } {
    const gridOffsetX = 200;
    const gridOffsetY = 400;
    const cellSize = 64;
    return {
      x: gridOffsetX + gridX * cellSize + cellSize / 2,
      y: gridOffsetY + gridY * cellSize + cellSize / 2,
    };
  }

  private onCrewMoved(entityId: number, gridPos: GridPosition) {
    console.log('Crew moved to grid position:', gridPos);
    // TODO: Call connection.reducers.moveCrew(crewId, gridPos.x)
  }

  private buyCrewFromShop(shopCrewId: bigint) {
    if (!this.connection || !this.localIdentity) return;

    console.log('Buying crew from shop:', shopCrewId);
    // Find empty slot
    const occupiedSlots = Array.from(this.connection.db.crew.iter())
      .filter(c => c.owner.isEqual(this.localIdentity) && c.slotIndex !== null)
      .map(c => c.slotIndex);

    for (let i = 0; i < 10; i++) {
      if (!occupiedSlots.includes(i)) {
        this.connection.reducers.buyCrew(shopCrewId, i);
        return;
      }
    }

    console.warn('No empty slots!');
  }

  private refreshShop() {
    this.connection.reducers.refreshShop();
  }

  private startBattle() {
    this.connection.reducers.startBattle();
    // TODO: Switch to battle scene
  }
}
