import React, { useRef } from 'react';
import { ChatInput } from './ChatInput';

export const ChatBox = () => {

  const inputRef = useRef<HTMLInputElement>(null);


  return (
    <div className="chat-box bg-[#221569] w-72 fixed bottom-2 left-2 z-10">
      <div className="chat-messages text-[#b5e6ff] bg-[#221569] h-24 overflow-y-hidden break-words"></div>
      <hr className="chat-line border-t border-black m-0" />
      <ChatInput inputRef={inputRef} />

      // <input ref={inputRef} type="text" className="chat-input bg-[#221569] border-none mt-1 outline-none shadow-none" />
    </div>
  );
};