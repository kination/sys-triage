## sys-triage: CLI for 'System Diagnostics' + 'Cleanup'

This is CLI tool, designed for quickly diagnosing and resolving common system resource issues like 'high CPU load', 'excessive disk usage', 'network bottlenecks', and more(WIP).

It has been implemented based on native Rust libraries to improve performance, and it does not rely on external tools like htop or du.

### Features
- Concurrent Scanning: Utilizes tokio for asynchronous file I/O and rayon for parallel CPU-bound filtering.
- YAML Configuration: Define specific thresholds for all monitored resources.
- CPU Health: Identify and terminate processes exceeding CPU usage limits (check/drop).
- Disk Cleanup: Asynchronously scan large directories and delete files above a size threshold, supporting filtering by extension (check/drop).
- Network I/O: Measure and flag network interfaces with high data transfer rates (check).


## Installation and Build
Clone the repository and build the executable in release mode for optimization:

```bash
git clone <repository_url>
cd sys-triage
cargo build --release
```
The executable will be located at ./target/release/triage.

### Configuration (`triage.yaml`)
The tool relies on a YAML configuration file to define thresholds. The default path is `triage.yaml`.

```yaml
cpu:
  threshold_max: 95.0
  threshold_min: 15.0

disk:
  scan_paths:
    - "/var/log"
    - "/tmp"
  size_threshold: "1 GB" 

network:
  threshold_rx: "10 MB" 
  threshold_tx: "10 MB"
```

### Usage
// TODO: Add usage examples

