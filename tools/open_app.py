import subprocess

schema =   {
        "type":"function",
        "function": {
            "name":"open_app",
            "description": "to open a app ",
            "parameters":{
                "app_name":{            #properties of the app name
                    "type":"string",        #type in which the name of app recieved
                    "description":"App to open"     #description of the app name
                }
            },
            "required": ["app_name"], 
        }
    }

def func(app_name):
    subprocess.Popen(app_name, shell=True)
    return f"Opened {app_name}"
