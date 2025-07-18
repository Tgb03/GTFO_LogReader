
# GTFO Log Reader

This is a DLL (Dynamic Link Library) that specializez in reading logs fast and efficiently. It is to be used in creating 
programs that deal with reading logs both in real time and only one time and grabbing data from them. The aim is to make
dealing with logs far easier and more chill to work with by abstracting away all their complexity.

# How it works

The way it works is the DLL is essentially a black box that you tell where to look for logs and which functions (from your
own code) to call whenever it finds data that you need.

The functions that are called by the DLL when data is found are called "Callback" functions. I will use this term a lot.

# Features

## General exposed functions

So far 3 functions are exposed. Using these you can read data from the logs in real time.

#### For adding callbacks in your code:

- `pub extern "C" fn add_callback(code: uint8_t, message_type: uint8_t, channel_id: uint32_t, event_callback_ptr: *const c_void)`

This function takes 5 parameters:

1. `code: uint8_t` this represents what type of request this callback will listen to.
    - `1`: Tokenizer, this returns ALL tokens parsed, I recommend using this to see which tokens are being parsed.
    - `2`: RunInfo, this returns ALL the info about runs. Level info, data, door opens, times for each event.
    - `3`: Mapper, this returns ALL the info about the level generation when dropping into a level.
    - `4`: SeedIndexer, this returns ALL the info obtained from the seed indexer. Be aware not all levels are supported and the info may be limited.

2. `message_type: uint8_t` this represents the format of the response you will get from the DLL.
     - `1`: JSON, this returns all the data in Json format.
     - `2`: ${\textsf{\color{red}NOT YET IMPLEMENTED}}$ BITDATA, this returns all the data directly in bitdata format. I recommend this only if you really care about
  performance. Tho, I do think it is overkill in almost every situation.
     - `3`: ${\textsf{\color{red}NOT YET IMPLEMENTED}}$ CSV, this format is a bit special as not all data can be serialized as CSV so it may remove certain information
     - `4`: ${\textsf{\color{red}NOT YET IMPLEMENTED}}$ XML, similar to json if you enjoy dealing with this format more.
  
3. `channel_id: uint32_t` this is the channel id being used. Use this for shutting down a certain callback function. Be aware that each `code` has unique `channel_id`s
so if you create a channel with id `3` and code `1`, in order to shut it down you need to give the correct `code` as well, not just the `channel_id`.

4. `callback_context: *const c_void` this is the context for the function that will be called. It is given back as is to the EventCallBack function.

5. `event_callback_ptr: *const c_void` this is the function that will be called by the DLL and given the data.
This functions needs to be of type:

`pub type EventCallback = extern "C" fn(context: *const c_void, message: *const c_char)`. The first variable is the context for the function call (Can be used for objects or to give special additional information. This is given as is from the moment you created the callback). All the rest of the data is given through the c_char pointer which can then be parsed. This pointer represents essentially an array of 8 bit integers. Make sure you are actually reading the data properly from it.

- `pub extern "C" fn remove_callback(code: uint8_t, channel_id: uint32_t)`

This function takes 2 parameters:

1. `code: uint8_t` this represents the type of request you want to remove
2. `channel_id: uint8_t` this represents the channel id you want to remove

## Live Reading

For reading logs live (aka while the game is open) the DLL exposes 1 function:

- `pub extern "C" fn start_listener(file_path: *const c_char)`

This function takes as parameter a single string that represents the folder where GTFO is currently generating logs.
Calling this function will start a listener which awaits for logs and then parses them whenever necessary.
Calling it again a second time or however many times after will modify where the log folder it reads from is. 

Currently the exact behaviour for which log is being read is: The latest log file in the project is opened and then parsed 
from the start. This means that upon using this function the DLL will immediately start spitting out information from 
this file, even if the file itself may be super old.

If the folder path does not exist the thread will do nothing. 

## Reading specific files

This will contain a function that allows you to pass the path to the file which will then be read.
This function is given files to be parsed and a callback handler that will be called every time. 

`pub extern "C" fn process_paths(paths: *const *const c_char, len: uint_32, code: uint_8, message_type: uint_8, callback_context: *const c_void, event_callback_ptr: *const c_void)`

This function takes 6 parameters, the first 2 are new, while the last 3 are familiar from the `add_callback` function:

1. `paths: *const *const c_char` this represents the vector of paths of files that need to be parsed.

2. `len: uint_32` this represents the size of the paths array. Make sure this is correct as the app will just have undefined behaviour otherwise.

3. `code: uint8_t` this represents what type of request this callback will listen to.
    - `1`: Tokenizer, this returns ALL tokens parsed, I recommend using this to see which tokens are being parsed.
    - `2`: RunInfo, this returns ALL the info about runs. Level info, data, door opens, times for each event.
    - `3`: Mapper, this returns ALL the info about the level generation when dropping into a level.
    - `4`: SeedIndexer, this returns ALL the info obtained from the seed indexer. Be aware not all levels are supported and the info may be limited.

4. `message_type: uint8_t` this represents the format of the response you will get from the DLL.
     - `1`: JSON, this returns all the data in Json format.
     - `2`: ${\textsf{\color{red}NOT YET IMPLEMENTED}}$ BITDATA, this returns all the data directly in bitdata format. I recommend this only if you really care about
  performance. Tho, I do think it is overkill in almost every situation.
     - `3`: ${\textsf{\color{red}NOT YET IMPLEMENTED}}$ CSV, this format is a bit special as not all data can be serialized as CSV so it may remove certain information
     - `4`: ${\textsf{\color{red}NOT YET IMPLEMENTED}}$ XML, similar to json if you enjoy dealing with this format more.

5. `callback_context: *const c_void` this is the context for the function that will be called. It is given back as is to the EventCallBack function.

6. `event_callback_ptr: *const c_void` this is the function that will be called by the DLL and given the data.
This functions needs to be of type:

`pub type EventCallback = extern "C" fn(context: *const c_void, message: *const c_char)`. The first variable is the context for the function call (Can be used for objects or to give special additional information. This is given as is from the moment you created the process request). All the rest of the data is given through the c_char pointer which can then be parsed. This pointer represents essentially an array of 8 bit integers. Make sure you are actually reading the data properly from it.


# Examples

In this git repo there is a folder `examples` that contains 3 python files that currently work with the DLL to make different things. Keep in mind that each python file is not 
made that greatly as it is intended to only work as example rather than actual applications you can use, this means you may need to modify them heavily if you wish to use them.

# Be aware 
- Certain mods may modify where the logs are being generated or if they are generated. This needs to be accounted for.
