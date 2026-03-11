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
        // Read the packed data from Rust
        // Stride is 6 floats: [ID, TypeVal, X, Y, Width/Radius, Height/Padding]
        for (let i = 0; i < renderData.length; i += 6) {
          const id = renderData[i];
          const typeVal = renderData[i + 1];
          const x = renderData[i + 2];
          const y = renderData[i + 3];
          const w_r = renderData[i + 4];
          const h_pad = renderData[i + 5];

          // Unpack type and flags
          const type = Math.floor(typeVal);
          const isHovered = (typeVal % 1.0) >= 0.09 && (typeVal % 1.0) < 0.15;
          const isSelected = (typeVal % 1.0) >= 0.19;

          // Dynamic colors based on ID
          const colors = ['#3b82f6', '#ec4899', '#10b981', '#f59e0b', '#8b5cf6'];
          const color = colors[Math.floor(id) % colors.length] || '#3b82f6';
          
          ctx.fillStyle = color;
          ctx.strokeStyle = isSelected ? '#ffffff' : (isHovered ? '#94a3b8' : '#475569');
          ctx.lineWidth = isSelected ? 5 : (isHovered ? 3 : 1.5);

          if (type === 1) {
            // Rectangle
            ctx.beginPath();
            ctx.rect(x, y, w_r, h_pad);
            ctx.fill();
            ctx.stroke();
          } else if (type === 2) {
            // Circle
            ctx.beginPath();
            ctx.arc(x, y, w_r, 0, 2 * Math.PI);
            ctx.fill();
            ctx.stroke();
          }
        }
      }

      animationFrameId = requestAnimationFrame(renderLoop);
    };

    const handleMouseMove = (e: MouseEvent) => {
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      client.handleMouseMove(x, y);
    };

    const handleMouseDown = () => {
      client.handleMouseDown();
    };

    canvas.addEventListener('mousemove', handleMouseMove);
    canvas.addEventListener('mousedown', handleMouseDown);

    renderLoop();

    return () => {
      cancelAnimationFrame(animationFrameId);
      canvas.removeEventListener('mousemove', handleMouseMove);
      canvas.removeEventListener('mousedown', handleMouseDown);
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
