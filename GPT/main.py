import openai
import os

# 设置OpenAI API密钥



# GPT-3.5模型生成文本
prompt = "The following is a conversation between an interviewer and a job candidate:"
model = "text-davinci-002"
completions = openai.Completion.create(
    engine=model,
    prompt=prompt,
    max_tokens=1024,
    n=1,
    stop=None,
    temperature=0.5,
)

message = completions.choices[0].text.strip()

print(message)




