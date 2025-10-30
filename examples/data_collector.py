

from collections import defaultdict
import csv
import ctypes
import datetime
import os
import json
from pathlib import Path
from ctypes import POINTER, c_char_p, c_void_p, c_uint8, c_uint32, CFUNCTYPE
from tkinter import Tk, filedialog

runs_collected = []

Tk().withdraw()  # Hide the root window

dll_relative_path = "../target/release/glr_dylib.dll"
log_folder_path = str(os.path.join(os.getenv('USERPROFILE'), 'AppData', 'LocalLow', '10 Chambers Collective', 'GTFO'))
output_path = "C:\\Users\\Tudor\\Desktop\\GTFO Marathon 2025\\"

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

# 4. Implement a Python callback function
# The callback returns a message that is based on the values
# u set when the callback is created by add_callback(...)
@CALLBACK_TYPE
def my_event_callback(context, message):
    global runs_collected

    if message:
        data = json.loads(message)
        if "LevelRun" in data:
            runs_collected.append(data["LevelRun"])


file_paths = filedialog.askopenfilenames(title="Select files to process")
encoded_paths = [path.encode('utf-8') for path in file_paths]
c_paths = (c_char_p * len(encoded_paths))(*encoded_paths)

# Add a callback with dummy values
callback_fn_ptr = ctypes.cast(my_event_callback, c_void_p)
length = c_uint32(len(encoded_paths))
code = 2          # e.g., SubscribeCode::Tokenizer
msg_type = 1      # e.g., SubscriptionType::JSON

lib.process_paths(c_paths, length, code, msg_type, 0, callback_fn_ptr)

start_first_stream = datetime.datetime.fromisoformat("2025-10-20T07:41:59.408+00:00")
start_secnd_stream = datetime.datetime.fromisoformat("2025-10-22T07:42:02.347+00:00")

def get_level(level_id):
    name = level_id["name"]
    r_id = name["rundown"]
    tier = name["tier"]
    level = name["level"]
    return f"{r_id}{chr(ord('A') + tier)}{level + 1}"

def get_in_stream_time(time):
    if time < start_secnd_stream:
        return time - start_first_stream
    else:
        return time - start_secnd_stream

sorted_data = sorted(
    runs_collected,
    key=lambda d: datetime.datetime.fromisoformat(d["utc_time_started"].replace("Z", "+00:00")),
    reverse=False
)

get_time = lambda r: datetime.datetime.fromisoformat(r["utc_time_started"].replace("Z", "+00:00"))
sorted_data = [r for i, r in enumerate(sorted_data) if i == 0 or (get_time(r) - get_time(sorted_data[i - 1])).total_seconds() > 1]

utc_start = datetime.datetime.fromisoformat("2025-10-20T08:00:00.000+00:00")

count_wins = sum(1 for entry in sorted_data if entry["is_win"])
count_losses = sum(1 for entry in sorted_data if entry["is_win"] == False)


data_obtained = []
in_game_time_win = datetime.timedelta()
in_game_time_loss = datetime.timedelta()
player_stats = {}
last_run_end = utc_start
beaten_expeditions = set()

for entry in sorted_data:
    players = entry["players"]
    was_win = entry["is_win"]
    level_name = get_level(entry)
    entry["completed_prior"] = level_name in beaten_expeditions
    for name, stats in players.items():
        name = name.lower()
        if name not in player_stats:
            player_stats[name] = {
                "name": name,
                "death_count": 0,
                "total_time": datetime.timedelta(milliseconds=0),
                "levels_done": [],
                "wins": 0,
                "losses": 0,
            }
        player_stats[name]["death_count"] += stats.get("death_count", 0)
        player_stats[name]["total_time"] += datetime.timedelta(milliseconds=entry["total_time"])
        if was_win:
            beaten_expeditions.add(level_name)
            player_stats[name]["levels_done"].append(level_name)
            player_stats[name]["wins"] += 1
        else:
            player_stats[name]["losses"] += 1
    time_start_run = datetime.datetime.fromisoformat(entry["utc_time_started"].replace("Z", "+00:00"))
    if was_win:
        in_game_time_win += datetime.timedelta(milliseconds=entry["total_time"])
    else:
        in_game_time_loss += datetime.timedelta(milliseconds=entry["total_time"])
    end_time = time_start_run + datetime.timedelta(milliseconds=entry["total_time"])
    elapsed = end_time - utc_start
    pause_prior = time_start_run - last_run_end
    pauses_total = elapsed - in_game_time_win - in_game_time_loss
    checkpoint_count = entry["used_checkpoint"]

    win_loss_text = ""
    if was_win:
        win_loss_text = " for win."
    else:
        win_loss_text = " lost."

    print(f"Paused for {pause_prior}")
    print(f"In {level_name} for: {end_time - time_start_run}{win_loss_text} ({checkpoint_count} checkpoints)")
    print("Cleared by: ", end = "")
    for name, stats in sorted(players.items()):
        print(f"{name}, ", end = "")
    # print(f"Time start run: {time_start_run}, IGT win: {in_game_time_win}, IGT loss: {in_game_time_loss}, OOGT: {pauses_total}, level: {level_name}, was_win: {was_win}, end: {end_time}")
    # print(f"{entry}")
    print()
    print()

    last_run_end = end_time

player_stats = sorted(player_stats.items())

print("       Player name        Winrate Wins Losses Deaths       IGT        Levels Done")
for player, data in player_stats:
    winrate = 100 * data["wins"] / (data["wins"] + data["losses"])
    wins = data["wins"]
    losses = data["losses"]
    death_count = data["death_count"]
    total_time = data["total_time"]
    data["levels_done"] = list(dict.fromkeys(data["levels_done"]))
    data["levels_done"] = sorted(data["levels_done"])
    levels_done = data["levels_done"]
    print(f"{player:<25} {winrate:>6.2f}% {wins:^5} {losses:^5} {death_count:^5} {total_time} {levels_done}")

player_stats = [key[1] for key in player_stats]

avg_pause = pauses_total / sorted_data.__len__()

print(f"Number of wins: {count_wins}. IGT: {in_game_time_win}")
print(f"Number of losses: {count_losses}. IGT: {in_game_time_loss}")

print(f"Pauses: {pauses_total}. Avg pause time: {avg_pause}")

max_id = 0

for data in sorted_data:
    data["name"] = get_level(data)
    data["splits"] = len(data["splits"])
    data["utc_time_started"] = datetime.datetime.fromisoformat(data["utc_time_started"].replace("Z", "+00:00"))
    data["utc_time_ended"] = data["utc_time_started"] + datetime.timedelta(milliseconds=entry["total_time"])
    data["total_time"] = datetime.timedelta(milliseconds=data["total_time"])
    data["in_stream_stamp"] = get_in_stream_time(data["utc_time_started"])
    data["in_stream_end_stamp"] = data["in_stream_stamp"] + data["total_time"]

    for id, player in enumerate(sorted(data["players"])):
        #print(id, player)
        data[f"Player {id + 1}"] = player
        max_id = max(id, max_id)
    
    del data["players"]

fieldnames = [
    "name",
    "total_time",
    "used_checkpoint",
    "is_win",
    "did_secondary",
    "did_overload",
    "completed_prior",
    "utc_time_started",
    "utc_time_ended",
    "in_stream_stamp",
    "in_stream_end_stamp",
    "splits",
]

for id in range(0, max_id + 1): 
    fieldnames.append(f"Player {id + 1}")

with open(output_path + "output_runs.csv", "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=fieldnames)
    writer.writeheader()
    writer.writerows(sorted_data)

with open(output_path + "output_players.csv", "w", newline="") as f:
    writer = csv.DictWriter(f, fieldnames=player_stats[0].keys())
    writer.writeheader()
    writer.writerows(player_stats)
