# Every tool has 2 things - 
# 1) function
# 2) it's schema 

#import both here in a data structure and import it to the main and call it 

#just to catchup later => the structure is something like =>
    # the name of the file is the name of the tool
    # import that file only do not use " from [tool] import [....] "
    # there are 2 parts in every tool file 
        #schema - description of tool (read by llm)
        #func - llm with directly call this giving params and rest is to be handled by this func
    #both named same in all files 

from . import get_current_time
from . import open_app


TOOL_SCHEMA = [
    get_current_time.schema,
    open_app.schema,
]

TOOL_FUNC = {
    "get_current_time" : get_current_time.func,
    "open_app" : open_app.func,

}