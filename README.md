# Dyndns

A powerful, lightweight Dynamic DNS updater written in Rust. This tool automatically updates your dynamic DNS records based on your current network configuration and IP address.

## Features

- Automatically fetches your public IP address and updates DNS records.
- Lightweight and efficient, built in Rust for performance and reliability.
- Structured configuration using systemd service and timer files.
- Cross-platform support (Linux primarily).
- Easy-to-customize and extend.

## Installation

### Build from Source

1. Install Rust and Cargo by following the instructions at [Rust's official site](https://www.rust-lang.org/tools/install).
2. Clone the repository:
   ```bash
   git clone https://github.com/your_username/dyndns.git
   cd dyndns
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Copy the generated executable:
   ```bash
   cp target/release/dyndns /usr/local/bin
   ```

### Debian Package (if applicable)

If you have created a Debian package, you can install it like so:
```bash
sudo dpkg -i dyndns_0.1.0_amd64.deb
```

## Usage

Dyndns can be run as a standalone binary or in conjunction with systemd for automated execution.

### Run Manually
You can run the application directly:
```bash
dyndns --config /path/to/config/file
```

### Systemd Integration
This project includes ready-made systemd service and timer files.

1. Copy the timer and service files to the appropriate directory:
   ```bash
   sudo cp linux/dyndns@.service /etc/systemd/system/
   sudo cp linux/dyndns@.timer /etc/systemd/system/
   ```
2. Enable and start the timer:
   ```bash
   sudo systemctl enable dyndns@your_configuration.timer
   sudo systemctl start dyndns@your_configuration.timer
   ```

This will periodically check and update your DNS records as per your specified schedule.

### Command-Line Options

This application uses `structopt` for parsing command-line options. To view all available options, run:
```bash
dyndns --help
```

The tool leverages `serde` and `serde_json` for efficient parsing of configuration files.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Author

Created by [Nuno](mailto:nuno@nunum.me).

## Acknowledgments

Special thanks to the Rust community for providing powerful libraries that make projects like this possible.