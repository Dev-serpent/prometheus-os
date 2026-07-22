"""System SDK - Access Prometheus OS system resources."""

import subprocess


class System:
    """Interface to Prometheus OS system resources."""

    def execute_command(self, command: str) -> str:
        """Execute a shell command and return output."""
        try:
            result = subprocess.run(
                command, shell=True, capture_output=True, text=True, timeout=30
            )
            return result.stdout
        except subprocess.TimeoutExpired:
            return "Command timed out"

    def memory_info(self) -> dict:
        """Get memory usage information."""
        with open("/proc/meminfo") as f:
            lines = f.readlines()
        info = {}
        for line in lines:
            parts = line.split(":")
            if len(parts) == 2:
                info[parts[0].strip()] = parts[1].strip()
        return info

    def cpu_usage(self) -> float:
        """Get current CPU usage percentage."""
        with open("/proc/stat") as f:
            line = f.readline()
        values = [int(x) for x in line.split()[1:]]
        total = sum(values)
        idle = values[3]
        return 100.0 * (1.0 - idle / total) if total > 0 else 0.0
