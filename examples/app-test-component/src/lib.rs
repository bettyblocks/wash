wit_bindgen::generate!({ generate_all });

use crate::betty_blocks::create::create::create;
use crate::betty_blocks::update::update::update;
use crate::exports::betty_blocks::action::action;
struct Component;

impl action::Guest for Component {
    fn run() -> Result<String, String> {
        let _ = create();
        let _ = update();

        Ok("Ran action".to_string())
    }
}
export!(Component);
