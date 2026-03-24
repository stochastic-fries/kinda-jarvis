from ddgs import DDGS 

schema = [
    {   
        "type":'function',
        "function":{
            "name":'web_search',
            "description": ' Search the internet for guides , mannuals, tips, solutions, explainations or informations',
            "parameters":{
                "type":'object',
                "properties":{
                    "query":{
                        "type":'string',
                        "description":'the search query',
                    },
                },
                "required":['query']
            }
        },

    },
    {
        "type":'function',
        "function":{
            "name":'news_search',
            "description":'Search for latest news and events on any topic',
            "parameters":{
                "type":'object',
                "properties":{
                    "query":{
                        "type":'string',
                        "description":'the topic related to which news is to be gathered',
                    }
                },
                "required":['query'],
            }
        }
    },
    {
        "type":'function',
        "function":{
            "name":'product_search',
            "description":'To search for products on the internet, use this when the user asks a price or wants to buy something',
            "parameters":{
                "properties":{
                    "query":{
                        "type":'string',
                        "description":'about the product to be searched',
                    }
                },
                "required":['query'],
            },
        }
    },
]
def web_search(query:str):
    print("searching ..... => ",query)        #trust ssues
    with DDGS() as ddgs:
        results = list(ddgs.text(query,max_results=10))
    if not results:
        return "no results found"
    return '\n\n'.join(x['body'] for x in results)

def news_search(query:str):
    print("searching ..... => ",query)        #trust issues

    with DDGS() as ddgs:
        results = list(ddgs.news(query, max_results=10))
    if not results:
        return "no results found"
    chunks = []     #chunks of news
    for i in results:
        chunks.append(f"[{i['source']}] {i['title']}\n {i['body']}")
    return "\n\n".join(chunks)

def product_search(query:str):
    query = query+" shopping"
    print("searching ..... => ",query)        #trust issues
    with DDGS() as ddgs:
        results = list(ddgs.text(query , max_results=10))
    if not results:
        return "no results found"
    return '\n\n'.join(x['body'] for x in results)          

func = {
    "web_search":web_search,
    "news_search":news_search,
    "product_search":product_search,
}