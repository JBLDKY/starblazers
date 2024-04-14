
import React, { useRef } from 'react';

export const ChatInput = (inputRef: HTMLInputElement) => {
  return (
    <input ref={inputRef} type="text" className="chat-input bg-[#221569] border-none mt-1 outline-none shadow-none" />

  );
};
