A simple command-line TODO manager program by using tested third-party crates for command-line parsing and error handling.

# Outline the application
It will be a command-line journal app to manage to-do items. 

The program interface handles three simple actions:

1. Add new tasks to a to-do list.
2. Remove completed tasks from that list.
3. Print all the current tasks in that list.

The program will persist our to-do items in some kind of storage. A text file should be good enough to store such kind of data, so we can stick to a file format, such as JSON to encode our information. Hence, we will need to handle saving and retrieving our data from our storage.
