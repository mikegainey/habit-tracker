======== List of Habits ========
Done today:
1) exercise (1hr elliptical)       (streak:  2)  (last 30 days:  6)  (time: 12:29)
2) take fiber (1 tbsp in water)    (streak:  2)  (last 30 days:  7)  (time: 23:15, 23:19)
6) stretch (15s x 3 reps)          (streak:  2)  (last 30 days:  2)  (time: 11:45)

Not done today:
3) prep drill (Army PRT)           (streak:  1)  (last 30 days:  5)
4) take fish oil                   (streak:  1)  (last 30 days:  1)
5) squats (10 reps, 10 lbs)        (streak:  1)  (last 30 days:  1)
================================

What do you want to do?
1) Mark a habit complete
2) Add a habit
3) Remove a habit
4) Change a habit name
q) Quit this program


This is just a little project (given to me by ChatGPT) to help me relearn Rust for the nth time.

It's a command-line app that tracks daily habits. You can add habits, delete habits, change the name of habits, and mark habits complete. If you do a habit every day, the program will keep track of your streak. It also tells you how many days you've done every habit in the last 30 days.

I used several external crates for the first time: time, serde, colored, and anyhow.

To Do:
- "view history" command: 30-day list or calendar grid view
- variable habit frequency: every nth day, every Mon-Wed-Fri, etc.
- save to a temp file, then, if successful, rename
- sort before dedup
- export to csv
- change last30days to last_30_days
- change let _ = app.remove_habit(index) to use .ok() and add #[allow(unused_must_use)]


Notes:
- without release build optimizations, release binary is 732K
- with optimizations, release binary is 415K
