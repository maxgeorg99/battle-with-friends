import { Entity, EntityId } from './Entity';

export class World {
  private entities: Map<EntityId, Entity> = new Map();
  private systems: System[] = [];

  createEntity(): Entity {
    const entity = new Entity();
    this.entities.set(entity.id, entity);
    return entity;
  }

  getEntity(id: EntityId): Entity | undefined {
    return this.entities.get(id);
  }

  destroyEntity(id: EntityId): void {
    this.entities.delete(id);
  }

  addSystem(system: System): void {
    this.systems.push(system);
  }

  update(delta: number): void {
    for (const system of this.systems) {
      system.update(this, delta);
    }
  }

  queryEntities(componentTypes: string[]): Entity[] {
    const result: Entity[] = [];
    for (const entity of this.entities.values()) {
      if (componentTypes.every(type => entity.hasComponent(type))) {
        result.push(entity);
      }
    }
    return result;
  }

  clear(): void {
    this.entities.clear();
  }
}

export interface System {
  update(world: World, delta: number): void;
}
