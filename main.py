import ollama
import subprocess

from tools import TOOL_SCHEMA, TOOL_FUNC

SYSTEM = {"role": "system", "content": "You are a helpful and friendly desktop assistant. Be concise."}


#func that runs the model
def run(user_input, history):
    history.append({"role": "user", "content": user_input})

    while True:
        response = ollama.chat(
            model = "qwen2.5:7b-instruct",
            messages=history,  # this contains current prompt with the previous ones
            tools=TOOL_SCHEMA   # defined above 
        )
        message = response["message"]

        if  not message.get("tool_calls"):
            history.append({"role":"assistant","content": message["content"]})
            return message["content"], history

        
        history.append(message) # this step is req. even if we get tool calls in message

        for tool_call in message["tool_calls"]:
            name = tool_call["function"]["name"]
            args = tool_call["function"]["arguments"]
            func   = TOOL_FUNC[name]
            result = func(**args)
            # call the matching Python function
            if name == "get_time":
                result = get_time()
            elif name == "open_app":
                result = open_app(**args)


            history.append({"role": "tool", "content": result})


def main ():
    history = []

    while True:
        user_input = input("You: ")
        reply, history = run(user_input, history)
        print(f"Assistant: {reply}")


if __name__ == "__main__":
    main()