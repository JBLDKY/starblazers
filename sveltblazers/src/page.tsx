
import React, { useEffect, useRef } from 'react';
import { SpaceInvadersGame } from '../lib/game/game';
import GameCanvas from '../components/GameCanvas';
import { ChatBox } from '../components/ChatBox';

export default function Game() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    if (canvasRef.current) {
      const canvasElement = canvasRef.current;
      canvasElement.width = 1280;
      canvasElement.height = 800;

      const game = new SpaceInvadersGame(canvasElement);
      game.start();
    }
  }, []);

  return (
    <div className="game flex flex-col items-center justify-center h-screen w-screen m-0 p-0">
      <GameCanvas ref={canvasRef} />
      <ChatBox />
    </div>
  );
}
