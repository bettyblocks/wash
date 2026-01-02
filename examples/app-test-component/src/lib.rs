wit_bindgen::generate!({ generate_all });

use crate::exports::betty_blocks::action::action;
struct Component;

impl action::Guest for Component {
    fn run() -> Result<String, String>{
        Ok("Ran action".to_string())
    }
}
export!(Component);
