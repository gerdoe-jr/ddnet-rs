pub mod pred_timer_ui;

use ui_generic::traits::UiPageInterface;

pub use api_ui::ui_impl::*;
pub use api_ui_game::render::*;

#[no_mangle]
fn mod_ui_new() -> Box<dyn UiPageInterface<()>> {
    Box::new(pred_timer_ui::page::PredTimerPage::new())
}
