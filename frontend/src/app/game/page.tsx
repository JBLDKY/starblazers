"use client"

import React, { useEffect, useRef } from 'react';
import { SpaceInvadersGame } from "../lib/game/game";
import { GameCanvas } from '../components/GameCanvas';
import { ChatBox } from '../components/ChatBox';

export default function Game() {

  const canvasRef = useRef<HTMLCanvasElement>(null);

  const initCanvas = (canvas: HTMLCanvasElement): HTMLCanvasElement => {
    canvas.width = 1280;
    canvas.height = 800;
    return canvas;
  };

  useEffect(() => {
    if (canvasRef.current) {

      const canvasElement = initCanvas(canvasRef.current);
      const game = new SpaceInvadersGame(canvasElement);
      game.start();
    }
  }, []);

  // return (
  //   <div className="game flex flex-col items-center justify-center h-screen w-screen m-0 p-0">
  //     <GameCanvas />
  //     <ChatBox />
  //   </div>
  // );
  return (
    <div>
      <div className="game">
        <canvas ref={canvasRef} id="game-canvas"></canvas>

        <div className="chat-box" id="chat-box">
          <div className="chat-messages" id="chat-messages"></div>
          <hr id="chat-line" className="chat-line" />
          <input type="text" id="chat-input" className="chat-input" />
        </div>
      </div>
    </div>
  );
}

