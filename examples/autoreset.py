

import ctypes
import os
import json
from pathlib import Path
from ctypes import c_char_p, c_void_p, c_uint8, c_uint32, CFUNCTYPE
from tkinter import *
from tkinter import ttk
from ahk import AHK
import time

#
# THIS IS ALL SETUP FOR THE DLL
# You could throw all this in a python class
# This will remain static.
#

# Get absolute path to the DLL relative to this script
script_dir = Path(__file__).resolve().parent
dll_path = os.path.join(script_dir, "../target/release/gtfo_log_reader_core.dll")
  # Replace with actual DLL name
lib = ctypes.CDLL(dll_path)

# 2. Define callback type: extern "C" fn(message: *const c_char)
CALLBACK_TYPE = CFUNCTYPE(None, ctypes.c_char_p)

# 3. Define Rust function signatures

# void start_listener(const char* file_path)
lib.start_listener.argtypes = [c_char_p]
lib.start_listener.restype = None

# void add_callback(uint8_t code, uint8_t message_type, uint32_t channel_id, void* callback)
lib.add_callback.argtypes = [c_uint8, c_uint8, c_uint32, c_void_p]
lib.add_callback.restype = None

# void remove_callback(uint32_t channel_id)
lib.remove_callback.argtypes = [c_uint32]
lib.remove_callback.restype = None

#
# THIS IS WHERE THE ACTUAL CODE STARTS
# From here u can modify the code and do whatever u want with it.
#

labels = []

root = Tk()
root.geometry("300x200")
root.title("Simple GUI for Seeds")
root.attributes('-topmost', True)

frame = ttk.Frame(root, padding=10)
frame.pack()

reset_counter = 0
reset_counter_label = Label(frame, text=f"Reset counter: {reset_counter}")
reset_counter_label.pack()

key_id = 0
hsu_id = 0

is_valid = False

ahk = AHK(version='v2', executable_path='C:\Program Files\AutoHotkey\\v2\\AutoHotkey64.exe')

# 4. Implement a Python callback function
# The callback returns a message that is based on the values
# u set when the callback is created by add_callback(...)
@CALLBACK_TYPE
def my_event_callback(message):
    global reset_counter
    global reset_counter_label
    global key_id, hsu_id, is_valid

    if message:
        data = json.loads(message)
        # print(data)

        if data == "GenerationStart":
            for label in labels:
                label.destroy()

            reset_counter += 1
            
            reset_counter_label.destroy()
            reset_counter_label = Label(frame, text=f"Reset counter: {reset_counter}")
            reset_counter_label.pack()

        if "Key" in data:
            name, zone, id = data["Key"]
            text = f"{name} in ZONE_{zone} at {id}"
            label = Label(frame, text=text)
            label.pack()
            labels.append(label)

            if name == 'Key':
                key_id = id

            if name == 'HSU':
                hsu_id = id

        if data == "GenerationEnd":
            print("Stop?: ", key_id, hsu_id, check_stop())
            if check_stop() is False and is_valid is True:
                cycle_reset()

def start_cycling():
    global is_valid
    is_valid = True
    cycle_reset()


def cycle_reset():
    ahk.run_script(r"""
        BlockInput("MouseMove")

        positions := [
            { x: -2000, y: -2000, pre: 100, click: 1, delay: 1 },
            { x: 300,   y: 43,    pre: 200, click: 100, delay: 200 },
            { x: 573,   y: 248,   pre: 200, click: 100, delay: 200 },
            { x: -25,   y: 400,   pre: 200, click: 900, delay: 200 }
        ]

        for index, pos in positions {
            DllCall("mouse_event", "UInt", 0x0001, "Int", pos.x, "Int", pos.y, "UInt", 0, "UPtr", 0)
            Sleep(pos.pre)

            DllCall("mouse_event", "UInt", 0x0002, "UInt", 0, "UInt", 0, "UInt", 0, "UPtr", 0)
            Sleep(pos.click)
            DllCall("mouse_event", "UInt", 0x0004, "UInt", 0, "UInt", 0, "UInt", 0, "UPtr", 0)

            Sleep(pos.delay)
        }

        BlockInput("MouseMoveOff")
        """)


def check_stop():
    if key_id in [0, 1, 2, 3]:
        return True
    
    stop_keys = [14, 16, 18]
    stop_hsus = [1]

    if key_id in stop_keys and hsu_id in stop_hsus:
        return True
    
    return False


ahk.add_hotkey('#n', callback=start_cycling)
ahk.start_hotkeys()

userprofile = os.getenv('USERPROFILE')
local_low = os.path.join(userprofile, 'AppData', 'LocalLow', '10 Chambers Collective', 'GTFO')
local_low = str(local_low)

# Start the listener thread
lib.start_listener(local_low.encode('utf-8'))

# Add a callback with dummy values
code = 4          # e.g., SubscribeCode::Tokenizer
msg_type = 1      # e.g., SubscriptionType::JSON
channel_id = 1    # your app-defined channel ID
callback_fn_ptr = ctypes.cast(my_event_callback, c_void_p)

lib.add_callback(code, msg_type, channel_id, callback_fn_ptr)

root.mainloop()