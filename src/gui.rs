use crate::os;
use crate::search;
use eframe::{
    egui::{self, ecolor::HexColor},
    NativeOptions,
};

#[derive(Default, Debug)]
struct ParamCheck {
    status: i32,     // 状态，成功 or 失败
    message: String, // 提示信息
}

pub fn configure_fonts(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily};
    let mut fonts = egui::FontDefinitions::default();

    // 加载自定义字体
    fonts.font_data.insert(
        "my_chinese_font".to_owned(),
        FontData::from_static(include_bytes!("./fonts/msyh.ttc")), // 字体路径
    );

    // 设置自定义字体为主要字体
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_chinese_font".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "my_chinese_font".to_owned());

    ctx.set_fonts(fonts);
}

pub fn configure_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::default();
    // visuals.dark_mode = true; // 使用深色主题
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 140, 140); // 背景色
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(128, 0, 30); // 悬停时的背景色
    visuals.override_text_color = Some(egui::Color32::from_rgb(234, 89, 40)); // 文本颜色

    ctx.set_visuals(visuals);
}

#[derive(Default)]
pub struct FileSearchApp {
    search_name: String, // 用户输入的文件名
    search_path: String, // 用户输入的路径

    search_hidden: bool, // 是否搜索隐藏文件夹
    file_only: bool,     // 是否仅搜索文件
    strict_mode: bool,   // 严格匹配模式
    case_miss: bool,     // 忽略大小写
    file_suffix: String,

    search_results: Vec<String>, // 搜索结果
    status_message: String,      // 状态信息
    message: Option<String>,
}

impl eframe::App for FileSearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("搜索文件");
            ui.separator();

            // 输入文件名与路径
            ui.horizontal(|ui| {
                ui.label("文件名称：");
                ui.text_edit_singleline(&mut self.search_name);
                ui.add_space(10.0); // 可选：在两个输入框之间增加间距
                ui.label("搜索目录：");
                ui.text_edit_singleline(&mut self.search_path);
            });
            ui.separator();
            // 隐藏文件夹、文件+文件夹选项
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.search_hidden, "是否搜索隐藏文件夹");
                ui.add_space(10.0); // 可选：在两个输入框之间增加间距
                ui.checkbox(&mut self.file_only, "是否仅搜索文件");
                ui.add_space(10.0); // 可选：在两个输入框之间增加间距
                ui.checkbox(&mut self.strict_mode, "严格匹配模式");
                ui.add_space(10.0); // 可选：在两个输入框之间增加间距
                ui.checkbox(&mut self.case_miss, "忽略大小写");
            });
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("指定后缀名：");
                ui.text_edit_singleline(&mut self.file_suffix);
            });
            ui.separator();
            // 按钮触发搜索
            if ui.button("开始搜索").clicked() {
                let check_param = check_search_param(&self.search_name);
                if check_param.status == 1 {
                    self.message = Some(check_param.message);
                } else {
                    self.status_message = "正在搜索...".to_string();

                    let argumets = search::Argument {
                        name: self.search_name.clone(),
                        target: self.search_path.clone(),
                        search_hidden: self.search_hidden,
                        file_only: self.file_only,
                        strict_mode: self.strict_mode,
                        case_miss: self.case_miss,
                        suffix: self.file_suffix.clone(),
                    };

                    self.search_results = search::search(argumets);
                    self.status_message = if self.search_results.is_empty() {
                        "未找到匹配的文件！".to_string()
                    } else {
                        format!("找到 {} 个文件！", self.search_results.len())
                    };
                }
            }

            ui.separator();
            // 显示状态信息
            ui.label(&self.status_message);

            // 显示搜索结果
            ui.separator();
            ui.label("搜索结果：");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in &self.search_results {
                    ui.horizontal(|ui| {
                        ui.colored_label(egui::Color32::from_rgb(129, 216, 208), file);
                        // 创建按钮，并通过样式定制颜色
                    });
                    ui.horizontal(|ui| {
                        if ui.button("在文件管理器中打开").clicked() {
                            os::open_in_file_explorer(&file);
                        }
                        if ui.button("复制").clicked() {
                            ctx.output_mut(|o| o.copied_text = file.clone()); // 将文件路径复制到剪贴板
                            self.status_message = format!("已复制: {}", file);
                        }
                    });
                }
            });

            if let Some(mes) = self.message.clone() {
                // 创建一个错误窗口，并设置位置和大小
                egui::Window::new("错误")
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0)) // 锚点设置为居中
                    .resizable(false)
                    .collapsible(false)
                    .show(ui.ctx(), |ui| {
                        ui.label(mes); // 错误信息
                        ui.separator();
                        if ui.button("关闭").clicked() {
                            self.message = None; // 清空错误信息
                        }
                    });
            }
        });
    }
}

fn check_search_param(search_name: &str) -> ParamCheck {
    if search_name.is_empty() {
        return ParamCheck {
            status: 1,
            message: String::from("搜索关键字不能为空"),
        };
    }
    ParamCheck {
        status: 2,
        message: String::from(""),
    }
}
