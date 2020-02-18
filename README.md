# libwmata

Rust library for getting WMATA arrival times.

## Notes

Some deserialized values are implemented as `serde_json::Value` due to the fact that the API sometimes returns a string and other times an integer.
