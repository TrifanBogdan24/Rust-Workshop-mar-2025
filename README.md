# 🌐 [Axum Web Servers | Rust Workshop](https://rust.ipworkshop.ro/docs/web_server/)




```sh
cargo run
```

API:
- Task 1 (healhcheck)
    - <http://localhost:8080/>
    - <http://localhost:8080/healthcheck>
- Task 2 (metrics)
    - <http://localhost:8080/metrics>
    - <http://localhost:8080/metrics/system>
    - <http://localhost:8080/metrics/process>
    - <http://localhost:8080/metrics/memory>
    - <http://localhost:8080/metrics/cpu>
    - <http://localhost:8080/metrics/summary>
- Task 3 (Real-Time)
    - <http://localhost:8080/realtime>
    - <http://localhost:8080/realtime/summary>
    - <http://localhost:8080/realtime/system>
    - <http://localhost:8080/realtime/process>
    - <http://localhost:8080/realtime/memory>
    - <http://localhost:8080/realtime/cpu>


Rust Crates:
- [crates.io](https://crates.io/)
- [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)
- [axum](https://docs.rs/sysinfo/latest/sysinfo/?search=axum)