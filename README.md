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


This is just a little project (given to me by ChatGPT) to help me learn Rust.

It's a command-line app that tracks daily habits. You can mark a habit complete, update a progress note, add habits,change the name of habits, and . If you do a habit every day, the program will keep track of your streak. It also tells you how many days you've done every habit in the last 30 days.

I used several external crates for the first time: time, serde, colored, and anyhow.

To Do:
- save to a temp file, then, if successful, rename
- variable habit frequency: every nth day, every Mon-Wed-Fri, etc.


Notes:
- without release build optimizations, release binary is 732K
- with optimizations, release binary is 415K
