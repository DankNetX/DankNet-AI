import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { DanknetSDK } from 'danknet-web3-sdk';

export const DankDashboard = () => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [chaosLevel, setChaosLevel] = useState(0);
  const [memeQueue, setMemeQueue] = useState<Meme[]>([]);

  useEffect(() => {
    const loadChaos = async () => {
      const chaos = await DanknetSDK.getChaosLevel(connection);
      setChaosLevel(chaos);
    };
    
    loadChaos();
    const interval = setInterval(loadChaos, 15000);
    return () => clearInterval(interval);
  }, [connection]);

  const generateMeme = async () => {
    const newMeme = await DanknetSDK.generateMeme(
      publicKey!,
      `Chaos Level: ${chaosLevel.toFixed(2)}`
    );
    
    setMemeQueue(prev => [newMeme, ...prev.slice(0, 4)]);
    
    // Trigger burn transaction
    await DanknetSDK.executeBurn(
      publicKey!,
      newMeme.chaosScore * 1000
    );
  };

  return (
    <div className="dank-container">
      <ChaosMeter level={chaosLevel} />
      <button 
        onClick={generateMeme} 
        className={`generate-btn ${chaosLevel > 8 ? 'critical' : ''}`}
      >
        {chaosLevel > 8 ? 'WARNING: MAXIMUM DANK' : 'Generate Meme'}
      </button>
      <MemeCarousel memes={memeQueue} />
    </div>
  );
};
