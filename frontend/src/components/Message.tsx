import styles from './Message.module.css';

type MessageProps = {
  isOwner: boolean;
  sender: string;
  content: string;
  timestamp: number;
}

export const Message: React.FC<MessageProps> = ({ isOwner, sender, content, timestamp }) => {
  return (
    <div className={styles.messageWrapper}>
      <div className={styles.messageHeader}>
        <span className={styles.sender}>{sender}</span>
        <span className={styles.timestamp}>{new Date(timestamp * 1000).toLocaleTimeString()}</span>
      </div>
      <div className={isOwner ? styles.messageContentSent : styles.messageContentReceived}>{content}</div>
    </div>
  )
};