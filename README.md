
# GTFO Log Reader

This is a DLL (Dynamic Link Library) that specializez in reading logs fast and efficiently. It is to be used in creating 
programs that deal with reading logs both in real time and only one time and grabbing data from them. The aim is to make
dealing with logs far easier and more chill to work with by abstracting away all their complexity.

# Examples

In this git repo there is a folder `examples` that contains 6 python files that currently work with the DLL to make different things. Keep in mind that each python file is not 
made that greatly as it is intended to only work as example rather than actual applications you can use, this means you may need to modify them if you wish to use them.

### How to use examples:

1. Download Python.
2. Download the latest release of the DLL. (On the right in the Github Page)
3. Download the script you want to use.
4. Put the script and DLL in the same folder and open the python script to modify the field `dll_relative_path` to match where the DLL is relative to the python script.
5. Open a CMD and run `python <filename>`
6. Python may say that you are missing a library in which case you need to run `pip install <library name>`. After that rerun the script.

> [!NOTE]
> `autoreset.py` uses the AutoHotKey application which can be downloaded here: https://www.autohotkey.com/

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

- `pub extern "C" fn shutdown_all()`

This function works to shutdown everything. Keep in mind that also clears all the callback data which means you need to recreate every single callback.

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


# What each part returns

### 1. Tokenizer

This represents all the tokens that are being recorded by the tool. Generally this data is not really that useful unless you are looking to track some very specific thing like player count.

```rust
enum Token {

    PlayerJoinedLobby,
    UserExitLobby,
    PlayerLeftLobby,
    SessionSeed(u64),

    GeneratingLevel,
    GeneratingFinished,
    ItemAllocated(KeyDescriptor),                     // name
    ItemSpawn(u64, u32),                              // zone, id
    CollectableAllocated(u64),                        // zone
    ObjectiveSpawnedOverride(u64, ObjectiveFunction), // id, name of objective
    CollectableItemID(u8),                            // item id
    CollectableItemSeed(u64),                         // item seed
    DimensionIncrease,
    DimensionReset,
    SelectExpedition(LevelDescriptor, i32),           // level info and seed
    GameStarting,
    GameStarted,
    PlayerDroppedInLevel(u32),
    DoorOpen,
    CheckpointReset,
    BulkheadScanDone,
    SecondaryDone,
    OverloadDone,
    GameEndWin,
    GameEndLost,
    GameEndAbort,
    LogFileEnd,

    Invalid,
}
```

### 2. RunInfo

This represents all the data that helps out with runs. 

```rust
enum RunGeneratorResult {

    GameStarted(LevelDescriptor, u8),   // level started, player count
    SplitAdded(NamedSplit),             // split containing time and name

    SecondaryDone,
    OverloadDone,
    CheckpointUsed,

    LevelRun(TimedRun<NamedSplit>),     // full level run obtained
}
```


### 3. Mapper

This represents all the items (keys, objective items) found from the logs directly. You can see exactly which items are obtained in the comments

```rust
enum Location {
    // name, zone, id
    ColoredKey(String, u64, u64),
    BulkheadKey(String, u64, u64),

    // gatherable identifier, zone, id
    Gatherable(ItemIdentifier, u64, u64),

    // hsu/terminal/other: name, zone and XX_area
    BigObjective(String, u64, u64),

    // big collectables (cryo, cargos etc.): only identifier and zone
    BigCollectable(ItemIdentifier, u64),

    // generation started
    GenerationStarted(String),
}
```

### 4. SeedIndexer

How we get that information is by checking the seed of the level and then seeing what UnityRandom generates and then interpreting that information based on what we know about the level generation. Because of this, new data may be added later based on what new information we find.

To be noted is that historically the SeedIndexer application developed years ago had the ability to show resource locations. However this application does not yet have this feature. (You can see that the ResourcePack value only contains the type and count, not the actual amount)

> [!IMPORTANT]
> The `Key(String, i32, i32)` option shows: Colored Keys, Bulkhead Keys, HSUs, Collectable Objective Items (IDs, GLPs, etc...).

```rust
enum OutputSeedIndexer {
    Seed(f32),
    Key(String, i32, i32),           // zone, id
    ResourcePack(ResourceType, i32), // count
    ConsumableFound(i32, bool),      // id of box, found or not
    GenerationEnd,
    GenerationStart,
    ZoneGenEnded(u32),
}
```

#### Implemented levels:
- Rundown 1: **FULLY DONE**
- Rundown 2: **FULLY DONE**
- Rundown 3: **FULLY DONE**
- Rundown 4: **FULLY DONE**
- Rundown 5: **FULLY DONE**
- Rundown 6: **FULLY DONE**
-  8 / 10 Rundown 7: R7A1 R7B1 R7B2 R7B3 R7C2 R7C3 R7D1 R7E1
-  4 / 12 Rundown 8: R8A1 R8B1 R8B3 R8E2

#### Contributing:
The way SeedIndexing works is by taking the seed of the level, generating the values the game will then use to build the map (which are 32 bit floating point values) and then checks where items will be spawned based on those values.

In order to add a level to Seed Indexer you need to use the debug version of the `gtfo_log_reader.dll`. This version instead of having the data included in the binary, it loads a file located in `resources/level_descriptors.json`. You also need to download this file and put it in the resources folder that is in the same folder as the DLL and Python Script. (If you messed up the DLL should output the exact error). After you got it to run, you can modify `level_descriptors.json` to produce whichever result you want. 

Generally the way the game works is: It throws out the first 5 values, then it generates all the keys in main and maybe a generator in between keys or after, then the zones along with stuff the zones contain such as cells (tied to zone, not cells tied to door), generators (maybe?, not sure yet) etc. After that the objective of the layer gets generated. Once everything from this layer is done it goes to the secondary, repeats the entire process and then overload. This does mean indeed that finding out keys for a level to reset faster is mostly trivial and should be VERY quick, while mapping objective items can take a whole day. A few things to note is each resource value you see in the ZoneConsumer is the datablock value, `consumable_in_container` means consumables that spawn in boxes/lockers while `consumable_in_worldspawn` is those that spawn on the ground. Make sure these are correct. I strongly recommend using [Kenny's Spreadsheets](https://docs.google.com/spreadsheets/d/1b_dDH7WG8pmAOGPToUE2XSHAkzfie2HzdxVcO3aNK4c) as he noted down a lot of the information being used and made it easier to access.

> [!WARNING]
> Remember that a lot of the info we have is inferred and we can't really say how exactly the level generation works as a lot of it is unkown so there is a lot of guesswork in figuring out a level.

# Be aware 
- Certain mods may modify where the logs are being generated or if they are generated. This needs to be accounted for.
