/// A calculator for 3D printing costs
#[derive(Default)]
pub struct CostCalculator {}

impl CostCalculator {
    /// Creates a new CostCalculator instance
    pub fn new() -> Self {
        Self {}
    }

    /// Calculates the cost of filament used
    ///
    /// # Arguments
    ///
    /// * `filament_used` - Amount of filament used in kilograms
    /// * `cost_per_kg` - Cost of filament per kilogram in currency units
    pub fn calculate_filament_cost(&self, filament_used: f64, cost_per_kg: f64) -> f64 {
        filament_used * cost_per_kg
    }

    /// Calculates the cost of electricity consumed during printing
    ///
    /// # Arguments
    ///
    /// * `print_time` - Print time in hours
    /// * `printer_wattage` - Printer power consumption in watts
    /// * `energy_cost_per_kwh` - Cost of electricity per kilowatt-hour
    pub fn calculate_energy_cost(&self, print_time: f64, printer_wattage: f64, energy_cost_per_kwh: f64) -> f64 {
        let total_energy_used = printer_wattage * print_time; // in watt-hours
        let total_energy_used_kwh = total_energy_used / 1000.0; // convert to kWh
        total_energy_used_kwh * energy_cost_per_kwh
    }

    /// Calculates the total cost of a 3D print
    ///
    /// # Arguments
    ///
    /// * `print_time` - Print time in hours
    /// * `filament_used` - Amount of filament used in kilograms
    /// * `cost_per_kg` - Cost of filament per kilogram
    /// * `printer_wattage` - Printer power consumption in watts
    /// * `energy_cost_per_kwh` - Cost of electricity per kilowatt-hour
    pub fn calculate_total_cost(
        &self, 
        print_time: f64, 
        filament_used: f64, 
        cost_per_kg: f64, 
        printer_wattage: f64, 
        energy_cost_per_kwh: f64
    ) -> f64 {
        self.calculate_filament_cost(filament_used, cost_per_kg) + 
        self.calculate_energy_cost(print_time, printer_wattage, energy_cost_per_kwh)
    }
}