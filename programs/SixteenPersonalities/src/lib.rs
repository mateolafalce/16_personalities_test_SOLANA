use anchor_lang::prelude::*;

declare_id!("oYt6UFzxCZLxweeFg9DQXipYjskuw3mANRbvBMaLNUa");

#[program]
pub mod sixteen_personalities {
    use super::*;

    pub fn create_quiz(ctx: Context<Create>, 
        authority: Pubkey
    ) -> Result<()> {
        let answer = &mut ctx.accounts.answer;
        answer.authority = authority;
        Ok(())
    }

    pub fn initialize(ctx: Context<Answers>,
        authority: Pubkey,
        i_am_a_worrier1: bool,
        i_make_friends_easily2: bool,
        i_have_a_lot_of_imagination3: bool,
        i_trust_others4: bool,
        i_complete_tasks_successfully5: bool,
        i_get_angry_easily6: bool,
        i_really_enjoy_large_parties_and_gatherings7: bool,
        i_think_at_is_important8: bool,
        i_sometimes_deceive_others_to_get_my_own_way9: bool,
        i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10: bool,
        i_often_feel_sad11: bool,
        i_like_to_take_charge_of_situations_and_events12: bool,
        i_experience_deep_and_varied_emotions13:bool,
        i_love_to_help_others14: bool,
        i_always_keep_my_promises15: bool,
        i_find_it_difficult_to_approach_others16: bool,
        i_am_always_busy_always_on_the_go17: bool,
        i_prefer_variety_to_routine18: bool,
        i_love_a_good_argument_a_good_fight19: bool,
        i_work_very_hard20: bool
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
        answer.i_often_feel_sad11 = i_often_feel_sad11; let i_often_feel_sad11_ = answer.i_often_feel_sad11 as u8;
        answer.i_like_to_take_charge_of_situations_and_events12 = i_like_to_take_charge_of_situations_and_events12; let i_like_to_take_charge_of_situations_and_events12_ = answer.i_like_to_take_charge_of_situations_and_events12 as u8;
        answer.i_experience_deep_and_varied_emotions13 = i_experience_deep_and_varied_emotions13; let i_experience_deep_and_varied_emotions13_ = answer.i_experience_deep_and_varied_emotions13 as u8;
        answer.i_love_to_help_others14 = i_love_to_help_others14; let i_love_to_help_others14_ = answer.i_love_to_help_others14 as u8;
        answer.i_always_keep_my_promises15 = i_always_keep_my_promises15; let i_always_keep_my_promises15_ = answer.i_always_keep_my_promises15 as u8;
        answer.i_find_it_difficult_to_approach_others16 = i_find_it_difficult_to_approach_others16; let i_find_it_difficult_to_approach_others16_ = answer.i_find_it_difficult_to_approach_others16 as u8;
        answer.i_am_always_busy_always_on_the_go17 = i_am_always_busy_always_on_the_go17; let i_am_always_busy_always_on_the_go17_ = answer.i_am_always_busy_always_on_the_go17 as u8;
        answer.i_prefer_variety_to_routine18 = i_prefer_variety_to_routine18; let i_prefer_variety_to_routine18_ = answer.i_prefer_variety_to_routine18 as u8;
        answer.i_love_a_good_argument_a_good_fight19 = i_love_a_good_argument_a_good_fight19; let i_love_a_good_argument_a_good_fight19_ = answer.i_love_a_good_argument_a_good_fight19 as u8;
        answer.i_work_very_hard20 = i_work_very_hard20; let i_work_very_hard20_ = answer.i_work_very_hard20 as u8;

        let score: u8 = i_am_a_worrier1_ 
        + i_make_friends_easily2_
        + i_have_a_lot_of_imagination3_
        + i_trust_others4_
        + i_complete_tasks_successfully5_
        + i_get_angry_easily6_
        + i_really_enjoy_large_parties_and_gatherings7_
        + i_think_at_is_important8_
        + i_sometimes_deceive_others_to_get_my_own_way9_
        + i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10_
        + i_often_feel_sad11_
        + i_like_to_take_charge_of_situations_and_events12_
        + i_experience_deep_and_varied_emotions13_
        + i_love_to_help_others14_
        + i_always_keep_my_promises15_
        + i_find_it_difficult_to_approach_others16_
        + i_am_always_busy_always_on_the_go17_
        + i_prefer_variety_to_routine18_
        + i_love_a_good_argument_a_good_fight19_
        + i_work_very_hard20_;

        /*if score <= 0 {
            msg!("Sos una persona timida timido");
        } else if score >= 1 {
            msg!("Sos una persona muy atrevida");
        }
        */
        msg!("{}", score);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = Answer16personalitites::LEN)]
    pub answer: Account<'info, Answer16personalitites>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Answers<'info> {
    #[account(mut, has_one = authority)]
    pub answer: Account<'info, Answer16personalitites>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Answer16personalitites {
    pub authority: Pubkey,
    pub i_am_a_worrier1: bool,
    pub i_make_friends_easily2: bool,
    pub i_have_a_lot_of_imagination3: bool,
    pub i_trust_others4: bool,
    pub i_complete_tasks_successfully5: bool,
    pub i_get_angry_easily6: bool,
    pub i_really_enjoy_large_parties_and_gatherings7: bool,
    pub i_think_at_is_important8: bool,
    pub i_sometimes_deceive_others_to_get_my_own_way9: bool,
    pub i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10: bool,
    pub i_often_feel_sad11: bool,
    pub i_like_to_take_charge_of_situations_and_events12: bool,
    pub i_experience_deep_and_varied_emotions13: bool,
    pub i_love_to_help_others14: bool,
    pub i_always_keep_my_promises15: bool,
    pub i_find_it_difficult_to_approach_others16: bool,
    pub i_am_always_busy_always_on_the_go17: bool,
    pub i_prefer_variety_to_routine18: bool,
    pub i_love_a_good_argument_a_good_fight19: bool,
    pub i_work_very_hard20: bool

}

impl Answer16personalitites {
    const LEN: usize = DISCRIMINATOR 
    + BOOL_ANSWERS 
    + PUBKEY;
}

const DISCRIMINATOR: usize = 8;
const BOOL_ANSWERS: usize = 1 * 20;
const PUBKEY: usize = 32;
