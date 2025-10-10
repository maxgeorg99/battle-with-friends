import { useState, useEffect } from 'react';
import './App.css';
import { DbConnection, CrewTemplate, ItemComponent, CompletedItem } from './autobindings';

function App() {
  const [activeTab, setActiveTab] = useState<'crew' | 'items' | 'completed'>('crew');
  const [connection, setConnection] = useState<DbConnection | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    let mounted = true;

    // Connect to SpacetimeDB
    const connectToSpacetime = async () => {
      try {
        const conn = DbConnection.builder()
          .withUri('ws://localhost:3000')
          .withModuleName('battle-with-friends')
          .onConnect((conn, identity, token) => {
            if (!mounted) return;

            console.log('Connected to SpacetimeDB!', identity);

            // Subscribe after a short delay to ensure connection is ready
            setTimeout(() => {
              try {
                console.log('Subscribing to tables...');
                conn.subscriptionBuilder().subscribeToAllTables();
                console.log('Subscription successful');
              } catch (err) {
                console.error('Subscription failed:', err);
              }
            }, 100);

            setConnection(conn);
            setIsConnected(true);
          })
          .build();
      } catch (error) {
        console.error('Failed to connect to SpacetimeDB:', error);
      }
    };

    connectToSpacetime();

    return () => {
      mounted = false;
    };
  }, []);

  return (
    <div className="admin-panel">
      <header>
        <h1>⚙️ Battle with Friends - Admin Panel</h1>
        <div className="connection-status">
          {isConnected ? (
            <span className="connected">🟢 Connected</span>
          ) : (
            <span className="disconnected">🔴 Disconnected</span>
          )}
        </div>
      </header>

      <nav className="tabs">
        <button
          className={activeTab === 'crew' ? 'active' : ''}
          onClick={() => setActiveTab('crew')}
        >
          👥 Crew Templates
        </button>
        <button
          className={activeTab === 'items' ? 'active' : ''}
          onClick={() => setActiveTab('items')}
        >
          🗡️ Item Components
        </button>
        <button
          className={activeTab === 'completed' ? 'active' : ''}
          onClick={() => setActiveTab('completed')}
        >
          ⚔️ Completed Items
        </button>
      </nav>

      <main className="content">
        {activeTab === 'crew' && <CrewTemplatesPanel connection={connection} />}
        {activeTab === 'items' && <ItemComponentsPanel connection={connection} />}
        {activeTab === 'completed' && <CompletedItemsPanel connection={connection} />}
      </main>
    </div>
  );
}

// Crew Templates Panel
function CrewTemplatesPanel({ connection }: { connection: DbConnection | null }) {
  const [crews, setCrews] = useState<CrewTemplate[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (!connection) return;

    // Load crew templates from cache
    const loadCrews = () => {
      try {
        const crewList = Array.from(connection.db.crewTemplate.iter());
        console.log('Loaded crews from cache:', crewList.length);
        setCrews(crewList);
        setIsLoading(false);
      } catch (error) {
        console.error('Failed to load crews:', error);
        setIsLoading(false);
      }
    };

    // Load immediately
    loadCrews();

    // Subscribe to changes
    const handleInsert = (ctx: any, crew: CrewTemplate) => {
      console.log('Crew inserted:', crew);
      setCrews(prev => [...prev, crew]);
    };

    const handleUpdate = (ctx: any, oldCrew: CrewTemplate, newCrew: CrewTemplate) => {
      console.log('Crew updated:', newCrew);
      setCrews(prev => prev.map(c => c.id === newCrew.id ? newCrew : c));
    };

    const handleDelete = (ctx: any, crew: CrewTemplate) => {
      console.log('Crew deleted:', crew);
      setCrews(prev => prev.filter(c => c.id !== crew.id));
    };

    connection.db.crewTemplate.onInsert(handleInsert);
    connection.db.crewTemplate.onUpdate(handleUpdate);
    connection.db.crewTemplate.onDelete(handleDelete);

    // Reload periodically in case we missed updates
    const interval = setInterval(loadCrews, 3000);

    return () => {
      connection.db.crewTemplate.removeOnInsert(handleInsert);
      connection.db.crewTemplate.removeOnUpdate(handleUpdate);
      connection.db.crewTemplate.removeOnDelete(handleDelete);
      clearInterval(interval);
    };
  }, [connection]);

  const handleUpdateCrew = (templateId: bigint, field: 'name' | 'maxHp' | 'attack' | 'defense' | 'cost', value: any) => {
    if (!connection) return;

    const updates: any = {
      name: field === 'name' ? value : undefined,
      maxHp: field === 'maxHp' ? value : undefined,
      attack: field === 'attack' ? value : undefined,
      defense: field === 'defense' ? value : undefined,
      cost: field === 'cost' ? value : undefined,
    };

    connection.reducers.adminUpdateCrewTemplate(templateId, updates.name, updates.maxHp, updates.attack, updates.defense, updates.cost);
  };

  return (
    <div className="panel">
      <h2>Crew Templates</h2>
      <p className="hint">Click any value to edit. Changes are saved to SpacetimeDB instantly.</p>

      <div className="table-container">
        <table>
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Rarity</th>
              <th>Trait 1</th>
              <th>Trait 2</th>
              <th>Max HP</th>
              <th>Attack</th>
              <th>Defense</th>
              <th>Cost (₿)</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              <tr>
                <td colSpan={9} className="empty">
                  Loading crew templates...
                </td>
              </tr>
            ) : crews.length === 0 ? (
              <tr>
                <td colSpan={9} className="empty">
                  No crew templates found. Make sure data exists in SpacetimeDB.
                </td>
              </tr>
            ) : (
              crews.map(crew => (
                <tr key={crew.id.toString()}>
                  <td>{crew.id.toString()}</td>
                  <td>
                    <EditableCell
                      value={crew.name}
                      onSave={(val) => handleUpdateCrew(crew.id, 'name', val)}
                    />
                  </td>
                  <td>{crew.rarity.tag}</td>
                  <td>{crew.trait1.tag}</td>
                  <td>{crew.trait2?.tag || '-'}</td>
                  <td>
                    <EditableCell
                      value={crew.maxHp}
                      type="number"
                      onSave={(val) => handleUpdateCrew(crew.id, 'maxHp', val)}
                    />
                  </td>
                  <td>
                    <EditableCell
                      value={crew.attack}
                      type="number"
                      onSave={(val) => handleUpdateCrew(crew.id, 'attack', val)}
                    />
                  </td>
                  <td>
                    <EditableCell
                      value={crew.defense}
                      type="number"
                      onSave={(val) => handleUpdateCrew(crew.id, 'defense', val)}
                    />
                  </td>
                  <td>
                    <EditableCell
                      value={crew.cost}
                      type="number"
                      onSave={(val) => handleUpdateCrew(crew.id, 'cost', val)}
                    />
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}

// Item Components Panel
function ItemComponentsPanel({ connection }: { connection: any }) {
  return (
    <div className="panel">
      <h2>Item Components (8 Base Items)</h2>
      <p className="hint">Adjust stat bonuses for each base item component.</p>

      <div className="grid">
        <ItemComponentCard name="Cutlass" stat="AD" value={10} />
        <ItemComponentCard name="Sniper Goggles" stat="Crit %" value={15} />
        <ItemComponentCard name="Shell Dial" stat="AS %" value={15} />
        <ItemComponentCard name="Tone Dial" stat="AP" value={15} />
        <ItemComponentCard name="Seastone Fragment" stat="Armor" value={10} />
        <ItemComponentCard name="Tidal Cloak" stat="MR" value={10} />
        <ItemComponentCard name="Energy Drink" stat="Mana" value={20} />
        <ItemComponentCard name="Meat" stat="HP" value={50} />
      </div>
    </div>
  );
}

function ItemComponentCard({ name, stat, value }: { name: string; stat: string; value: number }) {
  const [editing, setEditing] = useState(false);
  const [val, setVal] = useState(value);

  return (
    <div className="item-card">
      <h3>{name}</h3>
      <div className="stat">
        <span className="stat-label">{stat}:</span>
        {editing ? (
          <input
            type="number"
            value={val}
            onChange={(e) => setVal(Number(e.target.value))}
            onBlur={() => {
              setEditing(false);
              console.log('Update', name, stat, val);
            }}
            autoFocus
          />
        ) : (
          <span className="stat-value" onClick={() => setEditing(true)}>
            {val}
          </span>
        )}
      </div>
    </div>
  );
}

// Completed Items Panel
function CompletedItemsPanel({ connection }: { connection: any }) {
  return (
    <div className="panel">
      <h2>Completed Items (15 Combinations)</h2>
      <p className="hint">Edit final item stats including special effects like splash damage, crit damage multipliers, etc.</p>

      <div className="completed-items">
        <p>Completed items list will appear here...</p>
      </div>
    </div>
  );
}

// Editable Cell Component
function EditableCell({
  value,
  type = 'text',
  onSave,
}: {
  value: any;
  type?: 'text' | 'number';
  onSave: (value: any) => void;
}) {
  const [editing, setEditing] = useState(false);
  const [val, setVal] = useState(value);

  const handleSave = () => {
    setEditing(false);
    if (val !== value) {
      onSave(type === 'number' ? Number(val) : val);
    }
  };

  if (editing) {
    return (
      <input
        type={type}
        value={val}
        onChange={(e) => setVal(e.target.value)}
        onBlur={handleSave}
        onKeyDown={(e) => {
          if (e.key === 'Enter') handleSave();
          if (e.key === 'Escape') {
            setVal(value);
            setEditing(false);
          }
        }}
        autoFocus
        className="editable-input"
      />
    );
  }

  return (
    <span className="editable-cell" onClick={() => setEditing(true)}>
      {value}
    </span>
  );
}

export default App;
