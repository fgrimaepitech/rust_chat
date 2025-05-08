'use client';

import { useEffect, useState, use } from 'react';
import { useRouter } from 'next/navigation';
import Chat from '../../../components/Chat';
import Navbar from '../../../components/Navbar';

export default function ChannelPage({ params }: { params: Promise<{ id: string }> }) {
  const router = useRouter();
  const [username, setUsername] = useState('');
  const { id } = use(params);

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
      } else {
        router.push('/');
      }
    }
  }, [router]);

  const handleJoinChannel = (channelId: string) => {
    router.push(`/channel/${channelId}`);
  };

  return (
    <div>
      <Navbar onJoinChannel={handleJoinChannel} />
      <Chat channelId={id} username={username || "Anonymous"} />
    </div>
  );
} 