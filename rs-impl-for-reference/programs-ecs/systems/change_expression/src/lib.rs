use bolt_lang::*;
use face::Face;

declare_id!("HTzQyEQyLYVSBTrVXYeZuXFYCXL6RpNS8xDN6XxeZN96");

#[system]
pub mod wink {

    pub fn execute(ctx: Context<Components>, _args_p: Vec<u8>) -> Result<Components> {
        let face = &mut ctx.accounts.face;
        face.eyes = String::from("closed");
        face.mouth = String::from("smirk");
        face.eyebrows = String::from("raised");
        Ok(ctx.accounts)
    }

    #[system_input]
    pub struct Components {
        pub face: Face,
    }
}
