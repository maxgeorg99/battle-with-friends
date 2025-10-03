import { Player } from '../autobindings';

/**
 * Bridge between React (with SpacetimeDB hooks) and Phaser
 * This allows us to use React hooks for data while keeping Phaser separate
 */
export class GameBridge {
  private _players: Player[] = [];
  private _localIdentity: string = '';
  private _listeners: Set<() => void> = new Set();
  
  public onPositionUpdate: ((x: number, y: number) => void) | null = null;

  updatePlayers(players: Player[], localIdentity: string) {
    this._players = players;
    this._localIdentity = localIdentity;
    this._notifyListeners();
  }

  getPlayers(): Player[] {
    return this._players;
  }

  getLocalIdentity(): string {
    return this._localIdentity;
  }

  updatePosition(x: number, y: number) {
    if (this.onPositionUpdate) {
      this.onPositionUpdate(x, y);
    }
  }

  subscribe(callback: () => void) {
    this._listeners.add(callback);
    return () => this._listeners.delete(callback);
  }

  private _notifyListeners() {
    this._listeners.forEach(cb => cb());
  }
}