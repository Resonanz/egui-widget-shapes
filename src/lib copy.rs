use egui::{Color32, Frame, Pos2, Rect, Response, Sense, Slider, Stroke, Ui, Vec2, Widget};

/// Configuration for the
/// widget appearance
//#[derive(Clone)]
pub struct ConfigFrame {
    // Frame config
    frame_size: Vec2,
    frame_rounding: f32,
    frame_stroke: Stroke,
    frame_stroke_hover: Stroke,
    frame_fill: Color32,
    frame_fill_hover: Color32,
    frame_rect_abs_coords: Rect,
}

impl Default for ConfigFrame {
    fn default() -> Self {
        Self {
            // Frame
            frame_size: Vec2::new(300.0, 240.0),
            frame_rounding: 4.0,
            frame_stroke: Stroke::new(2.0, Color32::from_gray(100)),
            frame_stroke_hover: Stroke::new(2.0, Color32::WHITE),
            frame_fill: Color32::BLACK,
            frame_fill_hover: Color32::from_black_alpha(123),
            frame_rect_abs_coords: Rect {
                min: egui::Pos2 { x: 0., y: 0. },
                max: egui::Pos2 { x: 0., y: 0. },
            },
        }
    }
}

pub struct ConfigButtonRect {
    rect_size: f32,
    rect_spacing: f32,
    inactive_color: Color32,
    active_color: Color32,
    hover_color: Color32,
    rect_rounding: f32,
}

impl Default for ConfigButtonRect {
    fn default() -> Self {
        Self {
            //
            rect_size: 50.0,
            rect_spacing: 40.0,
            inactive_color: Color32::from_gray(180),
            active_color: Color32::from_rgb(100, 200, 100),
            hover_color: Color32::from_rgb(150, 220, 150),
            rect_rounding: 2.0,
        }
    }
}

// WidgetState is the portal that
// stores shared state between the
// widget crate and other crates.
//#[derive(Default)]
pub struct WidgetState<'a> {
    // state: ToggleState<'a>,
    // config: ToggleRectsConfig,
    button_on_1: &'a mut bool,
    button_on_2: &'a mut bool,
    button_pressed_1: &'a mut bool,
    button_pressed_2: &'a mut bool,
    widget_hovered: &'a mut bool,
    slider_val: &'a mut f32,
    //config: WidgetConfig,
}

impl<'a> WidgetState<'a> {
    pub fn new(
        button_on_1: &'a mut bool,
        button_on_2: &'a mut bool,
        button_pressed_1: &'a mut bool,
        button_pressed_2: &'a mut bool,
        widget_hovered: &'a mut bool,
        slider_val: &'a mut f32,
    ) -> Self {
        Self {
            button_on_1,
            button_on_2,
            button_pressed_1,
            button_pressed_2,
            widget_hovered,
            slider_val,
            //config: Default::default(),
        }
    }
}

//
//
//
//
//
impl<'a> Widget for WidgetState<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        // Instantiate the required
        // widget building blocks
        let mut config_frame = ConfigFrame::default();
        let config_button_rect = ConfigButtonRect::default();

        let frame = Frame::none()
            .inner_margin(10.0)
            .outer_margin(10.)
            .fill(if *self.widget_hovered {
                config_frame.frame_fill_hover
            } else {
                config_frame.frame_fill
            })
            .stroke(if *self.widget_hovered {
                config_frame.frame_stroke_hover
            } else {
                config_frame.frame_stroke
            })
            .rounding(config_frame.frame_rounding);

        let frame_response = frame.show(ui, |ui| {
            ui.set_min_size(config_frame.frame_size);
            config_frame.frame_rect_abs_coords = ui.min_rect(); // Capture the frame coordinates

            // Access the absolute position and size of the frame
            config_frame.frame_rect_abs_coords = ui.min_rect(); // ui.min_rect() is the smallest rectangle that
                                                                // fully encompasses the widget you’re working on.
                                                                // The top-left corner of this rectangle is the
                                                                // absolute position of the widget in the window.
            let frame_max_rect = ui.max_rect(); // ui.max_rect() is the maximum rectangle
                                                //allocated to the UI, which can be helpful
                                                //if you want the bounds of the entire UI area
                                                //available to the current context.

            let total_width =
                (config_button_rect.rect_size * 2.0) + config_button_rect.rect_spacing;
            let start_x = (config_frame.frame_size.x - total_width) / 2.0;
            let start_y = (config_frame.frame_size.y - config_button_rect.rect_size) / 2.0;

            //
            // Left rectangle
            //
            let left_rect = Rect::from_min_size(
                // Calculate top left corner of rect
                ui.min_rect().min + Vec2::new(start_x, start_y),
                // Size of rect, stretching down/right from top left corner
                Vec2::new(config_button_rect.rect_size, config_button_rect.rect_size),
            );
            // egui "ignores layout of the Ui and puts widget here!"
            let left_response = ui.allocate_rect(left_rect, Sense::click());

            if left_response.clicked() {
                *self.button_on_1 = !*self.button_on_1;
                *self.button_pressed_1 = true;
            }
            self.draw_rect(
                ui,
                &config_frame,
                &config_button_rect,
                left_rect,
                *self.button_on_1,
                &left_response,
            );

            //
            // Right rectangle
            //
            let right_rect = Rect::from_min_size(
                ui.min_rect().min
                    + Vec2::new(
                        start_x + config_button_rect.rect_size + config_button_rect.rect_spacing,
                        start_y,
                    ),
                Vec2::new(config_button_rect.rect_size, config_button_rect.rect_size),
            );
            let right_response = ui.allocate_rect(right_rect, Sense::click());

            if right_response.clicked() {
                *self.button_on_2 = !*self.button_on_2;
                *self.button_pressed_2 = true;
            }
            self.draw_rect(
                ui,
                &config_frame,
                &config_button_rect,
                right_rect,
                *self.button_on_2,
                &right_response,
            );

            //
            // Draw slider
            //
            // fn show_slider(ui: &mut egui::Ui, slider_val: &mut f32) {
            // Define the exact position and size of the slider
            let position = egui::pos2(left_rect.left() - 50.0, left_rect.bottom() + 20.0); // X and Y coordinates
            let size = Vec2::new(150.0, 30.0); // Width and height
            let rect = Rect::from_min_size(position, size);

            ui.style_mut().spacing.slider_width = 150.;

            // Place the slider at the defined rectangle
            ui.put(
                rect,
                Slider::new(&mut *self.slider_val, 0.0..=100.0).text("My value"),
            );

            // Define rect for slider to fit into
            let position = egui::pos2(left_rect.left() - 50.0, left_rect.bottom() + 44.0); // X and Y coordinates
            let size = Vec2::new(120.0, 30.0); // Width and height
            let rect = Rect::from_min_size(position, size);

            // Place the slider at the defined rectangle
            ui.put(
                rect,
                Slider::new(&mut *self.slider_val, 0.0..=100.0).text("My value"),
            );
            // }

            // Return both responses for combining
            (left_response, right_response)
        });

        // Combine the responses
        let mut response = frame_response.response;
        let (left_response, right_response) = frame_response.inner;

        // Update the response's clicked state based on either rectangle being clicked
        if left_response.clicked() || right_response.clicked() {
            response.mark_changed();
        }

        // Handle frame hover effect
        if response.contains_pointer() {
            *self.widget_hovered = true;
        } else {
            *self.widget_hovered = false;
        }
        response
    }
}

//
//
//
//
//
impl<'a> WidgetState<'a> {
    fn draw_rect(
        &mut self,
        ui: &mut Ui,
        config_frame: &ConfigFrame,
        config_button_rect: &ConfigButtonRect,
        rect: Rect,
        is_active: bool,
        response: &Response,
    ) {
        let color = if is_active {
            if response.hovered() {
                config_button_rect.hover_color
            } else {
                config_button_rect.active_color
            }
        } else {
            if response.hovered() {
                config_button_rect.hover_color
            } else {
                config_button_rect.inactive_color
            }
        };

        ui.painter()
            .rect_filled(rect, config_button_rect.rect_rounding, color);

        let origin = Pos2 {
            x: config_frame.frame_rect_abs_coords.min.x,
            y: config_frame.frame_rect_abs_coords.min.y,
        };

        // let my_wee_rect = Rect {
        //     min: egui::pos2(0., 0.),
        //     max: egui::pos2(33., 33.),
        // };

        // for i in 1..50 {
        //     let rect = my_wee_rect.translate(Vec2 {
        //         x: origin.x + i as f32,
        //         y: origin.y + i as f32,
        //     });

        //     ui.painter()
        //         .rect_filled(rect, widget_config.rect_rounding, color);
        // }
    }
}
