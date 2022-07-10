# Caching
Sometimes a calculation needs to be performed multiple times with the same input.  If this calculation takes a long time, this can be expensive for the performance of the application.  One solution is to cache the return value from function calls, so that these can be returned directly on subsequent calls.

This is implemented here in Rust.
