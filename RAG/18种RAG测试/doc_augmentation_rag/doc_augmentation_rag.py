import json
import asyncio
from typing import List, Dict
import aiofiles

from langchain_community.document_loaders import PyMuPDFLoader
from langchain.text_splitter import RecursiveCharacterTextSplitter
from langchain_huggingface import HuggingFaceEmbeddings
from openai import AsyncOpenAI
import re

from openai.types.chat import ChatCompletionMessageParam, ChatCompletionSystemMessageParam, \
    ChatCompletionUserMessageParam
from tqdm import tqdm

from simple_vector_store import SimpleVectorStore


class DocAugmentationRAG:
    def __init__(self):
        self.chat_client = self.get_chat_client()
        self.embedding_client = self.get_embedding_client()

    @staticmethod
    def extract_text_from_pdf(pdf_path):
        """
        读取PDF文件
        :param pdf_path: 文件路径
        :return:
        """
        pdf_loader = PyMuPDFLoader(pdf_path)
        documents = pdf_loader.load()
        all_text = []
        for page in documents:
            text = page.page_content
            all_text.append(text)
        return "".join(all_text)

    @staticmethod
    def chunk_text(text: str, n=500, overlap=100):
        splitter = RecursiveCharacterTextSplitter(
            chunk_size=n,
            chunk_overlap=overlap
        )
        return splitter.split_text(text)

    @staticmethod
    def get_chat_client():
        client = AsyncOpenAI(
            base_url="https://api.deepseek.com",
            api_key="sk-8881d400f0d245ecab7d71a1dc087a56"
        )
        return client

    async def generate_questions(self, text_chunk, num_questions=5, model="deepseek-chat"):
        """
        根据给定的文本块，生成相关的问题。
        :param text_chunk: 给定文本块。
        :param num_questions: 生成的相关问题数量。
        :param model: 使用的大语言模型。
        :return:
        List[str]: 生成的问题列表
        """
        system_prompt = "你是根据文本生成相关问题的专家。创建仅使用提供的文本即可回答的简明问题。专注于关键信息和概念。"
        user_prompt = f"""
        根据以下文本生成 {num_questions} 个问题，这些问题仅能使用以下文本来回答：
         
        {text_chunk}
        
        将你的回答设置为仅带有编号的问题列表，每个问题以 ? 结尾，没有其他任何文本。
        """
        messages: list[ChatCompletionMessageParam] = [
            ChatCompletionSystemMessageParam(role="system", content=system_prompt),
            ChatCompletionUserMessageParam(role="user", content=user_prompt),
        ]
        response = await self.chat_client.chat.completions.create(
            model=model,
            temperature=0.7,
            messages=messages
        )
        questions_text = response.choices[0].message.content
        if questions_text is None:
            return []
        questions = []
        for line in questions_text.split("\n"):
            cleaned_line = re.sub(r'^\d+\.\s*', '', line.strip())
            if cleaned_line and cleaned_line.endswith('?'):
                questions.append(cleaned_line)
        return questions

    @staticmethod
    def get_embedding_client(model="maidalun1020/bce-embedding-base_v1"):
        embedding_model = HuggingFaceEmbeddings(model_name=model)
        return embedding_model

    async def create_embeddings(self, text):
        # 由于HuggingFaceEmbeddings不支持异步，我们在线程池中运行
        loop = asyncio.get_event_loop()
        response_embedding = await loop.run_in_executor(
            None, self.embedding_client.embed_query, text
        )
        return response_embedding

    async def process_chunk_with_questions(self, chunk, chunk_index, questions_per_chunk, vector_store):
        """
        异步处理单个文本块及其生成的问题
        """
        # 创建文本块的嵌入
        chunk_embeddings = await self.create_embeddings(chunk)
        # 如果向量存储存在，则添加文本块
        if vector_store is not None:
            vector_store.add(
                text=chunk,
                embedding=chunk_embeddings,
                metadata={"type": "chunk", "index": chunk_index}
            )
        
        # 生成问题
        questions = await self.generate_questions(chunk, num_questions=questions_per_chunk)
        
        # 并发创建问题的嵌入
        question_tasks = []
        for j, question in enumerate(questions):
            question_embedding = await self.create_embeddings(question)
            question_tasks.append((j, question, question_embedding))
        
        # 等待所有问题嵌入完成
        for j, question, question_embedding in question_tasks:
            if vector_store is not None:
                vector_store.add(
                    text=question,
                    embedding=question_embedding,
                    metadata={"type": "question", "chunk_index": chunk_index, "question_index": j, "original_chunk": chunk}
                )
        
        return {
            'chunk_embedding': chunk_embeddings,
            'questions': questions,
            'question_embeddings': [task for _, _, task in question_tasks],
            'chunk_index': chunk_index
        }

    async def process_document(self, pdf_path, chunk_size=500, chunk_overlap=100, questions_per_chunk=5, max_concurrent=5):
        """
        扩充处理文档
        :param pdf_path: 待处理的文档
        :param chunk_size: 文本分块大小
        :param chunk_overlap: 文本块之间的重叠数量
        :param questions_per_chunk: 生成扩充文本块的问题的数量
        :param max_concurrent: 最大并发处理数量
        :return:
        Tuple[List[str], SimpleVectorStore]: 文本块 与 向量数据库
        """
        docs = self.extract_text_from_pdf(pdf_path)
        text_chunks = self.chunk_text(docs, chunk_size, chunk_overlap)
        vector_store = SimpleVectorStore()
        
        # 创建信号量来控制并发数量
        semaphore = asyncio.Semaphore(max_concurrent)
        
        async def process_chunk_with_semaphore(chunk, chunk_index):
            async with semaphore:
                return await self.process_chunk_with_questions(chunk, chunk_index, questions_per_chunk, vector_store)
        
        # 并发处理所有文本块
        tasks = []
        for i, chunk in enumerate(text_chunks):
            task = process_chunk_with_semaphore(chunk, i)
            tasks.append(task)
        
        # 使用tqdm显示进度
        for i, task in enumerate(tqdm(asyncio.as_completed(tasks), total=len(tasks), desc="Processing Chunks")):
            await task
        
        return text_chunks, vector_store

    async def semantic_search(self, query: str, vector_store: SimpleVectorStore, k=5) -> List[Dict]:
        """
        使用 query 在VectorStore中进行搜索。
        :param query: 用户问题
        :param vector_store: 在这个VectorStore中执行搜索
        :param k: 匹配top K
        :return: List[Dict] 前K个相关Item
        """
        query_embedding = await self.create_embeddings(query)
        return vector_store.similarity_search(query_embedding=query_embedding, k=k)

    @staticmethod
    def prepare_context(search_results):
        """
        根据向量数据库的检索结果组建统一的上下文，用来做最终检索。
        先将所有匹配的文本块放进去，再将所有来自问题的文本块也放进去。
        :param search_results:
        :return:
        """
        # chunk_indices 用来进行chunk过滤
        chunk_indices = set()
        context_chunks = []
        # 将匹配的文本块放到上下文
        for result in search_results:
            if result["metadata"]["type"] == "chunk":
                chunk_indices.add(result["metadata"]["index"])
                context_chunks.append(f"Chunk {result['metadata']['index']}: \n{result['text']}")
        # 将来自问题的文本块也放进去
        for result in search_results:
            if result["metadata"]["type"] == "question":
                chunk_idx = result["metadata"]["chunk_index"]
                if chunk_idx not in chunk_indices:
                    chunk_indices.add(chunk_idx)
                    context_chunks.append(f"Chunk {chunk_idx} (引用于问题 '{result["text"]}'): \n{result['metadata']['original_chunk']}")

        full_context = "\n\n".join(context_chunks)
        return full_context

    async def generate_response(self, query, context, model="deepseek-chat"):
        """
        根据查询和上下文生成最终响应
        :param model:
        :param query: 用户问题
        :param context: 一阶段检索的上下文信息
        :return:
        """
        system_prompt = "你是一个 AI 助手，严格根据给定的上下文进行回答。如果无法直接从提供的上下文中得出答案，请回复：'我没有足够的信息来回答这个问题。'"
        user_prompt = f"""
            Context: 
            {context}
            
            Question: {query}
            
            根据Context下的内容提供答案，简洁准确！
        """
        messages: list[ChatCompletionMessageParam] = [
            ChatCompletionSystemMessageParam(role="system", content=system_prompt),
            ChatCompletionUserMessageParam(role="user", content=user_prompt),
        ]
        response = await self.chat_client.chat.completions.create(
            model=model,
            temperature=0.7,
            messages=messages,
        )
        return response.choices[0].message.content.strip()

    async def evaluate_response(self, query, response, reference_answer, model="deepseek-chat"):
        evaluate_system_prompt = f"""您是一个智能评估系统，负责评估 AI 响应答案与正确答案的匹配度。
    
        将 AI 助手的响应与真实/参考答案进行比较，并根据以下条件进行评估：
        1. 事实正确性 - 回复是否包含准确的信息？
        2. 完整性 - 它是否涵盖了参考文献中的所有重要方面？
        3. 相关性 - 它是否直接解决了问题？
        
        分配一个从 0 到 1 的分数：
        - 1.0：内容和意义完美匹配，如果参考文献中没有相关信息，则应该回复没有相关信息来回答问题
        - 0.8：非常好，有轻微的遗漏/差异
        - 0.6：好，涵盖了要点，但遗漏了一些细节
        - 0.4：部分回答，有重大遗漏
        - 0.2：最少的相关信息
        - 0.0：不正确或不相关
        
        提供您的分数并说明理由。
        """
        evaluation_prompt = f"""
        用户提问: {query}
    
        AI答案:
        {response}
    
        参考答案:
        {reference_answer}
    
        请根据参考答案，为AI的响应打分。
        """
        messages: list[ChatCompletionMessageParam] = [
            ChatCompletionSystemMessageParam(role="system", content=evaluate_system_prompt),
            ChatCompletionUserMessageParam(role="user", content=evaluation_prompt),
        ]
        response = await self.chat_client.chat.completions.create(
            model=model,
            temperature=0.7,
            messages=messages,
        )
        return response.choices[0].message.content

    async def cal_single_data(self, single_data, vector_store):
        """
        计算单数据的评分
        :param single_data: 单个数据
        :param vector_store: 向量数据库
        :return:
        """
        query = single_data["question"]
        reference_answer = single_data["ideal_answer"]
        search_result = await self.semantic_search(query, vector_store, k=5)

        chunk_results = []
        question_results = []
        for result in search_result:
            if result["metadata"]["type"] == "chunk":
                chunk_results.append(result)
            else:
                question_results.append(result)
        
        print("\nQuery:", query)
        print(f"\n Relevant Chunks: ")
        for i, result in enumerate(chunk_results):
            print(f"Context {i + 1} (similarity: {result['similarity']:.4f})")
            print(result["text"][:300] + "...")
            print("========================================")

        print(f"\n Matched Questions: ")
        for i, result in enumerate(question_results):
            print(f"Context {i + 1} (similarity: {result['similarity']:.4f})")
            print(result["text"])
            chunk_idx = result["metadata"]["chunk_index"]
            print(f"From chunk {chunk_idx}")
            print("========================================")

        # 根据一阶段向量数据库检索结果准备上下文
        context = self.prepare_context(search_result)

        # 喂给LLM生成最后的回复
        response_text = await self.generate_response(query, context, model="deepseek-chat")

        print("\nResponse:")
        print(response_text)

        evaluation = await self.evaluate_response(query, response_text, reference_answer, model="deepseek-chat")
        print("\nEvaluation:")
        print(evaluation)
        return evaluation


async def main():
    doc_rag = DocAugmentationRAG()
    pdf_path = "./data/易速鲜花员工手册.pdf"
    _, vector_store = await doc_rag.process_document(pdf_path, chunk_size=500, chunk_overlap=100, questions_per_chunk=3)
    
    async with aiofiles.open('./data/flower_val.json', 'r', encoding='utf-8') as f:
        content = await f.read()
        data = json.loads(content)
    
    all_evaluations = []
    # 并发处理评估数据
    tasks = []
    for single_data in data:
        task = await doc_rag.cal_single_data(single_data, vector_store)
        tasks.append((single_data["question"], task))
    
    # 使用信号量控制并发数量，避免API限制
    semaphore = asyncio.Semaphore(3)  # 限制并发数量
    
    async def process_evaluation_with_semaphore(question, task):
        async with semaphore:
            return {"query": question, "response": task}
    
    evaluation_tasks = [process_evaluation_with_semaphore(question, task) for question, task in tasks]
    
    for task in tqdm(asyncio.as_completed(evaluation_tasks), total=len(evaluation_tasks), desc="Processing Evaluations"):
        result = await task
        all_evaluations.append(result)
    
    async with aiofiles.open('./test/test_doc_augmentation_rag.json', 'w', encoding="utf-8") as f:
        await f.write(json.dumps(all_evaluations, ensure_ascii=False, indent=4))


if __name__ == '__main__':
    asyncio.run(main())


