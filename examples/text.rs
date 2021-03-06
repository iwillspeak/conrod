

#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use conrod::{Theme, Widget};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};

type Ui = conrod::Ui<Glyphs>;


fn main() {

    // Construct the window.
    let window: PistonWindow =
        WindowSettings::new("Text Demo", [1080, 720])
            .exit_on_esc(true).build().unwrap();

    // Construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    // Poll events from the window.
    for event in window.ups(60) {
        ui.handle_event(&event);
        event.update(|_| ui.set_widgets(set_ui));
        event.draw_2d(|c, g| ui.draw(c, g));
    }

}


fn set_ui(ui: &mut Ui) {
    use conrod::{color, Colorable, Positionable, Sizeable, Split, Text};

    // Generate a unique const `WidgetId` for each widget.
    widget_ids!{
        MASTER,
        LEFT_COL,
        MIDDLE_COL, 
        RIGHT_COL,
        LEFT_TEXT,
        MIDDLE_TEXT,
        RIGHT_TEXT,
    }

    // Our `Canvas` tree, upon which we will place our text widgets.
    Split::new(MASTER).flow_right(&[
        Split::new(LEFT_COL).pad(20.0).color(color::black()),
        Split::new(MIDDLE_COL).pad(20.0).color(color::dark_charcoal()),
        Split::new(RIGHT_COL).pad(20.0).color(color::charcoal()),
    ]).set(ui);

    const DEMO_TEXT: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
        finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
        fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
        Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
        Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
        magna est, efficitur suscipit dolor eu, consectetur consectetur urna.";

    Text::new(DEMO_TEXT)
        .color(color::light_red())
        .kid_area_width_of(LEFT_COL)
        .mid_top_of(LEFT_COL)
        .align_text_left()
        .line_spacing(10.0)
        .set(LEFT_TEXT, ui);

    Text::new(DEMO_TEXT)
        .color(color::light_green())
        .kid_area_width_of(MIDDLE_COL)
        .middle_of(MIDDLE_COL)
        .align_text_middle()
        .set(MIDDLE_TEXT, ui);

    Text::new(DEMO_TEXT)
        .color(color::light_blue())
        .kid_area_width_of(RIGHT_COL)
        .mid_bottom_of(RIGHT_COL)
        .align_text_right()
        .line_spacing(5.0)
        .set(RIGHT_TEXT, ui);
}

