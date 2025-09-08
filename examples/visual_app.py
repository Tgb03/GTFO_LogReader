

import ctypes
import os
import json
from pathlib import Path
from ctypes import c_char_p, c_void_p, c_uint8, c_uint32, CFUNCTYPE
from tkinter import *
from tkinter import ttk
from collections import Counter
from collections import defaultdict


dll_relative_path = "../target/release/glr_dylib.dll"
# dll_relative_path = "gtfo_log_reader_core_64bit.dll"
log_folder_path = str(os.path.join(os.getenv('USERPROFILE'), 'AppData', 'LocalLow', '10 Chambers Collective', 'GTFO'))

#
# THIS IS ALL SETUP FOR THE DLL
# You could throw all this in a python class
# This will remain static.
#

# Get absolute path to the DLL relative to this script
script_dir = Path(__file__).resolve().parent
dll_path = os.path.join(script_dir, dll_relative_path)
  # Replace with actual DLL name
lib = ctypes.CDLL(dll_path)

# 2. Define callback type: extern "C" fn(context: *const c_void, message: *const c_char)
CALLBACK_TYPE = CFUNCTYPE(None, ctypes.c_void_p, ctypes.c_char_p)

# 3. Define Rust function signatures

# void start_listener(const char* file_path)
lib.start_listener.argtypes = [c_char_p]
lib.start_listener.restype = None

# void add_callback(uint8_t code, uint8_t message_type, uint32_t channel_id, void* context, void* callback)
lib.add_callback.argtypes = [c_uint8, c_uint8, c_uint32, c_void_p, c_void_p]
lib.add_callback.restype = None

# void remove_callback(uint32_t channel_id)
lib.remove_callback.argtypes = [c_uint8, c_uint32]
lib.remove_callback.restype = None

#
# THIS IS WHERE THE ACTUAL CODE STARTS
# From here u can modify the code and do whatever u want with it.
#

labels = []
groups = defaultdict(list)

root = Tk()
root.geometry("300x200")
root.title("Simple GUI for Seeds")
root.attributes('-topmost', True)

frame = ttk.Frame(root, padding=10)
frame.pack()

reset_counter = 0
reset_counter_label = Label(frame, text=f"Reset counter: {reset_counter}")
reset_counter_label.pack()

class NumberCounter:
    def __init__(self):
        self.counts = Counter()
    
    def add(self, number: int):
        """Increment count for a number."""
        self.counts[number] += 1
    
    def get_count(self, number: int) -> int:
        """Get how many times a number has appeared."""
        return self.counts[number]
    
    def reset(self):
        """Reset the counter (clear all counts)."""
        self.counts.clear()

    def __iter__(self):
        """Allow iteration over (number, count) pairs."""
        return iter(sorted(self.counts.items()))

counter = NumberCounter()

# 4. Implement a Python callback function
# The callback returns a message that is based on the values
# u set when the callback is created by add_callback(...)
@CALLBACK_TYPE
def my_event_callback(context, message):
    global reset_counter
    global reset_counter_label, counter

    if message:
        data = json.loads(message)
        # print(message)

        if data == "GenerationStart":
            for label in labels:
                label.destroy()

            groups.clear()
            counter.reset()

            reset_counter += 1
            
            reset_counter_label.destroy()
            reset_counter_label = Label(frame, text=f"Reset counter: {reset_counter}")
            reset_counter_label.pack()

        if "Key" in data:
            name, zone, id = data["Key"]
            if name in ["ConsumableWorldspawn", "ConsumableContainer", "ArtifactWorldspawn"]:
                return
            
            # if name in ["ID", "ConsumableWorldspawn", "ConsumableContainer", "ArtifactWorldspawn", 
            #             "ArtifactContainer", "Ammopack", "Healthpack", "ToolRefillpack", "DisinfectPack"]:
            #     groups[(name, zone)].append(id)
            #     # counter.add(zone)
            #     return

            # if name == "ArtifactWorldspawn":
            #     return

            text = f"{name} in ZONE_{zone} at {id}"
            label = Label(frame, text=text)
            label.pack()
            labels.append(label)

        if "ConsumableFound" in data:
            c_id, found = data["ConsumableFound"]
            text = f"Container {c_id}: {found}"
            label = Label(frame, text=text)
            label.pack()
            labels.append(label)

        if "ResourcePack" in data:
            name, zone, id, size = data["ResourcePack"]
            if name in ["ID", "ConsumableWorldspawn", "ConsumableContainer", "ArtifactWorldspawn", 
                        "ArtifactContainer", "Ammopack", "Healthpack", "ToolRefillpack", "DisinfectPack"]:
                groups[(name, zone)].append(id)
                # counter.add(zone)
                return
            label = Label(frame, text=f"{name} in ZONE_{zone} of size {size} at {id}")
            label.pack()
            labels.append(label)

        if "ZoneGenEnded" in data:
            zone_id = data["ZoneGenEnded"]
            
            label = Label(frame, text=f"Zone {zone_id} done")
            label.pack()
            labels.append(label)

        if data == "Seed":
            print(data)

        if data == "GenerationEnd":
            for (name, zone) in sorted(groups.keys()):
                ids = groups[(name, zone)]
                label = Label(frame, text=f"ZONE_{zone} has {name}: {ids}")
                label.pack()
                labels.append(label)
            for num, count in counter:
                if num in [40, 42, 45]:
                    continue
                
                label = Label(frame, text=f"ZONE_{num} has {count} IDs")
                label.pack()
                labels.append(label)


# Add a callback with dummy values
code = 4          # e.g., SubscribeCode::Tokenizer
msg_type = 1      # e.g., SubscriptionType::JSON
channel_id = 1    # your app-defined channel ID
callback_fn_ptr = ctypes.cast(my_event_callback, c_void_p)

lib.add_callback(code, msg_type, channel_id, 0, callback_fn_ptr)

# Start the listener thread
lib.start_listener(log_folder_path.encode('utf-8'))

root.mainloop()