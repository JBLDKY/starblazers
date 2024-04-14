
export const ChatInput = ({ inputRef }: { inputRef: React.RefObject<HTMLInputElement> }) => {
  return (
    <input ref={inputRef} id="chat-input" type="text" className="chat-input bg-[#221569] border-none mt-1 outline-none shadow-none" />
  );
};
