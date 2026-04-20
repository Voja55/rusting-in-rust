use macroquad::prelude::*;
use crate::grid::{CellState, Grid};
use crate::rules::next_cell_state;

//ai pallette
const COLOR_CLEAN:        Color = Color::new(0.70, 0.74, 0.78, 1.0); // steel gray
const COLOR_SURFACE_RUST: Color = Color::new(0.70, 0.35, 0.12, 1.0); // light rust
const COLOR_HEAVY_RUST:   Color = Color::new(0.47, 0.16, 0.04, 1.0); // deep rust
const COLOR_ROTTEN:       Color = Color::new(0.16, 0.08, 0.02, 1.0); // near black
const COLOR_BG:           Color = Color::new(0.10, 0.10, 0.10, 1.0);
const COLOR_PANEL:        Color = Color::new(0.15, 0.15, 0.15, 1.0);
const COLOR_BTN:          Color = Color::new(0.25, 0.25, 0.28, 1.0);
const COLOR_BTN_HOVER:    Color = Color::new(0.35, 0.35, 0.40, 1.0);
const COLOR_BTN_ACTIVE:   Color = Color::new(0.45, 0.42, 0.55, 1.0);

const PANEL_W:  f32 = 220.0;
const PADDING:  f32 = 16.0;

struct Button {
    x: f32, y: f32, w: f32, h: f32,
    label: String,
}

impl Button {
    fn new(x: f32, y: f32, w: f32, h: f32, label: &str) -> Self {
        Self { x, y, w, h, label: label.to_string() }
    }

    fn hovered(&self) -> bool {
        let (mx, my) = mouse_position();
        mx >= self.x && mx <= self.x + self.w &&
        my >= self.y && my <= self.y + self.h
    }

    fn clicked(&self) -> bool {
        self.hovered() && is_mouse_button_pressed(MouseButton::Left)
    }

    fn draw(&self, active: bool) {
        let color = if active        { COLOR_BTN_ACTIVE }
                    else if self.hovered() { COLOR_BTN_HOVER }
                    else                   { COLOR_BTN };
        draw_rectangle(self.x, self.y, self.w, self.h, color);
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 1.0,
            Color::new(0.5, 0.5, 0.5, 0.5));

        let font_size = 16.0;
        let text_w = measure_text(&self.label, None, font_size as u16, 1.0).width;
        draw_text(
            &self.label,
            self.x + (self.w - text_w) / 2.0,
            self.y + self.h / 2.0 + font_size / 3.0,
            font_size,
            WHITE,
        );
    }
}

struct Slider {
    x: f32, y: f32, w: f32,
    label: String,
    value: f32,
    min: f32, max: f32,
    dragging: bool,
}

impl Slider {
    fn new(x: f32, y: f32, w: f32, label: &str, value: f32, min: f32, max: f32) -> Self {
        Self { x, y, w, label: label.to_string(), value, min, max, dragging: false }
    }

    fn update(&mut self) {
        let track_y = self.y + 28.0;
        let (mx, my) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left)
            && mx >= self.x && mx <= self.x + self.w
            && my >= track_y - 8.0 && my <= track_y + 8.0
        {
            self.dragging = true;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging {
            let t = ((mx - self.x) / self.w).clamp(0.0, 1.0);
            self.value = self.min + t * (self.max - self.min);
        }
    }

    fn draw(&self) {
        // label + value
        draw_text(&self.label, self.x, self.y + 16.0, 14.0, LIGHTGRAY);
        let val_text = format!("{:.2}", self.value);
        let val_w = measure_text(&val_text, None, 14, 1.0).width;
        draw_text(&val_text, self.x + self.w - val_w, self.y + 16.0, 14.0, WHITE);

        // track
        let track_y = self.y + 28.0;
        draw_rectangle(self.x, track_y - 2.0, self.w, 4.0,
            Color::new(0.35, 0.35, 0.35, 1.0));

        // fill
        let t = (self.value - self.min) / (self.max - self.min);
        draw_rectangle(self.x, track_y - 2.0, self.w * t, 4.0,
            Color::new(0.55, 0.45, 0.75, 1.0));

        // thumb
        draw_circle(self.x + self.w * t, track_y, 7.0, WHITE);
    }
}

fn cell_color(state: &CellState) -> Color {
    match state {
        CellState::Clean       => COLOR_CLEAN,
        CellState::SurfaceRust => COLOR_SURFACE_RUST,
        CellState::HeavyRust   => COLOR_HEAVY_RUST,
        CellState::Rotten      => COLOR_ROTTEN,
    }
}

fn draw_grid(grid: &Grid, cell_size: f32) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let color = cell_color(&grid.rust[y][x]);
            draw_rectangle(
                x as f32 * cell_size,
                y as f32 * cell_size,
                cell_size - 0.5,
                cell_size - 0.5,
                color,
            );
        }
    }
}

fn draw_panel(
    screen_w: f32,
    screen_h: f32,
    step: usize,
    paused: bool,
    fps: f32,
) {
    let px = screen_w - PANEL_W;
    draw_rectangle(px, 0.0, PANEL_W, screen_h, COLOR_PANEL);
    draw_line(px, 0.0, px, screen_h, 1.0, Color::new(0.3, 0.3, 0.3, 1.0));

    // title
    draw_text("Rusting in Rust", px + PADDING, 36.0, 18.0, WHITE);
    draw_line(px + PADDING, 46.0, screen_w - PADDING, 46.0, 1.0,
        Color::new(0.3, 0.3, 0.3, 1.0));

    // stats
    draw_text(&format!("Step:  {}", step),        px + PADDING, 72.0,  14.0, LIGHTGRAY);
    draw_text(&format!("FPS:   {:.0}", fps),       px + PADDING, 92.0,  14.0, LIGHTGRAY);
    draw_text(if paused { "State: Paused" } else { "State: Running" },
              px + PADDING, 112.0, 14.0,
              if paused { YELLOW } else { GREEN });
}

fn draw_legend(panel_x: f32, y: f32) {
    let items = [
        (COLOR_CLEAN,        "Clean"),
        (COLOR_SURFACE_RUST, "Surface rust"),
        (COLOR_HEAVY_RUST,   "Heavy rust"),
        (COLOR_ROTTEN,       "Rotten"),
    ];
    draw_text("Legend", panel_x + PADDING, y, 14.0, LIGHTGRAY);
    for (i, (color, label)) in items.iter().enumerate() {
        let iy = y + 20.0 + i as f32 * 22.0;
        draw_rectangle(panel_x + PADDING, iy - 10.0, 14.0, 14.0, *color);
        draw_text(label, panel_x + PADDING + 22.0, iy, 13.0, WHITE);
    }
}

pub async fn run_gui(width: usize, height: usize, humidity: f32, oxygen: f32) {
    let mut grid = Grid::new_with_params(width, height, humidity, oxygen);

    // force one rusted cell in the middle to start spreading
    grid.rust[height / 2][width / 2] = crate::grid::CellState::HeavyRust;

    let screen_w = screen_width();
    let screen_h = screen_height();
    let grid_w   = screen_w - PANEL_W;
    let cell_size = (grid_w / width as f32).min(screen_h / height as f32);

    let panel_x = screen_w - PANEL_W;

    // buttons
    let btn_y0 = 140.0;
    let btn_w  = PANEL_W - PADDING * 2.0;
    let mut btn_pause  = Button::new(panel_x + PADDING, btn_y0,        btn_w, 36.0, "Pause");
    let mut btn_reset  = Button::new(panel_x + PADDING, btn_y0 + 46.0, btn_w, 36.0, "Reset");
    let mut btn_slow   = Button::new(panel_x + PADDING, btn_y0 + 92.0, btn_w / 2.0 - 4.0, 30.0, "Slow");
    let mut btn_fast   = Button::new(panel_x + PADDING + btn_w / 2.0 + 4.0, btn_y0 + 92.0, btn_w / 2.0 - 4.0, 30.0, "Fast");

    // sliders
    let slider_y0 = btn_y0 + 140.0;
    let mut s_humidity = Slider::new(panel_x + PADDING, slider_y0,        btn_w, "Humidity", humidity, 0.0, 1.0);
    let mut s_oxygen   = Slider::new(panel_x + PADDING, slider_y0 + 56.0, btn_w, "Oxygen",   oxygen,   0.0, 1.0);

    let mut paused      = false;
    let mut step        = 0usize;
    let mut frame_timer = 0.0f32;
    let mut frame_delay = 0.08f32; // seconds between simulation steps

    loop {
        let dt = get_frame_time();

        // --- input ---
        if btn_pause.clicked() {
            paused = !paused;
            btn_pause.label = if paused { "Resume".to_string() } else { "Pause".to_string() };
        }

        if btn_reset.clicked() {
            grid = Grid::new_with_params(width, height, s_humidity.value, s_oxygen.value);
            grid.rust[height / 2][width / 2] = crate::grid::CellState::HeavyRust;
            step = 0;
            paused = false;
            btn_pause.label = "Pause".to_string();
        }

        if btn_slow.clicked() { frame_delay = (frame_delay * 1.5).min(1.0); }
        if btn_fast.clicked() { frame_delay = (frame_delay * 0.67).max(0.016); }

        s_humidity.update();
        s_oxygen.update();

        // apply slider values to grid every frame
        for y in 0..grid.height {
            for x in 0..grid.width {
                grid.humidity[y][x] = s_humidity.value;
                grid.oxygen[y][x]   = s_oxygen.value;
            }
        }

        // --- simulation step ---
        if !paused {
            frame_timer += dt;
            if frame_timer >= frame_delay {
                frame_timer = 0.0;

                let new_rust: Vec<Vec<_>> = (0..grid.height)
                    .map(|y| (0..grid.width)
                        .map(|x| next_cell_state(&grid, x, y))
                        .collect())
                    .collect();
                grid.rust = new_rust;
                step += 1;
            }
        }

        // --- draw ---
        clear_background(COLOR_BG);

        draw_grid(&grid, cell_size);
        draw_panel(screen_w, screen_h, step, paused, get_fps() as f32);
        draw_legend(panel_x, 380.0);

        btn_pause.draw(paused);
        btn_reset.draw(false);
        btn_slow.draw(false);
        btn_fast.draw(false);

        s_humidity.draw();
        s_oxygen.draw();

        // speed indicator
        let speed_text = format!("Speed: {:.0}ms/step", frame_delay * 1000.0);
        draw_text(&speed_text, panel_x + PADDING, btn_y0 + 132.0, 13.0, LIGHTGRAY);

        next_frame().await;
    }
}