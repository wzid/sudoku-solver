use eframe::egui::*;

pub mod square;
pub mod solver;
use square::Square;
use solver::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800.0, 625.0)),
        ..Default::default()
    };

    eframe::run_native(
        "sudoku solver",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    theme: catppuccin_egui::Theme,
    grid: Vec<Vec<Square>>,
    error_message: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            theme: catppuccin_egui::MACCHIATO,
            grid: vec![vec![Square::default(); 9]; 9],
            error_message: String::new()
        }
    }
}


trait ThemeName {
    fn get_name(&self) -> String;
}

// This allows me to get a String of the theme easier
impl ThemeName for catppuccin_egui::Theme {
    fn get_name(&self) -> String {
        String::from(match *self {
            catppuccin_egui::FRAPPE => "Frappe",
            catppuccin_egui::MACCHIATO => "Macchiato",
            catppuccin_egui::MOCHA => "Mocha",
            _ => "",
        })
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Set the theme for the UI using the catppuccin crate
        catppuccin_egui::set_theme(ctx, self.theme.clone());

        TopBottomPanel::top("Top panel???").show(ctx, |ui| {
            ui.label(RichText::new("sudoku solver").size(25.0));

            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Theme");
                // ComboBox for the different themes
                ComboBox::from_id_source("Theme")
                    .selected_text(self.theme.get_name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.theme, catppuccin_egui::MACCHIATO, "Macchiato");
                        ui.selectable_value(&mut self.theme, catppuccin_egui::FRAPPE, "Frappe");
                        ui.selectable_value(&mut self.theme, catppuccin_egui::MOCHA, "Mocha");
                    });
                              
                let solve_response = ui.add(
                    Button::new(RichText::new("Solve").color(self.theme.mantle))
                        .fill(self.theme.text)
                        .stroke(Stroke::NONE)
                );
                
                if solve_response.clicked() {
                    // Verify it is a valid sudoku board
                    if verify_grid(&self.grid) {
                        
                        // Set the error message based on the result of `solve_grid`
                        match solve_grid(&mut self.grid) {
                            SolveResult::Unique => {
                                self.error_message = String::new();
                            },
                            SolveResult::Invalid => {
                                self.error_message = "No valid solutions".to_string();
                            },
                            SolveResult::NotUnique => {
                                self.error_message = "Solution is not unique".to_string();
                            },
                        }

                    } else {
                        self.error_message = "Invalid board!".to_string();
                    }
                }

                let reset_response = ui.add(
                    Button::new(RichText::new("Reset").color(self.theme.mantle))
                        .fill(self.theme.text)
                        .stroke(Stroke::NONE)
                );

                // If you just clicked the reset button then it will reset the solved cells
                if reset_response.clicked() {
                    self.error_message = String::new();
                    for i in 0..9 {
                        for j in 0..9 {
                            if self.grid[i][j].solved_cell {
                                self.grid[i][j].solved_cell = false;
                                self.grid[i][j].show_text = false;
                                self.grid[i][j].value = String::new();
                            }
                        }
                    }
                }

                // If you double clicked the response button then reset everything
                if reset_response.double_clicked() {
                    self.error_message = String::new();
                    for i in 0..9 {
                        for j in 0..9 {
                            self.grid[i][j].solved_cell = false;
                            self.grid[i][j].show_text = false;
                            self.grid[i][j].value = String::new();
                        }
                    }
                }
                
                // Put the error message next to the buttons
                if !self.error_message.is_empty() {
                    ui.label(RichText::new(&self.error_message).color(self.theme.red));
                }

                
            });
            
            ui.add_space(5.0);
        });

        CentralPanel::default().show(ctx, |ui| {
            let square_size = 55.0;

            // 9 squares of square size and 3 spaces of 2.0 width
            let adjust = ((9.0 * square_size) + 9.0) / 2.0;

            // This sets the initial position of the grid
            let initial_position = Pos2::new(
                // Using some pretty basic logic we just get the center and subtract by adjust
                ui.available_width() / 2.0 - adjust,
                // Doing the same here but we need to make sure we have the correct starting position
                ui.next_widget_position().y + ui.available_height() / 2.0 - adjust,
            );

            // This is going to be our position that we continually update as we move through the grid
            let mut pos = initial_position;

            //Grid
            ui.horizontal_wrapped(|ui| {
                for i in 0..9 {
                    for j in 0..9 {
                        let rect = Rect::from_two_pos(pos, pos + vec2(square_size, square_size));
                        let response = ui.allocate_rect(rect, Sense::click());

                        // Show the text box if you click on the square
                        if response.clicked() {
                            self.grid[i][j].show_text = true;
                        }

                        ui.painter().rect_filled(
                            rect,
                            0.0,
                            // Checkerboard style color and alt color for solved cells
                            if self.grid[i][j].solved_cell {
                                self.theme.crust
                            }else if (i + j) % 2 == 0 {
                                self.theme.surface1
                            } else {
                                self.theme.surface2
                            },
                        );
                        

                        if self.grid[i][j].show_text || self.grid[i][j].solved_cell {
                            ui.allocate_ui_at_rect(rect, |ui| {
                                // Create TextEdit object
                                let text_edit = TextEdit::singleline(&mut self.grid[i][j].value)
                                    .vertical_align(Align::Center)
                                    .font(FontId::new(30.0, FontFamily::Proportional))
                                    .frame(false)
                                    .horizontal_align(Align::Center);


                                let text_response = ui.centered_and_justified(|ui| ui.add(text_edit)).inner;
                                    
                                if text_response.has_focus() {
                                    // Here we go through and check to see if the user has pressed any of the arrow keys and then change the focus of which cell they are on
                                    if ui.input_mut(|inp| inp.consume_key(Modifiers::NONE, Key::ArrowDown)) {
                                        text_response.surrender_focus();
                                        let row_down = (i + 1) % 9;
                                        self.grid[i][j].focus = false;
                                        self.grid[row_down][j].focus = true;
                                        self.grid[row_down][j].show_text = true;
                                    }

                                    if ui.input_mut(|inp| inp.consume_key(Modifiers::NONE, Key::ArrowUp)) {
                                        text_response.surrender_focus();
                                        // Prevent subtract with overflow error
                                        let row_up = (9 + i - 1) % 9;
                                        self.grid[i][j].focus = false;
                                        self.grid[row_up][j].focus = true;
                                        self.grid[row_up][j].show_text = true;
                                    }

                                    if ui.input_mut(|inp| inp.consume_key(Modifiers::NONE, Key::ArrowLeft)) {
                                        text_response.surrender_focus();
                                        // Prevent subtract with overflow error
                                        let col_left = (9 + j - 1) % 9;
                                        self.grid[i][j].focus = false;
                                        self.grid[i][col_left].focus = true;
                                        self.grid[i][col_left].show_text = true;
                                    }

                                    if ui.input_mut(|inp| inp.consume_key(Modifiers::NONE, Key::ArrowRight)) {
                                        text_response.surrender_focus();
                                        let col_right = (j + 1) % 9;
                                        self.grid[i][j].focus = false;
                                        self.grid[i][col_right].focus = true;
                                        self.grid[i][col_right].show_text = true;
                                    }
                                }
                                
                                // Make sure to keep requesting focus until you have it
                                if text_response.has_focus() && self.grid[i][j].focus {
                                    self.grid[i][j].focus = false;
                                }

                                // If you just clicked on an empty square then immediatly focus to allow typing
                                // Or if this is the square that the cursor needs to move too
                                if response.clicked() || self.grid[i][j].focus {
                                    text_response.request_focus();
                                }
                                
                                // If you do not have focus and the value is empty then do not show anymore
                                if !text_response.has_focus() && self.grid[i][j].value.is_empty() && !self.grid[i][j].focus {
                                    self.grid[i][j].show_text = false;
                                }
                                
                                
                                // This makes sure that we both limit the length of the input and make sure its a number 1-9 inclusive
                                if let Some(first_char) = self.grid[i][j].value.chars().next() {
                                    // TODO: Remove `square.value.len() > 1` once https://github.com/emilk/egui/pull/2816 is accepted
                                    if self.grid[i][j].value.len() > 1 {
                                        self.grid[i][j].value = first_char.to_string();
                                    } else if !first_char.is_ascii_digit() || first_char == '0' {
                                        self.grid[i][j].value.clear();
                                    }
                                }
                                
                            });
                        }
                        
                        pos.x += square_size;

                        // This is for the horizontal spacing for each 3x3 Cube
                        if (j + 1) % 3 == 0 {
                            pos.x += 3.0;
                        }
                    }

                    // Reset the x position to the initial start for a row
                    pos.x = initial_position.x;
                    // Increment the y position to a new row
                    pos.y += square_size;

                    // Vertical spacing for 3x3 Cube
                    if (i + 1) % 3 == 0 {
                        pos.y += 3.0;
                    }
                }
            });
        });
    }
}
