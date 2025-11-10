

import ctypes
import os
import json
from pathlib import Path
from ctypes import POINTER, c_char_p, c_void_p, c_uint8, c_uint32, CFUNCTYPE
from tkinter import Tk, filedialog

Tk().withdraw()  # Hide the root window

dll_relative_path = "../target/release/glr_dylib.dll"
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

# void process_paths(paths: *const *const c_char, len: uint_32, code: uint_8, message_type: uint_8, context: *const c_void, event_callback_ptr: *const c_void)
lib.process_paths.argtypes = [POINTER(c_char_p), c_uint32, c_uint8, c_uint8, c_void_p, c_void_p]
lib.process_paths.restype = None

#
# THIS IS WHERE THE ACTUAL CODE STARTS
# From here u can modify the code and do whatever u want with it.
#

count = 0

# 4. Implement a Python callback function
# The callback returns a message that is based on the values
# u set when the callback is created by add_callback(...)
@CALLBACK_TYPE
def my_event_callback(context, message):
    global count
    
    data = json.loads(message)
    
    if "SelectExpedition" in data: 
        level = data["SelectExpedition"][0]
        if level["rundown"] == "R1" and level["tier"] == 0 and level["level"] == 0:
            count += 1

file_paths = filedialog.askopenfilenames(title="Select files to process")
encoded_paths = [path.encode('utf-8') for path in file_paths]
c_paths = (c_char_p * len(encoded_paths))(*encoded_paths)

# Add a callback with dummy values
callback_fn_ptr = ctypes.cast(my_event_callback, c_void_p)
length = c_uint32(len(encoded_paths))
code = 1          # e.g., SubscribeCode::Tokenizer
msg_type = 1      # e.g., SubscriptionType::JSON

lib.process_paths(c_paths, length, code, msg_type, 0, callback_fn_ptr)

print(f"Done: {count}")
