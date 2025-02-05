export class DanknetSDK {
  constructor(rpcUrl, apiKey) {
    this.connection = new Connection(rpcUrl);
    this.apiKey = apiKey;
  }

  async generateMeme(wallet, seedText) {
    const response = await fetch('https://api.danknet.ai/v1/generate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-API-Key': this.apiKey
      },
      body: JSON.stringify({
        wallet: wallet.publicKey.toString(),
        prompt: seedText,
        chaos_boost: true
      })
    });
    
    return this._handleDankResponse(response);
  }

  async executeBurn(wallet, amount) {
    const transaction = new Transaction().add(
      createBurnInstruction(
        wallet.publicKey,
        amount,
        DANK_TOKEN_PROGRAM_ID
      )
    );
    
    return wallet.sendTransaction(transaction, this.connection);
  }

  async _handleDankResponse(response) {
    if (!response.ok) {
      throw new Error(`DANK Error: ${response.statusText}`);
    }
    
    const data = await response.json();
    
    if (data.chaos_level > 8.5) {
      console.warn('⚠️ Extreme dankness detected! Proceed with caution');
    }
    
    return data;
  }
}
