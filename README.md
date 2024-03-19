# LLMQueue

Just a simple experiment to see if I can get postgres to work as a message queue
and a vector db to run with some open soruce llm :)

## What do you need to run this locally for now?

 - Ollama Docker Image
   ```bash
   docker run -d --gpus=all -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama #(Well if you have a gpu ;))
    # OR
   docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
   ```

## Things added till now

 - A layer over pgmq, just simple functions to add and get data from a queue
 - An interface over Llama as LlamaInstance struct, can generate responses to pormpts

## TBD
 - Pass Messages to more than one instances of Ollama at a time with PGMQ
 - Implement a RAG pipeline too
