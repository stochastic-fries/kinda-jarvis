import datetime

schema = {
        "type": "function",
        "function":{
            "name":"get_current_time",              #must match func name
            "description": "this gives the current time",# decides when to call
            "parameters" : {
                "type":"object",
                "properties":{},        #inputs
                "required":[],

            }
        }
    }

def func():
    return datetime.datetime.now().strftime("%I:%M %p")

