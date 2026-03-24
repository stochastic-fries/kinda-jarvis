import psutil
import shutil

schema = [
    {
        "type":'function',
        "function":{
            "name": "get_system_stats",
            "description": "Get RAM usage, CPU usage, disk space, and battery level. Call this when the user asks about ram, memory, cpu, processor, disk, storage, or battery.",
            "parameters": {
                "type": "object",
                "properties": {},
                "required": [],
            },
        }
    },

]

def get_system_stats() :
    vm = psutil.virtual_memory()
    total, used, _ = shutil.disk_usage("/")
    batt = psutil.sensors_battery()

    return {
        "cpu_percent": psutil.cpu_percent(interval=0.5),
        "ram_used_gb": round(vm.used / 1e9, 2),
        "ram_total_gb": round(vm.total / 1e9, 2),
        "ram_percent": vm.percent,
        "disk_used_gb": round(used / 1e9, 2),
        "disk_total_gb": round(total / 1e9, 2),
        "disk_percent": round(used / total * 100, 1),
        "battery_percent": round(batt.percent, 1) if batt else None,
        "battery_charging": batt.power_plugged if batt else None,
    }