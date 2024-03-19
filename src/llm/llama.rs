use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

struct LlamaInstance{
    model_name: String, 
    model_url: String,
    model_port: u16,
    instance: Ollama
}

trait LlamaFunctions{
    fn new(&self, model_name: String, model_url: String, model_port: u16) -> Self;
    fn generate_response(&self, prompt_text: String) -> String;
}

impl LlamaFunctions for LlamaInstance{
    fn new(&self, model_name: String, model_url: String, model_port: u16) -> Self{
        let ollama = Ollama::new(model_url.clone(), model_port);
        LlamaInstance{
            model_name,
            model_url,
            model_port,
            instance: ollama
        }
    }
    pub async fn generate_response(&self, prompt_text: String) -> Result<String, String> {
        let res = self.instance.generate(GenerationRequest::new(self.model_name.clone(), prompt_text)).await;
        if let Ok(res) = res {
            return Ok(res.response);
        }else{
            return Err("Unable to generate any response".to_string());
        }
    }
}

pub async fn init_llama2(){
    let model = "llama2:latest".to_string();
    let prompt = "Why is the sky blue?".to_string();

    let ollama = Ollama::new("http://0.0.0.0".to_string(), 11434);
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }
}
