# TODO

- Publication artifact v1 implements bounded strict JSON decoding and exact service, destination and table-profile binding. Fence evidence uses `sha256:<64 lowercase hex digits>` references, endpoints exclude paths/userinfo/query/fragment, and diagnostics redact untrusted fields. Seven focused local tests passed. Runtime digest pinning, complete restore, publisher integration and deployment acceptance remain downstream gates.
