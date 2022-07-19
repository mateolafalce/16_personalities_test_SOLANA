use anchor_lang::prelude::*;

declare_id!("2sZ2p39s4wTQrVteV8XP5r8d8cbqr2pdQnpvU3BvcbkK");

#[program]
pub mod sixteen_personalities {
    use super::*;

    pub fn initialize(ctx: Context<Answers>,
        i_am_a_worrier1: bool,
        i_make_friends_easily2: bool,
        i_have_a_lot_of_imagination3: bool,
        i_trust_others4: bool,
        i_complete_tasks_successfully5: bool,
        i_get_angry_easily6: bool,
        i_really_enjoy_large_parties_and_gatherings7: bool,
        i_think_at_is_important8: bool,
        i_sometimes_deceive_others_to_get_my_own_way9: bool
        i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10: bool
    ) -> Result<()> {
        let answer = &mut ctx.accounts.answer;
        answer.i_am_a_worrier1 = i_am_a_worrier1; let i_am_a_worrier1_ = answer.i_am_a_worrier1 as u8;
        answer.i_make_friends_easily2 = i_make_friends_easily2; let i_make_friends_easily2_ = answer.i_make_friends_easily2 as u8;
        answer.i_have_a_lot_of_imagination3 = i_have_a_lot_of_imagination3; let i_have_a_lot_of_imagination3_ = answer.i_have_a_lot_of_imagination3 as u8;
        answer.i_trust_others4 = i_trust_others4; let i_trust_others4_ = answer.i_trust_others4 as u8;
        answer.i_complete_tasks_successfully5 = i_complete_tasks_successfully5; let i_complete_tasks_successfully5_ = answer.i_complete_tasks_successfully5 as u8;
        answer.i_get_angry_easily6 = i_get_angry_easily6; let i_get_angry_easily6_ = answer.i_get_angry_easily6 as u8;
        answer.i_really_enjoy_large_parties_and_gatherings7 = i_really_enjoy_large_parties_and_gatherings7; let i_really_enjoy_large_parties_and_gatherings7_ = answer.i_really_enjoy_large_parties_and_gatherings7 as u8;
        answer.i_think_at_is_important8 = i_think_at_is_important8; let i_think_at_is_important8_ = answer.i_think_at_is_important8 as u8;
        answer.i_sometimes_deceive_others_to_get_my_own_way9 = i_sometimes_deceive_others_to_get_my_own_way9; let i_sometimes_deceive_others_to_get_my_own_way9_ = answer.i_sometimes_deceive_others_to_get_my_own_way9 as u8;
        answer.i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10 = i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10; let i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10_ = answer.i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10 as u8;


        let score: u8 = i_am_a_worrier1_ 
        + i_make_friends_easily2_
        + i_have_a_lot_of_imagination3_
        + i_trust_others4_
        + i_complete_tasks_successfully5_
        + i_get_angry_easily6_
        + i_really_enjoy_large_parties_and_gatherings7_
        + i_think_at_is_important8_
        + i_sometimes_deceive_others_to_get_my_own_way9
        + i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10;

        /*if score <= 0 {
            msg!("Sos una persona timida timido");
        } else if score >= 1 {
            msg!("Sos una persona muy atrevida");
        }
        */
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Answers<'info> {
    #[account(init, payer = user, space = Answer16personalitites::LEN)]
    pub answer: Account<'info, Answer16personalitites>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Answer16personalitites {
    i_am_a_worrier1: bool,
    i_make_friends_easily2: bool,
    i_have_a_lot_of_imagination3: bool,
    i_trust_others4: bool,
    i_complete_tasks_successfully5: bool,
    i_get_angry_easily6: bool,
    i_really_enjoy_large_parties_and_gatherings7: bool,
    i_think_at_is_important8: bool,
    i_sometimes_deceive_others_to_get_my_own_way9: bool,
    i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10: bool

}

impl Answer16personalitites {
    const LEN: usize = DISCRIMINATOR 
    + BOOL_ANSWERS;
}

const DISCRIMINATOR: usize = 8;
const BOOL_ANSWERS: usize = 1 * 10;
