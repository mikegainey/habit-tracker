// This module handles file I/O.
// Its job:
// save the whole app state to disk
// load the whole app state from disk
// parse text file lines into data
// convert data into lines to write
// This is where you will practice:
// std::fs
// read_to_string
// opening/writing files
// parsing strings
// handling malformed input
// This module should not ask the user anything.

// You should probably have functions whose purposes are:
// save the app to a path
// load the app from a path
// convert one habit into one text line
// parse one habit from one text line
// convert one journal entry into one text line
// parse one journal entry from one text line

// Test things like:
// serialize one habit line
// parse one valid line
// reject one malformed line
// round-trip: save then parse back into the same data
