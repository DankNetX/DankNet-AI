from transformers import GPT4ForCausalLM, AutoTokenizer

class DankTextGenerator:
    def __init__(self, model_id: str = "danknet/gpt4-meme-tuned"):
        self.model = GPT4ForCausalLM.from_pretrained(model_id)
        self.tokenizer = AutoTokenizer.from_pretrained(model_id)
        self.chaos_factor = 0.69  # Updated from on-chain data

    def generate_edgy_text(self, seed: str) -> str:
        inputs = self.tokenizer(
            f"Generate a crypto meme about: {seed} [CHAOS-LEVEL:{self.chaos_factor}]",
            return_tensors="pt"
        )
        outputs = self.model.generate(
            inputs.input_ids,
            max_length=50,
            temperature=self.chaos_factor * 2,
            do_sample=True
        )
        return self.tokenizer.decode(outputs[0], skip_special_tokens=True)
