'use client';
import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import Navbar from '../components/Navbar';

export default function Home() {
  const router = useRouter();
  const [username, setUsername] = useState('');

  useEffect(() => {
    // Get username from localStorage or prompt for it
    const savedUsername = localStorage.getItem('username');
    if (savedUsername) {
      setUsername(savedUsername);
    } else {
      const newUsername = prompt('Please enter your username:');
      if (newUsername) {
        localStorage.setItem('username', newUsername);
        setUsername(newUsername);
      }
    }
  }, []);

  const handleJoinChannel = (channelId: string) => {
    router.push(`/channel/${channelId}`);
  };

  if (!username) {
    return null; // or a loading spinner
  }

  return (
    <main>
      <Navbar onJoinChannel={handleJoinChannel} />
      <div style={{ 
        display: 'flex', 
        justifyContent: 'center', 
        alignItems: 'center', 
        height: 'calc(100vh - 64px)',
        marginLeft: '250px',
        padding: '20px'
      }}>
        <div style={{ textAlign: 'center' }}>
          <h1>Welcome to Chat App</h1>
          <p>Join an existing channel or create a new one to start chatting!</p>
        </div>
      </div>
    </main>
  );
} 