# 3D Printing Cost Calculator

This project is a 3D printing cost calculator tool designed for macOS. It provides a graphical user interface (GUI) for users to input various parameters related to 3D printing costs, including print time, filament usage, filament cost, and electricity costs.

## Features

- Input fields for:
  - Total print time (hours)
  - Total filament used (kg)
  - Cost per kilogram of filament (€/kg)
  - Printer power consumption (W)
  - Electricity cost (€/kWh)
- Real-time cost calculation as you type, without needing to press a button
- Shows breakdown of material and electricity costs
- User-friendly graphical interface with centered, well-spaced layout
- Native macOS integration

## Project Structure

```
rust-3d-printing-calculator
├── src
│   ├── main.rs          # Entry point of the application and UI implementation
│   ├── calculator        # Module for cost calculation functionality
│   │   ├── mod.rs       # Exports calculator functions and types
│   │   └── cost.rs      # Defines cost calculation methods
│   └── macos            # macOS-specific functionality
│       ├── mod.rs       # Exports macOS integration functions
│       └── integration.rs # Implementation of macOS features
├── resources
│   └── icons
│       └── app_icon.icns # Application icon
├── Cargo.toml           # Project configuration and dependencies
├── build.rs             # Custom build script
└── README.md            # Project documentation
```

## Setup Instructions

1. Clone the repository:
   ```
   git clone <repository-url>
   cd rust-3d-printing-calculator
   ```

2. Ensure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).

3. Build the project:
   ```
   cargo build
   ```

4. Run the application:
   ```
   cargo run
   ```

## Usage

- Open the application and enter the required parameters in the input fields.
- The cost calculation updates in real-time as you enter data.
- View the total cost breakdown showing both material and electricity costs.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.