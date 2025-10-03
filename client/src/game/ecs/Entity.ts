// Simple ECS implementation for the game

export type EntityId = number;

let nextEntityId = 1;

export class Entity {
  public readonly id: EntityId;
  private components: Map<string, any> = new Map();

  constructor() {
    this.id = nextEntityId++;
  }

  addComponent<T>(componentType: string, component: T): this {
    this.components.set(componentType, component);
    return this;
  }

  getComponent<T>(componentType: string): T | undefined {
    return this.components.get(componentType);
  }

  hasComponent(componentType: string): boolean {
    return this.components.has(componentType);
  }

  removeComponent(componentType: string): void {
    this.components.delete(componentType);
  }
}
