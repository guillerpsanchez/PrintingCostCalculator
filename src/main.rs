use eframe::egui;

mod calculator;
use calculator::cost::CostCalculator;

/// Main application state
struct CalculatorApp {
    calculator: CostCalculator,
    print_time: String,
    filament_amount: String,
    filament_cost: String,
    printer_wattage: String,
    energy_cost: String,
}

impl CalculatorApp {
    fn new() -> Self {
        Self {
            calculator: CostCalculator::new(),
            print_time: String::new(),
            filament_amount: String::new(),
            filament_cost: String::new(),
            printer_wattage: String::new(), // Ahora está vacío por defecto
            energy_cost: String::new(),
        }
    }
    
    /// Parse a string to f64, returning a default value if parsing fails
    fn parse_f64(&self, value: &str, default: f64) -> f64 {
        value.parse().unwrap_or(default)
    }
    
    /// Calculate all costs based on current values
    fn calculate_costs(&self) -> (f64, f64, f64) {
        let pt = self.parse_f64(&self.print_time, 0.0);
        let fa = self.parse_f64(&self.filament_amount, 0.0);
        let fc = self.parse_f64(&self.filament_cost, 0.0);
        let pw = self.parse_f64(&self.printer_wattage, 200.0);
        let ec = self.parse_f64(&self.energy_cost, 0.0);
        
        let material_cost = self.calculator.calculate_filament_cost(fa, fc);
        let electricity_cost = self.calculator.calculate_energy_cost(pt, pw, ec);
        let total_cost = material_cost + electricity_cost;
        
        (total_cost, material_cost, electricity_cost)
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints for responsive updates
        ctx.request_repaint();
        
        // Calculate costs with current values
        let (total_cost, material_cost, electricity_cost) = self.calculate_costs();
        let any_input = !self.print_time.is_empty() || 
                       !self.filament_amount.is_empty() || 
                       !self.filament_cost.is_empty() || 
                       !self.energy_cost.is_empty();
                       
        egui::CentralPanel::default().show(ctx, |ui| {
            // Centrar el título y hacerlo más grande
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                let title = egui::RichText::new("3D Printing Cost Calculator")
                    .size(28.0)
                    .strong();
                ui.heading(title);
            });
            
            // Aumentar la separación vertical
            ui.add_space(30.0);
            
            // Centrar el grupo de datos de entrada
            ui.vertical_centered(|ui| {
                egui::Frame::group(ui.style())
                    .show(ui, |ui| {
                        ui.set_width(400.0); // Establecer un ancho fijo para el grupo
                        ui.vertical_centered(|ui| {
                            ui.heading("Print Data");
                        });
                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label("Print Time (hours):");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                let width = 120.0;
                                ui.add(egui::TextEdit::singleline(&mut self.print_time).desired_width(width));
                            });
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label("Filament Used (kg):");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                let width = 120.0;
                                ui.add(egui::TextEdit::singleline(&mut self.filament_amount).desired_width(width));
                            });
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label("Filament Cost (€/kg):");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                let width = 120.0;
                                ui.add(egui::TextEdit::singleline(&mut self.filament_cost).desired_width(width));
                            });
                        });
                        ui.add_space(15.0);
                        ui.vertical_centered(|ui| {
                            ui.heading("Electricity Data");
                        });
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label("Printer Power (W):");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                let width = 120.0;
                                ui.add(egui::TextEdit::singleline(&mut self.printer_wattage).desired_width(width));
                            });
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label("Electricity Cost (€/kWh):");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(10.0);
                                let width = 120.0;
                                ui.add(egui::TextEdit::singleline(&mut self.energy_cost).desired_width(width));
                            });
                        });
                        ui.add_space(5.0);
                    });
            });
            
            // Aumentar la separación vertical para los resultados
            ui.add_space(30.0);
            
            // Mostrar los resultados en un cuadro dedicado
            if total_cost > 0.0 || any_input {
                ui.vertical_centered(|ui| {
                    egui::Frame::group(ui.style())
                        .show(ui, |ui| {
                            ui.set_width(400.0);
                            
                            ui.vertical_centered(|ui| {
                                ui.add_space(10.0);
                                let cost_text = egui::RichText::new(format!("Total Cost: {:.2} €", total_cost))
                                    .size(22.0)
                                    .strong();
                                ui.heading(cost_text);
                                ui.add_space(10.0);
                            });
                            
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.label(format!("Material: {:.2} €", material_cost));
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(10.0);
                                    ui.label(format!("Electricity: {:.2} €", electricity_cost));
                                });
                            });
                            ui.add_space(10.0);
                        });
                });
            } else {
                ui.vertical_centered(|ui| {
                    egui::Frame::group(ui.style())
                        .show(ui, |ui| {
                            ui.set_width(400.0);
                            ui.vertical_centered(|ui| {
                                ui.add_space(10.0);
                                ui.label("Enter your 3D print data to see cost calculation");
                                ui.add_space(10.0);
                            });
                        });
                });
            }
            
            // Añadir espacio al final
            ui.add_space(20.0);
        });
    }
}

mod macos;
use macos::integration::setup_menu;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    // Setup macOS menu
    setup_menu();

    eframe::run_native(
        "3D Printing Cost Calculator",
        options,
        Box::new(|_cc| Box::new(CalculatorApp::new())),
    )
}