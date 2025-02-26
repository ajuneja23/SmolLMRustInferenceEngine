import sys
import os

import smollm_pb2, smollm_pb2_grpc

from transformers import AutoTokenizer, AutoModelForCausalLM


import torch
from transformers import AutoTokenizer, AutoModelForCausalLM
import grpc
from datetime import datetime
import os

import logging
import asyncio


log_dir = "./logs"
os.makedirs(log_dir, exist_ok=True)
port = int(os.getenv("INTERNAL_PORT", 8080))
folder_path = "./SmolLM"
log_file = f"{log_dir}/server_{port}_{datetime.now().strftime('%Y%m%d_%H%M%S')}.log"
logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    handlers=[logging.StreamHandler(), logging.FileHandler(log_file)],
)

logger = logging.getLogger("smollm_server")
logger.setLevel(logging.DEBUG)


if not os.path.exists(folder_path):
    os.makedirs(folder_path)
    logger.info("Downloading model")
    tokenizer = AutoTokenizer.from_pretrained("HuggingFaceTB/SmolLM-135M-Instruct")
    model = AutoModelForCausalLM.from_pretrained("HuggingFaceTB/SmolLM-135M-Instruct")
    tokenizer.save_pretrained(folder_path)
    model.save_pretrained(folder_path)
    logger.info("Model saved successfully")
else:

    tokenizer = AutoTokenizer.from_pretrained(folder_path)
    model = AutoModelForCausalLM.from_pretrained(folder_path)
    logger.info("Model loaded successfully")


class SmolLMServicer(smollm_pb2_grpc.smollmServicer):
    def sendReq(self, request, context):
        prompt = request.prompt
        max_tokens = 20
        logging.info(f"Node {port-8079} has received request: {prompt}")
        input_ids = tokenizer.encode(prompt, return_tensors="pt")
        generated_tokens = []

        for _ in range(max_tokens):
            with torch.no_grad():
                outputs = model(input_ids)

                next_token_logits = outputs.logits[:, -1, :]
                next_token_probs = torch.nn.functional.softmax(
                    next_token_logits, dim=-1
                )  # Normalize logits to probabilities
                next_token_id = torch.argmax(next_token_logits, dim=-1)
                input_ids = torch.cat([input_ids, next_token_id.unsqueeze(0)], dim=1)

                token_id = next_token_id.item()
                generated_tokens.append(token_id)
                token_text = tokenizer.decode(token_id)

                print(f"Token {_+1}: ID={token_id}, Text='{token_text}'")
                yield smollm_pb2.SmolLMRes(
                    curToken=token_text,
                    tokenNum=_ + 1,
                    tokenProbability=torch.max(next_token_probs, dim=-1).values.item(),
                )
        yield smollm_pb2.SmolLMRes(
            curToken="DONE STREAMING RESPONSE", tokenNum=_ + 1, tokenProbability=1
        )


async def serve():
    server = grpc.aio.server()
    smollm_pb2_grpc.add_smollmServicer_to_server(SmolLMServicer(), server)
    listen_addr = "[::]:" + str(os.getenv("INTERNAL_PORT", 8080))
    print(f"Listening on {listen_addr}")
    server.add_insecure_port(listen_addr)
    logging.info("Starting server on %s", listen_addr)
    await server.start()
    await server.wait_for_termination()


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    asyncio.run(serve())
