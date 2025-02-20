# Load model directly
from transformers import AutoTokenizer, AutoModelForCausalLM

import os
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM

# Define the folder path
folder_path = "./SmolLM"

# Check if the folder exists
if not os.path.exists(folder_path):
    # If not, create the folder
    os.makedirs(folder_path)
    # Load the model and tokenizer directly
    tokenizer = AutoTokenizer.from_pretrained("HuggingFaceTB/SmolLM-135M-Instruct")
    model = AutoModelForCausalLM.from_pretrained("HuggingFaceTB/SmolLM-135M-Instruct")
    # Save the model and tokenizer to the folder
    tokenizer.save_pretrained(folder_path)
    model.save_pretrained(folder_path)
else:
    # If the folder exists, load the model and tokenizer from the folder
    tokenizer = AutoTokenizer.from_pretrained(folder_path)
    model = AutoModelForCausalLM.from_pretrained(folder_path)

# Stream inference on a random prompt
prompt = "What is the meaning of life?"
max_tokens = 20
input_ids = tokenizer.encode(prompt, return_tensors="pt")
generated_tokens = []

# Generate tokens one by one
for _ in range(max_tokens):
    with torch.no_grad():  # Disable gradient calculation for inference
        outputs = model(input_ids)

    next_token_logits = outputs.logits[:, -1, :]
    next_token_id = torch.argmax(next_token_logits, dim=-1)
    input_ids = torch.cat([input_ids, next_token_id.unsqueeze(0)], dim=1)

    token_id = next_token_id.item()
    generated_tokens.append(token_id)
    token_text = tokenizer.decode(token_id)
    next_token_probs = torch.nn.functional.softmax(next_token_logits, dim=-1)
    print(torch.max(next_token_probs, dim=-1).values.item())

    print(f"Token {_+1}: ID={token_id}, Text='{token_text}'")
# Print the full generated text
complete_text = tokenizer.decode(input_ids[0], skip_special_tokens=True)
print("\nComplete generated text:")
print(complete_text)
