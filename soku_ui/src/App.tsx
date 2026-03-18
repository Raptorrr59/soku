import { useEffect, useRef, useState } from 'react';
import { SokuClient } from './engine/SokuClient';
import './App.css';

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const renderLoopRunning = useRef(false);
  const [client] = useState(() => new SokuClient());
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    console.log("App mounting, initializing SokuClient...");
    client.init().then(() => {
      console.log("SokuClient ready.");
      setIsReady(true);
    });
  }, [client]);

  useEffect(() => {
    if (!isReady || !canvasRef.current || renderLoopRunning.current) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    renderLoopRunning.current = true;
    let animationFrameId: number;

    const renderLoop = () => {
      if (!renderLoopRunning.current) return;

      try {
        client.update();
        const renderData = client.getRenderData();
        const camera = client.getCamera();

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Apply Camera Transform
        ctx.save();
        ctx.scale(camera.zoom, camera.zoom);
        ctx.translate(-camera.x, -camera.y);

        if (renderData) {
          for (let i = 0; i < renderData.length; i += 6) {
            if (i + 5 >= renderData.length) break;

            const id = renderData[i];
            const typeVal = renderData[i + 1];
            const x = renderData[i + 2];
            const y = renderData[i + 3];
            const w_r = renderData[i + 4];
            const h_pad = renderData[i + 5];

            const type = Math.floor(typeVal);
            const isHovered = (typeVal % 1.0) >= 0.09 && (typeVal % 1.0) < 0.15;
            const isSelected = (typeVal % 1.0) >= 0.19;

            const colors = ['#3b82f6', '#ec4899', '#10b981', '#f59e0b', '#8b5cf6'];
            const color = colors[Math.floor(id) % colors.length] || '#3b82f6';
            
            ctx.fillStyle = color;
            ctx.strokeStyle = isSelected ? '#ffffff' : (isHovered ? '#94a3b8' : '#475569');
            ctx.lineWidth = isSelected ? 5 : (isHovered ? 3 : 1.5);

            if (type === 1) {
              ctx.beginPath();
              ctx.rect(x, y, w_r, h_pad);
              ctx.fill();
              ctx.stroke();
            } else if (type === 2) {
              ctx.beginPath();
              ctx.arc(x, y, w_r, 0, 2 * Math.PI);
              ctx.fill();
              ctx.stroke();
            }
          }
        }
        ctx.restore();
      } catch (err) {
        console.error("Render loop error:", err);
        renderLoopRunning.current = false;
        return;
      }

      animationFrameId = requestAnimationFrame(renderLoop);
    };

    const handleMouseDown = (e: MouseEvent) => {
      if (e.button === 0) client.handleMouseDown();
    };
    const handleMouseUp = () => client.handleMouseUp();

    const handleWheel = (e: WheelEvent) => {
      e.preventDefault();
      const delta = e.deltaY > 0 ? 0.9 : 1.1;
      client.zoomCamera(delta);
    };

    const handleMouseMove = (e: MouseEvent) => {
      const rect = canvas.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      
      if (e.buttons & 4) { // Middle mouse button
        const camera = client.getCamera();
        client.moveCamera(-e.movementX / camera.zoom, -e.movementY / camera.zoom);
      } else {
        client.handleMouseMove(x, y);
      }
    };

    canvas.addEventListener('mousemove', handleMouseMove);
    canvas.addEventListener('mousedown', handleMouseDown);
    canvas.addEventListener('wheel', handleWheel, { passive: false });
    window.addEventListener('mouseup', handleMouseUp);

    renderLoop();

    return () => {
      renderLoopRunning.current = false;
      cancelAnimationFrame(animationFrameId);
      canvas.removeEventListener('mousemove', handleMouseMove);
      canvas.removeEventListener('mousedown', handleMouseDown);
      canvas.removeEventListener('wheel', handleWheel);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isReady, client]);

  return (
    <div className="app-container">
      <header className="app-header">
        <h1 className="app-title">SOKU (即) Engine</h1>
        <p className="app-subtitle">Zero-Copy Wasm Bridge Active</p>
      </header>
      
      <main className="app-main">
        {!isReady ? (
          <div className="loading-text">Loading Wasm Engine...</div>
        ) : (
          <div className="canvas-wrapper">
            <div className="toolbar">
              <button 
                className="tool-btn" 
                onClick={() => {
                  const camera = client.getCamera();
                  client.spawnShape(1, (50 / camera.zoom) + camera.x, (50 / camera.zoom) + camera.y);
                }}
                title="Add Rectangle"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/></svg>
              </button>
              <button 
                className="tool-btn" 
                onClick={() => {
                  const camera = client.getCamera();
                  client.spawnShape(2, (100 / camera.zoom) + camera.x, (100 / camera.zoom) + camera.y);
                }}
                title="Add Circle"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><circle cx="12" cy="12" r="10"/></svg>
              </button>
              <div className="tool-divider" />
              <button 
                className="tool-btn delete-btn" 
                onClick={() => client.deleteSelected()}
                title="Delete Selected"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>
              </button>
            </div>
            <canvas 
              ref={canvasRef}
              width={800}
              height={600}
              className="main-canvas"
            />
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
