// Component definitions for ECS

export interface GridPosition {
  x: number; // Grid column (0-9 for ship, -1+ for bench)
  y: number; // Grid row
  width: number; // Cell width (for multi-cell items)
  height: number; // Cell height
}

export interface CrewData {
  id: number; // SpacetimeDB crew ID
  name: string;
  rarity: 'Common' | 'Rare' | 'Epic' | 'Legendary';
  trait1: string;
  trait2?: string;
  maxHp: number;
  currentHp: number;
  attack: number;
  defense: number;
  level: number;
}

export interface ShopCrewData {
  id: number; // SpacetimeDB shop_crew ID
  name: string;
  rarity: 'Common' | 'Rare' | 'Epic' | 'Legendary';
  trait1: string;
  trait2?: string;
  maxHp: number;
  attack: number;
  defense: number;
  cost: number;
}

export interface Sprite {
  gameObject: Phaser.GameObjects.Container;
}

export interface Draggable {
  isDragging: boolean;
  originalPosition: { x: number; y: number };
  originalGridPos?: GridPosition;
}

export interface Clickable {
  onClick: () => void;
  onHover?: () => void;
  onHoverEnd?: () => void;
}

// Component type names
export const ComponentTypes = {
  GRID_POSITION: 'GridPosition',
  CREW_DATA: 'CrewData',
  SHOP_CREW_DATA: 'ShopCrewData',
  SPRITE: 'Sprite',
  DRAGGABLE: 'Draggable',
  CLICKABLE: 'Clickable',
} as const;
