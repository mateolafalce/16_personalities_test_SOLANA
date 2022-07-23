use anchor_lang::prelude::*;

declare_id!("CiLfoGqJQuKfXTMDgqrdPuXMGhzHAwEP8b7bizVQBeqp");

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
        i_work_very_hard20: bool,
        i_will_overindulge_at_times21: bool,
        i_love_excitement22: bool,
        i_enjoy_reading_challenging_books_and_articles23: bool,
        i_believe_that_i_am_better_than_others24: bool,
        i_am_always_prepared25: bool,
        i_panic_easily26: bool,
        i_am_a_really_cheerful_person27: bool,
        i_tend_to_support_progress_and_reform28: bool,
        i_sympathise_with_the_homeless29: bool,
        i_am_very_spontaneous_i_act_without_thinking30:bool,
        i_fear_for_the_worst31: bool,
        i_feel_comfortable_around_people32: bool,
        i_enjoy_wild_flights_of_fantasy33: bool,
        i_believe_that_people_basically_have_good_intentions34: bool,
        when_i_do_something_i_always_do_it_well35: bool,
        i_get_irritated_easily36: bool,
        i_always_chat_to_lots_of_different_people_at_parties37: bool,
        i_see_beauty_in_things_that_others_might_not_notice38: bool,
        i_dont_mind_cheating_to_get_ahead39: bool,
        i_often_forget_to_put_things_back_in_their_proper_place40: bool,
        i_sometimes_dislike_myself41: bool,
        i_try_to_be_in_charge_to_lead_others42: bool,
        i_am_empathetic_i_feel_others_emotions43: bool,
        i_am_concerned_about_others44: bool,
        i_tell_the_truth45: bool,
        i_am_afraid_to_draw_attention_to_myself46: bool,
        i_never_sit_still_im_always_on_the_go47: bool,
        i_prefer_to_stick_with_things_that_i_know48: bool,
        i_shout_and_yell_at_people49: bool,
        i_do_more_than_whats_expected_of_me50: bool,
        i_rarely_over_indulge51: bool,
        i_go_out_of_my_way_to_seek_adventure52: bool,
        i_avoid_philosophical_discussions53: bool,
        i_think_highly_of_myself54: bool,
        i_get_the_job_done_and_carry_out_my_plans55: bool,
        i_become_overwhelmed_by_events56: bool,
        i_have_a_lot_of_fun57: bool,
        i_believe_that_there_is_no_absolute_right_or_wrong58: bool,
        i_feel_sympathy_for_those_who_are_worse_off_than_myself59: bool,
        i_make_rash_decisions60: bool

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
        answer.i_will_overindulge_at_times21 = i_will_overindulge_at_times21; let i_will_overindulge_at_times21_ = answer.i_will_overindulge_at_times21 as u8;
        answer.i_love_excitement22 = i_love_excitement22; let i_love_excitement22_ = answer.i_love_excitement22 as u8;
        answer.i_enjoy_reading_challenging_books_and_articles23 = i_enjoy_reading_challenging_books_and_articles23; let i_enjoy_reading_challenging_books_and_articles23_ = answer.i_enjoy_reading_challenging_books_and_articles23 as u8;
        answer.i_believe_that_i_am_better_than_others24 = i_believe_that_i_am_better_than_others24; let i_believe_that_i_am_better_than_others24_ = answer.i_believe_that_i_am_better_than_others24 as u8;
        answer.i_am_always_prepared25 = i_am_always_prepared25; let i_am_always_prepared25_ = answer.i_am_always_prepared25 as u8;
        answer.i_panic_easily26 = i_panic_easily26; let i_panic_easily26_ = answer.i_panic_easily26 as u8;
        answer.i_am_a_really_cheerful_person27 = i_am_a_really_cheerful_person27; let i_am_a_really_cheerful_person27_ = answer.i_am_a_really_cheerful_person27 as u8;
        answer.i_tend_to_support_progress_and_reform28 = i_tend_to_support_progress_and_reform28; let i_tend_to_support_progress_and_reform28_ = answer.i_tend_to_support_progress_and_reform28 as u8;
        answer.i_sympathise_with_the_homeless29 = i_sympathise_with_the_homeless29; let i_sympathise_with_the_homeless29_ = answer.i_sympathise_with_the_homeless29 as u8;
        answer.i_am_very_spontaneous_i_act_without_thinking30 = i_am_very_spontaneous_i_act_without_thinking30; let i_am_very_spontaneous_i_act_without_thinking30_ = answer.i_am_very_spontaneous_i_act_without_thinking30 as u8;
        answer.i_fear_for_the_worst31 = i_fear_for_the_worst31; let i_fear_for_the_worst31_ = answer.i_fear_for_the_worst31 as u8;
        answer.i_feel_comfortable_around_people32 = i_feel_comfortable_around_people32; let i_feel_comfortable_around_people32_ = answer.i_feel_comfortable_around_people32 as u8;
        answer.i_enjoy_wild_flights_of_fantasy33 = i_enjoy_wild_flights_of_fantasy33; let i_enjoy_wild_flights_of_fantasy33_ = answer.i_enjoy_wild_flights_of_fantasy33 as u8;
        answer.i_believe_that_people_basically_have_good_intentions34 = i_believe_that_people_basically_have_good_intentions34; let i_believe_that_people_basically_have_good_intentions34_ = answer.i_believe_that_people_basically_have_good_intentions34 as u8;
        answer.when_i_do_something_i_always_do_it_well35 = when_i_do_something_i_always_do_it_well35; let when_i_do_something_i_always_do_it_well35_ = answer.when_i_do_something_i_always_do_it_well35 as u8;
        answer.i_get_irritated_easily36 = i_get_irritated_easily36; let i_get_irritated_easily36_ = answer.i_get_irritated_easily36 as u8;
        answer.i_always_chat_to_lots_of_different_people_at_parties37 = i_always_chat_to_lots_of_different_people_at_parties37; let i_always_chat_to_lots_of_different_people_at_parties37_ = answer.i_always_chat_to_lots_of_different_people_at_parties37 as u8;
        answer.i_see_beauty_in_things_that_others_might_not_notice38 = i_see_beauty_in_things_that_others_might_not_notice38; let i_see_beauty_in_things_that_others_might_not_notice38_ = answer.i_see_beauty_in_things_that_others_might_not_notice38 as u8;
        answer.i_dont_mind_cheating_to_get_ahead39 = i_dont_mind_cheating_to_get_ahead39; let i_dont_mind_cheating_to_get_ahead39_ = answer.i_dont_mind_cheating_to_get_ahead39 as u8;
        answer.i_often_forget_to_put_things_back_in_their_proper_place40 = i_often_forget_to_put_things_back_in_their_proper_place40; let i_often_forget_to_put_things_back_in_their_proper_place40_ = answer.i_often_forget_to_put_things_back_in_their_proper_place40 as u8;
        answer.i_sometimes_dislike_myself41 = i_sometimes_dislike_myself41; let i_sometimes_dislike_myself41_ = answer.i_sometimes_dislike_myself41 as u8;
        answer.i_try_to_be_in_charge_to_lead_others42 = i_try_to_be_in_charge_to_lead_others42; let i_try_to_be_in_charge_to_lead_others42_ = answer.i_try_to_be_in_charge_to_lead_others42 as u8;
        answer.i_am_empathetic_i_feel_others_emotions43 = i_am_empathetic_i_feel_others_emotions43; let i_am_empathetic_i_feel_others_emotions43_ = answer.i_am_empathetic_i_feel_others_emotions43 as u8;
        answer.i_am_concerned_about_others44 = i_am_concerned_about_others44; let i_am_concerned_about_others44_ = answer.i_am_concerned_about_others44 as u8;
        answer.i_tell_the_truth45 = i_tell_the_truth45; let i_tell_the_truth45_ = answer.i_tell_the_truth45 as u8;
        answer.i_am_afraid_to_draw_attention_to_myself46 = i_am_afraid_to_draw_attention_to_myself46; let i_am_afraid_to_draw_attention_to_myself46_ = answer.i_am_afraid_to_draw_attention_to_myself46 as u8;
        answer.i_never_sit_still_im_always_on_the_go47 = i_never_sit_still_im_always_on_the_go47; let i_never_sit_still_im_always_on_the_go47_ = answer.i_never_sit_still_im_always_on_the_go47 as u8;
        answer.i_prefer_to_stick_with_things_that_i_know48 = i_prefer_to_stick_with_things_that_i_know48; let i_prefer_to_stick_with_things_that_i_know48_ = answer.i_prefer_to_stick_with_things_that_i_know48 as u8;
        answer.i_shout_and_yell_at_people49 = i_shout_and_yell_at_people49; let i_shout_and_yell_at_people49_ = answer.i_shout_and_yell_at_people49 as u8;
        answer.i_do_more_than_whats_expected_of_me50 = i_do_more_than_whats_expected_of_me50; let i_do_more_than_whats_expected_of_me50_ = answer.i_do_more_than_whats_expected_of_me50 as u8;
        answer.i_rarely_over_indulge51 = i_rarely_over_indulge51; let i_rarely_over_indulge51_ = answer.i_rarely_over_indulge51 as u8;
        answer.i_go_out_of_my_way_to_seek_adventure52 = i_go_out_of_my_way_to_seek_adventure52; let i_go_out_of_my_way_to_seek_adventure52_ = answer.i_go_out_of_my_way_to_seek_adventure52 as u8;
        answer.i_avoid_philosophical_discussions53 = i_avoid_philosophical_discussions53; let i_avoid_philosophical_discussions53_ = answer.i_avoid_philosophical_discussions53 as u8;
        answer.i_think_highly_of_myself54 = i_think_highly_of_myself54; let i_think_highly_of_myself54_ = answer.i_think_highly_of_myself54 as u8;
        answer.i_get_the_job_done_and_carry_out_my_plans55 = i_get_the_job_done_and_carry_out_my_plans55; let i_get_the_job_done_and_carry_out_my_plans55_ = answer.i_get_the_job_done_and_carry_out_my_plans55 as u8;
        answer.i_become_overwhelmed_by_events56 = i_become_overwhelmed_by_events56; let i_become_overwhelmed_by_events56_ = answer.i_become_overwhelmed_by_events56 as u8;
        answer.i_have_a_lot_of_fun57 = i_have_a_lot_of_fun57; let i_have_a_lot_of_fun57_ = answer.i_have_a_lot_of_fun57 as u8;
        answer.i_believe_that_there_is_no_absolute_right_or_wrong58 = i_believe_that_there_is_no_absolute_right_or_wrong58; let i_believe_that_there_is_no_absolute_right_or_wrong58_ = answer.i_believe_that_there_is_no_absolute_right_or_wrong58 as u8;
        answer.i_feel_sympathy_for_those_who_are_worse_off_than_myself59 = i_feel_sympathy_for_those_who_are_worse_off_than_myself59; let i_feel_sympathy_for_those_who_are_worse_off_than_myself59_ = answer.i_feel_sympathy_for_those_who_are_worse_off_than_myself59 as u8;
        answer.i_make_rash_decisions60 = i_make_rash_decisions60; let i_make_rash_decisions60_ = answer.i_make_rash_decisions60 as u8;


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
        + i_work_very_hard20_
        + i_will_overindulge_at_times21_
        + i_love_excitement22_
        + i_enjoy_reading_challenging_books_and_articles23_
        + i_believe_that_i_am_better_than_others24_
        + i_am_always_prepared25_
        + i_panic_easily26_
        + i_am_a_really_cheerful_person27_
        + i_tend_to_support_progress_and_reform28_
        + i_sympathise_with_the_homeless29_
        + i_am_very_spontaneous_i_act_without_thinking30_
        + i_fear_for_the_worst31_
        + i_feel_comfortable_around_people32_
        + i_enjoy_wild_flights_of_fantasy33_
        + i_believe_that_people_basically_have_good_intentions34_
        + when_i_do_something_i_always_do_it_well35_
        + i_get_irritated_easily36_
        + i_always_chat_to_lots_of_different_people_at_parties37_
        + i_see_beauty_in_things_that_others_might_not_notice38_
        + i_dont_mind_cheating_to_get_ahead39_
        + i_often_forget_to_put_things_back_in_their_proper_place40_
        + i_sometimes_dislike_myself41_
        + i_try_to_be_in_charge_to_lead_others42_
        + i_am_empathetic_i_feel_others_emotions43_
        + i_am_concerned_about_others44_
        + i_tell_the_truth45_
        + i_am_afraid_to_draw_attention_to_myself46_
        + i_never_sit_still_im_always_on_the_go47_
        + i_prefer_to_stick_with_things_that_i_know48_
        + i_shout_and_yell_at_people49_
        + i_do_more_than_whats_expected_of_me50_
        + i_rarely_over_indulge51_
        + i_go_out_of_my_way_to_seek_adventure52_
        + i_avoid_philosophical_discussions53_
        + i_think_highly_of_myself54_
        + i_get_the_job_done_and_carry_out_my_plans55_
        + i_become_overwhelmed_by_events56_
        + i_have_a_lot_of_fun57_
        + i_believe_that_there_is_no_absolute_right_or_wrong58_
        + i_feel_sympathy_for_those_who_are_worse_off_than_myself59_
        + i_make_rash_decisions60_;

        if score <= 8 {
            msg!("ESTJ - The Controller: Everything around them should be and function as they wish.");
        } else if score > 8 && score <= 16 {
            msg!("ESTP - The charismatic dominatrix: They love that others do what they say.");
        } else if score > 16 && score <= 24 {
            msg!("ESFJ - The helpful: They are interested in serving other people, treating them because of their help.");
        } else if score > 24 && score <= 32 {
            msg!("ESFP - The fun: They show joy and try to transmit it to others.");
        } else if score > 32 && score <= 40 {
            msg!("ISTJ - The Moralist: Everything that moves them should have a moral and normative sense. They are organized and methodical in every aspect of their story.");
        } else if score > 40 && score <= 48 {
            msg!("ISTP - The reserved: They love to exercise logic in everything and look for quick and effective solutions.")
        } else if score > 48 && score <= 56 {
            msg!("ISFJ - The Sacrifice: They love to satisfy the expectations that others have of them and inspire confidence, by not being ambitious.")
        } else if score > 56 && score <= 64 {
            msg!("ISFP - The Realist: Those with their feet on the ground live based on day-to-day situations.")
        } else if score > 64 && score <= 72 {
            msg!("ENTJ - The Boss: His way of being, thinking and acting, makes others continue and admire him.")
        } else if score > 72 && score <= 80 {
            msg!("ENTP - The Curious: They are fascinated by continually experiencing new challenges, more than anything on the scientific intellectual plane.")
        } else if score > 80 && score <= 88 {
            msg!("ENFJ - The Guide: Just as they love to learn, they also love to teach.")
        } else if score > 88 && score <= 96 {
            msg("ENFP - The Innovator: He loves the relationship with others and feeling part of a group.")
        } else if score > 96 && score <= 104 {
            msg!("INTJ - The centered: They continually move in functionality to their own ideas.")
        } else if score > 104 && score <= 112 {
            msg!("INTP - The perfectionist: They love to fix others and are thoughtful.")
        } else if score > 112 && score <= 115 {
            msg!("INFP - The Artistic: With a lot of creativity and also moralistic.")
        } else if score > 115 && score <= 120 {
            msg!("INFJ - The ideological: Quite sensitive, basing her story on a clear ideal or ideals.")
        }
        

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
    pub i_work_very_hard20: bool,
    pub i_will_overindulge_at_times21: bool,
    pub i_love_excitement22: bool,
    pub i_enjoy_reading_challenging_books_and_articles23: bool,
    pub i_believe_that_i_am_better_than_others24: bool,
    pub i_am_always_prepared25: bool,
    pub i_panic_easily26: bool,
    pub i_am_a_really_cheerful_person27: bool,
    pub i_tend_to_support_progress_and_reform28: bool,
    pub i_sympathise_with_the_homeless29: bool,
    pub i_am_very_spontaneous_i_act_without_thinking30: bool,
    pub i_fear_for_the_worst31: bool,
    pub i_feel_comfortable_around_people32: bool,
    pub i_enjoy_wild_flights_of_fantasy33: bool,
    pub i_believe_that_people_basically_have_good_intentions34: bool,
    pub when_i_do_something_i_always_do_it_well35: bool,
    pub i_get_irritated_easily36: bool,
    pub i_always_chat_to_lots_of_different_people_at_parties37: bool,
    pub i_see_beauty_in_things_that_others_might_not_notice38: bool,
    pub i_dont_mind_cheating_to_get_ahead39: bool,
    pub i_often_forget_to_put_things_back_in_their_proper_place40: bool,
    pub i_sometimes_dislike_myself41: bool,
    pub i_try_to_be_in_charge_to_lead_others42: bool,
    pub i_am_empathetic_i_feel_others_emotions43: bool,
    pub i_am_concerned_about_others44: bool,
    pub i_tell_the_truth45: bool,
    pub i_am_afraid_to_draw_attention_to_myself46: bool,
    pub i_never_sit_still_im_always_on_the_go47: bool,
    pub i_prefer_to_stick_with_things_that_i_know48: bool,
    pub i_shout_and_yell_at_people49: bool,
    pub i_do_more_than_whats_expected_of_me50: bool,
    pub i_rarely_over_indulge51: bool,
    pub i_go_out_of_my_way_to_seek_adventure52: bool,
    pub i_avoid_philosophical_discussions53: bool,
    pub i_think_highly_of_myself54: bool,
    pub i_get_the_job_done_and_carry_out_my_plans55: bool,
    pub i_become_overwhelmed_by_events56: bool,
    pub i_have_a_lot_of_fun57: bool,
    pub i_believe_that_there_is_no_absolute_right_or_wrong58: bool,
    pub i_feel_sympathy_for_those_who_are_worse_off_than_myself59: bool,
    pub i_make_rash_decisions60: bool

}

impl Answer16personalitites {
    const LEN: usize = DISCRIMINATOR 
    + BOOL_ANSWERS 
    + PUBKEY;
}

const DISCRIMINATOR: usize = 8;
const BOOL_ANSWERS: usize = 1 * 60;
const PUBKEY: usize = 32;
