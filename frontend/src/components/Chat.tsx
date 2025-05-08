'use client';

import { useState, useEffect, useRef } from 'react';
import { postMessage, getMessages } from '../../pages/api/chat';
import styles from './Chat.module.css';
import { Message } from './Message';
import { FaPaperclip, FaGlobe, FaRegCommentDots, FaEye, FaEllipsisH, FaArrowAltCircleRight } from 'react-icons/fa';

interface ChatProps {
  channelId: string;
  username: string;
}

interface Message {
  id: string;
  channel: string;
  sender: string;
  content: string;
  timestamp: number;
}

export default function Chat({ channelId, username }: ChatProps) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [newMessage, setNewMessage] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const messagesContainerRef = useRef<HTMLDivElement>(null);
  const messagesRef = useRef<Message[]>([]);
  const [shouldScroll, setShouldScroll] = useState(false);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    messagesRef.current = messages;
  }, [messages]);

  useEffect(() => {
    if (shouldScroll) {
      scrollToBottom();
      setShouldScroll(false);
    }
  }, [messages, shouldScroll]);

  useEffect(() => {
    const loadMessages = async () => {
      try {
        const loadedMessages = await getMessages(channelId);
        const reversedMessages = loadedMessages.reverse();

        if ((messagesRef.current?.length || 0) === 0) {
          setMessages(reversedMessages);
          setShouldScroll(true);
        } else if (reversedMessages.length > (messagesRef.current?.length || 0)) {
          setMessages(reversedMessages);
          setShouldScroll(true);
        } else {
          setMessages(reversedMessages);
        }
      } catch (error) {
        console.error('Error loading messages:', error);
      }
    };

    loadMessages();
    const interval = setInterval(loadMessages, 5000);
    return () => clearInterval(interval);
  }, [channelId]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newMessage.trim()) return;

    try {
      const message = await postMessage({
        channel: channelId,
        sender: username,
        content: newMessage.trim(),
      });
      
      // Add new message at the end of the array
      setMessages(prev => [...prev, message]);
      setNewMessage('');
      setShouldScroll(true);
    } catch (error) {
      console.error('Error sending message:', error);
    }
  };

  return (
    <div className={styles.chatContainer}>
      <div 
        ref={messagesContainerRef}
        className={styles.messagesContainer}
      >
        {messages.map((message) => (
          <Message 
            key={message.id}
            isOwner={message.sender === username}
            sender={message.sender}
            content={message.content}
            timestamp={message.timestamp}
          />
        ))}
        <div ref={messagesEndRef} />
      </div>

      <form onSubmit={handleSubmit} className={styles.inputForm}>
        <div className={styles.inputBar}>
          <input
            type="text"
            value={newMessage}
            onChange={(e) => setNewMessage(e.target.value)}
            placeholder="Ask me anything..."
            className={styles.inputModern}
          />
          <span className={styles.iconRight}><FaArrowAltCircleRight /></span>
        </div>
      </form>
    </div>
  );
}