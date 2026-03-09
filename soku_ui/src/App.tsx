import { useEffect, useRef, useState } from 'react';
import { SokuClient } from './engine/SokuClient';
import './App.css';

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [client] = useState(() => new SokuClient());
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    // Initialize Wasm
    client.init().then(() => {
      setIsReady(true);
    });
  }, [client]);

  useEffect(() => {
    if (!isReady || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let animationFrameId: number;

    const renderLoop = () => {
      // 1. Tell Rust to update the buffer
      client.update();

      // 2. Get the Zero-Copy Float32Array
      const renderData = client.getRenderData();

      // 3. Clear canvas
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      if (renderData) {
        // Read the dummy data we pushed in Rust:
        // [ID, Type, X, Y, Width, Height]
        // Currently, we only have 1 shape, so 6 floats.
        for (let i = 0; i < renderData.length; i += 6) {
          const id = renderData[i];
          const type = renderData[i + 1];
          const x = renderData[i + 2];
          const y = renderData[i + 3];
          const width = renderData[i + 4];
          const height = renderData[i + 5];

          if (type === 1.0) {
            // Rectangle
            ctx.fillStyle = '#3b82f6'; // Blue
            ctx.fillRect(x, y, width, height);
            
            ctx.strokeStyle = '#1e3a8a';
            ctx.lineWidth = 2;
            ctx.strokeRect(x, y, width, height);

            // Draw ID for debug
            ctx.fillStyle = 'white';
            ctx.font = '16px sans-serif';
            ctx.fillText(`Shape ID: ${id}`, x + 10, y + 20);
          }
        }
      }

      animationFrameId = requestAnimationFrame(renderLoop);
    };

    renderLoop();

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, [isReady, client]);

  return (
    <div style={{ width: '100vw', height: '100vh', display: 'flex', flexDirection: 'column', backgroundColor: '#0f172a' }}>
      <header style={{ padding: '1rem', backgroundColor: '#1e293b', color: 'white', borderBottom: '1px solid #334155' }}>
        <h1 style={{ margin: 0, fontSize: '1.5rem', fontWeight: 'bold' }}>SOKU (即) Engine</h1>
        <p style={{ margin: 0, fontSize: '0.8rem', color: '#94a3b8' }}>Zero-Copy Wasm Bridge Active</p>
      </header>
      
      <main style={{ flex: 1, display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
        {!isReady ? (
          <div style={{ color: 'white' }}>Loading Wasm Engine...</div>
        ) : (
          <canvas 
            ref={canvasRef}
            width={800}
            height={600}
            style={{ 
              backgroundColor: '#1e293b', 
              borderRadius: '8px',
              boxShadow: '0 20px 25px -5px rgb(0 0 0 / 0.5), 0 8px 10px -6px rgb(0 0 0 / 0.5)'
            }}
          />
        )}
      </main>
    </div>
  );
}

export default App;
