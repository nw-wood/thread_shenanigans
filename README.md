Multithreaded Application Example

This Rust program demonstrates a multithreaded application using Arc and Mutex for shared state management. The application consists of three threads:

    User Input Thread: Reads input from the user via the standard input.
    Server Simulation Thread: Generates random messages periodically.
    Drawing Thread: Periodically prints all collected messages and clears them.
