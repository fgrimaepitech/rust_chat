import axios from 'axios';

// API base URL
const API_BASE_URL = 'http://localhost:8000'; // Adjust this to match your backend URL

// Types matching backend structs
interface ChatMessage {
  id: string;
  channel: string; 
  sender: string;
  content: string;
  timestamp: number;
}

interface Channel {
  id: string;
  name: string;
  created_at: string;
}

interface NewMessage {
  channel: string;
  sender: string;
  content: string;
}

// API functions
export const postMessage = async (message: NewMessage): Promise<ChatMessage> => {
  const response = await axios.post(`${API_BASE_URL}/messages`, message);
  return response.data;
};

export const getMessages = async (channel: string, limit?: number): Promise<ChatMessage[]> => {
  const params = { channel, ...(limit && { limit }) };
  const response = await axios.get(`${API_BASE_URL}/messages`, { params });
  return response.data;
};

export const createChannel = async (name: string): Promise<Channel> => {
  const response = await axios.post(`${API_BASE_URL}/channels`, { name });
  return response.data;
};

export const listChannels = async (): Promise<Channel[]> => {
  const response = await axios.get(`${API_BASE_URL}/channels`);
  return response.data;
};

export const joinChannel = async (channelId: string): Promise<{status: string, channel_id: string}> => {
  const response = await axios.post(`${API_BASE_URL}/channels/${channelId}/join`);
  return response.data;
};
