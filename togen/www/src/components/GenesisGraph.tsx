import { useCallback } from 'react';
import {
  ReactFlow,
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  type Connection,
  BackgroundVariant,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';

// --- Definición de Nodos Iniciales (Arquitectura Genesis) ---
const initialNodes = [
  // Sensores (Inputs)
  { id: 'eye', position: { x: 0, y: 100 }, data: { label: '👁️ Ojo (Sensor)' }, type: 'input', style: { background: '#fef3c7', border: '1px solid #d97706' } },
  { id: 'ear', position: { x: 0, y: 300 }, data: { label: '👂 Oído (Sensor)' }, type: 'input', style: { background: '#fef3c7', border: '1px solid #d97706' } },

  // Cortezas (Procesamiento Primario)
  { id: 'visual_cortex', position: { x: 250, y: 100 }, data: { label: '🎨 Visual Cortex' }, style: { background: '#dbeafe', border: '1px solid #2563eb' } },
  { id: 'auditory_cortex', position: { x: 250, y: 300 }, data: { label: '🎵 Auditory Cortex' }, style: { background: '#dbeafe', border: '1px solid #2563eb' } },

  // Arcos Superiores (Asociación)
  { id: 'broca', position: { x: 500, y: 200 }, data: { label: '🗣️ Broca\'s Arc (Language)' }, style: { background: '#fce7f3', border: '1px solid #db2777', width: 180 } },
  { id: 'hippocampus', position: { x: 500, y: 50 }, data: { label: '🧠 Hippocampus (Memory)' }, style: { background: '#d1fae5', border: '1px solid #059669' } },
  { id: 'amygdala', position: { x: 500, y: 350 }, data: { label: '🔥 Amygdala (Emotion)' }, style: { background: '#fee2e2', border: '1px solid #dc2626' } },

  // Salida
  { id: 'motor', position: { x: 750, y: 200 }, data: { label: '🦾 Motor Output' }, type: 'output', style: { background: '#e5e7eb', border: '1px solid #4b5563' } },
];

// --- Definición de Conexiones (Flujo de Togens) ---
const initialEdges = [
  { id: 'e1-1', source: 'eye', target: 'visual_cortex', animated: true, label: 'Raw Data' },
  { id: 'e1-2', source: 'ear', target: 'auditory_cortex', animated: true, label: 'Raw Audio' },
  
  { id: 'e2-1', source: 'visual_cortex', target: 'broca', animated: true, label: 'Togen Visual' },
  { id: 'e2-2', source: 'auditory_cortex', target: 'broca', animated: true, label: 'Togen Auditivo' },
  
  { id: 'e2-3', source: 'visual_cortex', target: 'hippocampus', animated: false },
  { id: 'e2-4', source: 'visual_cortex', target: 'amygdala', animated: false },
  
  { id: 'e3-1', source: 'broca', target: 'motor', animated: true, label: 'Action' },
];

export default function GenesisGraph() {
  const [nodes, , onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges],
  );

  return (
    <div style={{ width: '100%', height: '80vh', border: '1px solid #ccc', borderRadius: '8px' }}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
      >
        <Controls />
        <MiniMap />
        <Background variant={BackgroundVariant.Dots} gap={12} size={1} />
      </ReactFlow>
    </div>
  );
}
