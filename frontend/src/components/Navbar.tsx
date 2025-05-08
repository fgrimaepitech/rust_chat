'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import styles from './Navbar.module.css';
import { createChannel, joinChannel } from '../../pages/api/chat';

interface NavbarProps {
  onJoinChannel: (channelId: string) => void;
}

export default function Navbar({ onJoinChannel }: NavbarProps) {
  const router = useRouter();
  const [channelId, setChannelId] = useState('');
  const [channelName, setChannelName] = useState('');
  const [isCreating, setIsCreating] = useState(false);

  const handleJoinSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (channelId.trim()) {
      try {
        await joinChannel(channelId.trim());
        router.push(`/channel/${channelId.trim()}`);
        setChannelId('');
      } catch (error) {
        console.error('Error joining channel:', error);
      }
    }
  };

  const handleCreateSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (channelName.trim()) {
      try {
        const channel = await createChannel(channelName.trim());
        router.push(`/channel/${channel.id}`);
        setChannelName('');
        setIsCreating(false);
      } catch (error) {
        console.error('Error creating channel:', error);
      }
    }
  };

  return (
    <nav className={styles.navbar}>
      <div className={styles.logo}>
        <h1>Chat App</h1>
      </div>
      
      {!isCreating ? (
        <>
          <form onSubmit={handleJoinSubmit} className={styles.joinForm}>
            <input
              type="text"
              value={channelId}
              onChange={(e) => setChannelId(e.target.value)}
              placeholder="Enter channel ID"
              className={styles.input}
            />
            <button type="submit" className={styles.button}>
              Join Channel
            </button>
          </form>
          <button 
            onClick={() => setIsCreating(true)} 
            className={`${styles.button} ${styles.createButton}`}
          >
            Create New Channel
          </button>
        </>
      ) : (
        <form onSubmit={handleCreateSubmit} className={styles.joinForm}>
          <input
            type="text"
            value={channelName}
            onChange={(e) => setChannelName(e.target.value)}
            placeholder="Enter channel name"
            className={styles.input}
          />
          <div className={styles.buttonGroup}>
            <button type="submit" className={styles.button}>
              Create & Join
            </button>
            <button 
              type="button" 
              onClick={() => setIsCreating(false)}
              className={`${styles.button} ${styles.cancelButton}`}
            >
              Cancel
            </button>
          </div>
        </form>
      )}
    </nav>
  );
} 