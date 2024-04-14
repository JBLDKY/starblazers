// In src/components/GameCanvas.tsx
import React from 'react';

const GameCanvas = React.forwardRef<HTMLCanvasElement>((_, ref) => {
  return (
    <canvas ref={ref} className="bg-[#221569]" id="game-canvas"></canvas>
  );
});

GameCanvas.displayName = 'GameCanvas?';

export default GameCanvas;
