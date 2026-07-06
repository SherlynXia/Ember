use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "我的热力图工具",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    data: Vec<Vec<f64>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 初始化数据（10x10 高斯分布）
        if self.data.is_empty() {
            let size = 10;
            self.data = (0..size)
                .map(|x| {
                    (0..size)
                        .map(|y| {
                            let cx = size as f64 / 2.0;
                            let cy = size as f64 / 2.0;
                            let sigma = 2.0;
                            let dx = x as f64 - cx;
                            let dy = y as f64 - cy;
                            ((dx * dx + dy * dy) / (2.0 * sigma * sigma)).exp()
                        })
                        .collect()
                })
                .collect();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔥 热力图 (原生绘制)");
            ui.label("每个方块颜色代表数值大小");

            // 获取当前可用空间
            let available_size = ui.available_size();
            let size = self.data.len(); // 假设是方阵
            let cell_size = (available_size.x.min(available_size.y) / size as f32).min(40.0);

            // 计算热力图整体的绘制区域（居中）
            let total_size = cell_size * size as f32;
            let start_x = (available_size.x - total_size) / 2.0;
            let start_y = (available_size.y - total_size) / 2.0;

            // 获取 painter
            let painter = ui.painter();

            // 遍历数据绘制方块
            for (row, row_data) in self.data.iter().enumerate() {
                for (col, &value) in row_data.iter().enumerate() {
                    // 计算方块位置
                    let x = start_x + col as f32 * cell_size;
                    let y = start_y + row as f32 * cell_size;
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::vec2(cell_size, cell_size),
                    );

                    // 将数值映射到颜色（蓝 -> 青 -> 黄 -> 红）
                    let color = value_to_color(value);
                    painter.rect_filled(rect, 0.0, color);

                    // 可选：在方块中心显示数值（保留一位小数）
                    let text = format!("{:.1}", value);
                    painter.text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        text,
                        egui::FontId::default(),
                        egui::Color32::WHITE,
                    );
                }
            }

            // 添加坐标轴标签（简单示例）
            ui.label("X轴 →");
        });
    }
}

// 将 [0,1] 范围内的值映射到颜色（蓝 -> 青 -> 黄 -> 红）
fn value_to_color(value: f64) -> egui::Color32 {
    let v = value.clamp(0.0, 1.0) as f32;
    // 分段线性插值：0->蓝(0,0,255), 0.33->青(0,255,255), 0.66->黄(255,255,0), 1->红(255,0,0)
    let (r, g, b) = if v < 0.33 {
        let t = v / 0.33;
        (0.0, t * 255.0, 255.0)
    } else if v < 0.66 {
        let t = (v - 0.33) / 0.33;
        (0.0, 255.0, 255.0 * (1.0 - t))
    } else {
        let t = (v - 0.66) / 0.34;
        (t * 255.0, 255.0 * (1.0 - t), 0.0)
    };
    egui::Color32::from_rgb(r as u8, g as u8, b as u8)
}