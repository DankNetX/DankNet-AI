import { useConnection, useWallet } from '@solana/wallet-adapter-react';

export const MemeFactory = () => {
  const { publicKey } = useWallet();
  const [chaosLevel, setChaosLevel] = useState(0.69);

  const generateMeme = async () => {
    const response = await fetch('/api/generate-meme', {
      method: 'POST',
      body: JSON.stringify({
        wallet: publicKey?.toBase58(),
        chaos: chaosLevel
      })
    });
    
    const { imageCID, text } = await response.json();
    // Render meme and trigger burn
  };

  return (
    <div className="dank-container">
      <ChaosSlider value={chaosLevel} onChange={setChaosLevel} />
      <button onClick={generateMeme} className="glowing-btn">
        Generate Dankness
      </button>
    </div>
  );
};
