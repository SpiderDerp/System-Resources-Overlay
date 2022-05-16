
use fltk::{app, prelude::*, window::*, enums::{Color, Align}, image::*, frame::*, text::*, widget::*, group::*};
use sys_info;

fn main() {
    let app = app::App::default();
    let mut wind = OverlayWindow::new(0, 0, 200, 100, "");
    let mut wid = Widget::new(0, 0, 200, 100, "");

    wind.set_color(Color::from_rgba_tuple((0, 0, 0, 0)));
    wind.set_border(true);
    wind.set_visible_focus();

    let mut pack = Pack::new(0, 0, 200, 100, "");
    //get ram usage
    //fra.set_image(Some(_image));
    pack.end();
    wind.end();
    wind.show();

    let ram = sys_info::mem_info().unwrap();
    let ram_usage_percent = (ram.free as f64 / ram.total as f64) * 100.0;

    let mut _image = JpegImage::load(&std::path::Path::new("images/quandaledingle.jpg")).unwrap();
    let mut fra = Frame::new(0, 0, 200, 50, "Ram Usage:");

    let mut text = TextDisplay::new(0,0,200,100, "");
    text.set_label(format!("{:.2}%", ram_usage_percent).as_str());

    pack.add(&fra);
    pack.add(&text);
    
    while app.wait(){
        let ram = sys_info::mem_info().unwrap();
        let ram_usage_percent = (ram.free as f64 / ram.total as f64) * 100.0;
        text.set_label(format!("{:.2}%", ram_usage_percent).as_str());
        wind.redraw();
    }
}