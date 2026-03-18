import { useEffect, useRef, useState } from 'react';
import { SokuClient } from './engine/SokuClient';
import './App.css';

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const renderLoopRunning = useRef(false);
  const [client] = useState(() => new SokuClient());
  const [isReady, setIsReady] = useState(false);
  const [fps, setFps] = useState(0);
  const [hasSelection, setHasSelection] = useState(false);

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
    let lastTime = performance.now();
    let frameCount = 0;

    const renderLoop = (time: number) => {
      if (!renderLoopRunning.current) return;

      frameCount++;
      if (time - lastTime >= 1000) {
        setFps(frameCount);
        frameCount = 0;
        lastTime = time;
      }

      try {
        const camera = client.getCamera();
        const minX = camera.x - 100; // Small padding
        const minY = camera.y - 100;
        const maxX = camera.x + (canvas.width / camera.zoom) + 100;
        const maxY = camera.y + (canvas.height / camera.zoom) + 100;

        client.update(minX, minY, maxX, maxY);
        const renderData = client.getRenderData();
        let anySelected = false;

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Apply Camera Transform
        ctx.save();
        ctx.scale(camera.zoom, camera.zoom);
        ctx.translate(-camera.x, -camera.y);

        if (renderData) {
          const colors = ['#3b82f6', '#ec4899', '#10b981', '#f59e0b', '#8b5cf6'];
          
          // 1. Group by Style
          const groups = new Map<number, number[]>();

          for (let i = 0; i < renderData.length; i += 6) {
            const typeVal = renderData[i + 1];
            
            const type = Math.floor(typeVal);
            const isHovered = (typeVal % 1.0) >= 0.09 && (typeVal % 1.0) < 0.15;
            const isSelected = (typeVal % 1.0) >= 0.19;
            const colorIdx = Math.floor((typeVal % 0.1) * 100.5);

            if (isSelected) anySelected = true;

            const styleKey = type | (colorIdx << 4) | (isSelected ? 256 : 0) | (isHovered ? 512 : 0);
            
            let group = groups.get(styleKey);
            if (!group) {
              group = [];
              groups.set(styleKey, group);
            }
            group.push(i);
          }

          // 2. Draw Batches
          const skipStrokes = camera.zoom < 0.2;

          for (const [styleKey, indices] of groups) {
            const type = styleKey & 0xF;
            const colorIdx = (styleKey >> 4) & 0xF;
            const isSelected = (styleKey & 256) !== 0;
            const isHovered = (styleKey & 512) !== 0;

            ctx.fillStyle = colors[colorIdx];
            ctx.strokeStyle = isSelected ? '#ffffff' : (isHovered ? '#94a3b8' : '#475569');
            ctx.lineWidth = isSelected ? 5 : (isHovered ? 3 : 1.5);

            ctx.beginPath();
            
            for (const i of indices) {
              const x = renderData[i + 2];
              const y = renderData[i + 3];
              const w_r = renderData[i + 4];
              const h_pad = renderData[i + 5];

              if (type === 1) { // Rectangle
                ctx.rect(x, y, w_r, h_pad);
              } else if (type === 2) { // Circle
                ctx.moveTo(x + w_r, y);
                ctx.arc(x, y, w_r, 0, 2 * Math.PI);
              } else if (type === 3) { // Triangle
                ctx.moveTo(x, y - h_pad / 2);
                ctx.lineTo(x - w_r / 2, y + h_pad / 2);
                ctx.lineTo(x + w_r / 2, y + h_pad / 2);
                ctx.lineTo(x, y - h_pad / 2);
              } else if (type === 4) { // Polygon
                const sides = Math.floor(h_pad);
                const radius = w_r;
                for (let j = 0; j < sides; j++) {
                  const angle = (j / sides) * 2 * Math.PI - Math.PI / 2;
                  const px = x + radius * Math.cos(angle);
                  const py = y + radius * Math.sin(angle);
                  if (j === 0) ctx.moveTo(px, py);
                  else ctx.lineTo(px, py);
                }
                ctx.closePath();
              }
            }
            
            ctx.fill();
            if (!skipStrokes || isSelected || isHovered) {
              ctx.stroke();
            }
          }
        }
        ctx.restore();
        setHasSelection(anySelected);
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

    animationFrameId = requestAnimationFrame(renderLoop);

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
        <p className="app-subtitle">Zero-Copy Wasm Bridge Active | FPS: {fps}</p>
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
              <button 
                className="tool-btn" 
                onClick={() => {
                  const camera = client.getCamera();
                  client.spawnShape(3, (150 / camera.zoom) + camera.x, (150 / camera.zoom) + camera.y);
                }}
                title="Add Triangle"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><path d="M12 2L2 22h20L12 2z"/></svg>
              </button>
              <button 
                className="tool-btn" 
                onClick={() => {
                  const camera = client.getCamera();
                  client.spawnShape(4, (200 / camera.zoom) + camera.x, (200 / camera.zoom) + camera.y);
                }}
                title="Add Polygon"
              >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5"><path d="M12 2l8.66 5v10L12 22l-8.66-5V7L12 2z"/></svg>
              </button>
              <div className="tool-divider" />
              
              {hasSelection && (
                <>
                  <div className="property-group">
                    {['#3b82f6', '#ec4899', '#10b981', '#f59e0b', '#8b5cf6'].map(color => (
                      <button 
                        key={color} 
                        className="color-swatch" 
                        style={{ backgroundColor: color }}
                        onClick={() => client.updateSelectedColor(color)}
                      />
                    ))}
                  </div>
                  <div className="tool-divider" />
                  <button className="tool-btn" onClick={() => client.resizeSelected(1.1)} title="Scale Up">+</button>
                  <button className="tool-btn" onClick={() => client.resizeSelected(0.9)} title="Scale Down">-</button>
                  <div className="tool-divider" />
                  <button className="tool-btn" onClick={() => client.updateSelectedZIndex(10)} title="Bring Forward">Fwd</button>
                  <button className="tool-btn" onClick={() => client.updateSelectedZIndex(-10)} title="Send Backward">Back</button>
                  <div className="tool-divider" />
                </>
              )}

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
