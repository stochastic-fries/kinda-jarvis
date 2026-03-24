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
from .system_stuff import schema as system_schema , get_system_stats
from .bluetooth_control import schema as bt_schema , func as bt_func #dictionary returning funcs
from .file_operations import schema as file_ops_schema , func as file_ops
from .manage_windows import schema as  windows_schema, func as window
from .web_search import schema as web_search_schema , func as search


TOOL_SCHEMA = [
    get_current_time.schema,
    open_app.schema,
    *system_schema,
    *bt_schema,
    *file_ops_schema,
    *windows_schema,
    *web_search_schema

]

TOOL_FUNC = {
    "get_system_stats"          :           get_system_stats,
    "get_current_time"          :           get_current_time.func,
    "open_app"                  :           open_app.func,
    "bluetooth_scan"            :           bt_func['bluetooth_scan'],
    "bluetooth_list"            :           bt_func['bluetooth_list'],
    "bluetooth_connect"         :           bt_func['bluetooth_connect'],
    "bluetooth_disconnect"      :           bt_func["bluetooth_disconnect"],
    "read_file"                 :           file_ops['read_file'],
    "write_file"                :           file_ops['write_file'],
    "list_directory"            :           file_ops['list_directory'],
    "list_active_windows"       :           window['list_active_windows'],
    "switch_workspace"          :           window['switch_workspace'],
    "web_search"                :           search['web_search'],
    "news_search"               :           search['news_search'],
    "product_search"            :           search['product_search']
}