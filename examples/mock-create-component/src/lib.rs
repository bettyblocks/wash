wit_bindgen::generate!({ generate_all });

use crate::exports::betty_blocks::create::create;
struct Component;

impl create::Guest for Component {
    fn create() -> Result<String, String> {
        Ok("CALLING CREATE".to_string())
    }
}
export!(Component);
