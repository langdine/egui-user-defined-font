#![cfg_attr(not(debug_assertions), windows_subsystem = "windows",)]
#![allow(unused_imports)] 
// windows_subsystem 告诉编译器，程序运行时隐藏命令行窗口,同时禁止终端输出。println!等输出不会出现在终端

use core::error;
use std::{error::Error, fmt::format, fs::File, io::Read, str::FromStr, vec};
use crate::egui::*;

use eframe::egui;

// use eframe::App;
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
      
        ..Default::default()
    };
    eframe::run_native(
        "Hello word 中文", //应用程序名称
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))), 
        //第三个参数为程序构建器(eframe::AppCreator类型)负责创建应用程序上下文(egui::Context)。_cc为&CreationContextl类型，_cc.egui_ctx字段即为Context。
        //之所以强调Context的创建过程，是因为显示中文字体需要配置Context。
    )
}

struct MyApp {
    count: u32,
    name: String,
    age: u32,
    err_msg: Vec<String>,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            count:0,
            name: "张三".to_owned(),
            age: 42,
            err_msg: vec![],
        }
    }
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        match load_font(&cc.egui_ctx){
            Ok(_) => {
                Self::default()
            },
            Err(e)=>{
                Self{
                    err_msg: vec![format!("{}",e )],//如果启用了font_path但是未在目录中找到字体文件，将错误记录到err_msg中。
                    ..Self::default()
                }
            }
            
        }
        //egui默认字体无法显示中文，需要加载中文字体。配置字体应该在构造函数中。本例从可执行文件同级别的fonts目录读取字体。
        //网上部分教程将字体配置写入了update函数，update函数每一帧都会运行一次，每秒60次，因此在update函数中加载字体是错误且低效的。
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.count = self.count +1;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui 0.29.1 Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                let _r = ui.text_edit_multiline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Add message").clicked() {
                self.err_msg.push(format!("Err:{} {} years",self.name,self.age));
            }
            ui.label(format!("count: {}",self.count));
            for i in self.err_msg.iter(){
                let l =RichText::new(i.as_str()).color(Color32::from_rgb(255, 0, 0));
                ui.label(l);
            }
            self.count= self.count+1;
        });
    }
}

#[cfg(not(feature = "font_path"))]
pub fn load_font(ctx: &egui::Context) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut fonts = eframe::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "AlibabaPuHuiTi-3-55-Regular".to_owned(), 
        eframe::egui::FontData::from_static(  include_bytes!("resources/AlibabaPuHuiTi-3-55-Regular.ttf")  )); // .ttf and .otf supported
    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "AlibabaPuHuiTi-3-55-Regular".to_owned());
    ctx.set_fonts(fonts);          
    Ok(()) // 未使用font_path
}


/// 为了支持中文，我们加载阿里巴巴普惠体字体：下载自https://fonts.alibabagroup.com/#/home
/// If font_path feature not actived, Embed fonts into the executable file. This will increase the size of the executable but provides portability.  
/// If font_path feature actived, fonts are not embedded into the executable, all ttf and otf fonts in the fonts directory at the same level as the executable will be loaded.
/// 
#[cfg(feature = "font_path")]
pub fn load_font(ctx: &egui::Context) -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use glob::glob;
    use std::path::Path;
    let tmp =env::current_exe()?;
    //可执行程序所在路径下查找 "./fonts/*.ttf" and "./fonts/*.otf"字体文件。
    let local_exec_path = tmp.as_path().parent().unwrap().to_str().unwrap();
    let font_path = format!("{}/fonts/",local_exec_path);
    let pattern0=format!("{}/*.ttf",font_path.as_str() );
    let pattern1=format!("{}/*.otf",font_path.as_str() );
    let font_path_vec :Vec<_> = 
                                glob(pattern0.as_str()).unwrap()
                                .chain(glob(pattern1.as_str()).unwrap())
                               .collect();
    if font_path_vec.len()==0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, r#"The font is not found in the application's font path. This will cause non-English text to not display. Please place a .ttf or .otf file into the "fonts/" directory."#)));
        }
    
    for font_path in font_path_vec.into_iter(){
        let f = font_path.unwrap();
        println!("{:?}",&f);
        let font_key = f.file_name().unwrap().to_os_string().into_string().unwrap();
        let mut f_io = File::open(f)?;
        let mut fonts_data = Vec::<u8>::new();
        f_io.read_to_end(&mut fonts_data)?;
        let mut fonts = eframe::egui::FontDefinitions::default();
        fonts.font_data.insert(
            font_key.clone(),
            eframe::egui::FontData::from_owned(     fonts_data   )); // .ttf and .otf supported
        fonts
            .families
            .get_mut(&eframe::egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, font_key);
        ctx.set_fonts(fonts);
    }             
    Ok(()) //使用了 font_path
}
