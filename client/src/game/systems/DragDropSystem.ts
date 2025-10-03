import { World, System } from '../ecs/World';
import { ComponentTypes, Draggable, Sprite, GridPosition } from '../ecs/Components';

export class DragDropSystem implements System {
  private scene: Phaser.Scene;
  private gridCellSize = 64;
  private shipGridOffsetX = 200;
  private shipGridOffsetY = 400;

  constructor(scene: Phaser.Scene) {
    this.scene = scene;
  }

  update(world: World, delta: number): void {
    const draggableEntities = world.queryEntities([
      ComponentTypes.DRAGGABLE,
      ComponentTypes.SPRITE,
    ]);

    for (const entity of draggableEntities) {
      const draggable = entity.getComponent<Draggable>(ComponentTypes.DRAGGABLE)!;
      const sprite = entity.getComponent<Sprite>(ComponentTypes.SPRITE)!;

      if (!sprite.gameObject.input) {
        this.setupDragAndDrop(entity, world);
      }
    }
  }

  private setupDragAndDrop(entity: any, world: World): void {
    const sprite = entity.getComponent<Sprite>(ComponentTypes.SPRITE)!;
    const draggable = entity.getComponent<Draggable>(ComponentTypes.DRAGGABLE)!;

    sprite.gameObject.setInteractive({ draggable: true });

    sprite.gameObject.on('dragstart', (pointer: Phaser.Input.Pointer) => {
      draggable.isDragging = true;
      draggable.originalPosition = {
        x: sprite.gameObject.x,
        y: sprite.gameObject.y,
      };
      const gridPos = entity.getComponent<GridPosition>(ComponentTypes.GRID_POSITION);
      if (gridPos) {
        draggable.originalGridPos = { ...gridPos };
      }
      sprite.gameObject.setAlpha(0.7);
      sprite.gameObject.setDepth(1000);
    });

    sprite.gameObject.on('drag', (pointer: Phaser.Input.Pointer, dragX: number, dragY: number) => {
      sprite.gameObject.setPosition(dragX, dragY);
    });

    sprite.gameObject.on('dragend', (pointer: Phaser.Input.Pointer) => {
      draggable.isDragging = false;
      sprite.gameObject.setAlpha(1);
      sprite.gameObject.setDepth(0);

      // Snap to grid
      const gridPos = this.screenToGrid(sprite.gameObject.x, sprite.gameObject.y);

      if (this.isValidGridPosition(gridPos)) {
        // Update grid position component
        const gridPosComponent = entity.getComponent<GridPosition>(ComponentTypes.GRID_POSITION);
        if (gridPosComponent) {
          gridPosComponent.x = gridPos.x;
          gridPosComponent.y = gridPos.y;
        } else {
          entity.addComponent(ComponentTypes.GRID_POSITION, gridPos);
        }

        // Snap sprite to grid
        const screenPos = this.gridToScreen(gridPos.x, gridPos.y);
        sprite.gameObject.setPosition(screenPos.x, screenPos.y);

        // Emit event for backend sync
        this.scene.events.emit('crew-moved', entity.id, gridPos);
      } else {
        // Invalid position, return to original
        if (draggable.originalGridPos) {
          const screenPos = this.gridToScreen(
            draggable.originalGridPos.x,
            draggable.originalGridPos.y
          );
          sprite.gameObject.setPosition(screenPos.x, screenPos.y);
        } else {
          sprite.gameObject.setPosition(
            draggable.originalPosition.x,
            draggable.originalPosition.y
          );
        }
      }
    });

    this.scene.input.setDraggable(sprite.gameObject);
  }

  private screenToGrid(screenX: number, screenY: number): GridPosition {
    const x = Math.floor((screenX - this.shipGridOffsetX) / this.gridCellSize);
    const y = Math.floor((screenY - this.shipGridOffsetY) / this.gridCellSize);
    return { x, y, width: 1, height: 1 };
  }

  private gridToScreen(gridX: number, gridY: number): { x: number; y: number } {
    return {
      x: this.shipGridOffsetX + gridX * this.gridCellSize + this.gridCellSize / 2,
      y: this.shipGridOffsetY + gridY * this.gridCellSize + this.gridCellSize / 2,
    };
  }

  private isValidGridPosition(pos: GridPosition): boolean {
    // Ship has 10 slots (0-9) in a single row
    return pos.x >= 0 && pos.x <= 9 && pos.y === 0;
  }
}
