import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;

describe("Sixteen Personalities Test", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SixteenPersonalities;
  const answer = anchor.web3.Keypair.generate();


  it("Testing your personality", async () => {

    const txe = await program.rpc.initialize(
      provider.wallet.publicKey,
      false, //i_worry_about_everything1 ?
      false, //i_make_friends_easily2 ?
      true, //i_have_a_lot_of_imagination3 ?
      false, //i_trust_others4 ?
      true, //i_complete_tasks_successfully5 ?
      false, //i_get_angry_easily6 ?
      false, //i_really_enjoy_large_parties_and_gatherings7 ?
      true, //i_think_at_is_important8 ?
      false, //i_sometimes_deceive_others_to_get_my_own_way9 ?
      true, //i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10 ?
      false, //i_often_feel_sad11 ?
      false, //i_like_to_take_charge_of_situations_and_events12 ?
      true, //i_experience_deep_and_varied_emotions13 ?
      false, //i_love_to_help_others14 ?
      true, //i_always_keep_my_promises15 ?
      false, //i_find_it_difficult_to_approach_others16 ?
      true, //i_am_always_busy_always_on_the_go17 ?
      false, //i_prefer_variety_to_routine18 ?
      true, //i_love_a_good_argument_a_good_fight19 ?
      true, //i_work_very_hard20 ?
      false, //i_will_overindulge_at_times21 ?
      true, //i_love_excitement22 ?
      true, //i_enjoy_reading_challenging_books_and_articles23 ?
      true, //i_believe_that_i_am_better_than_others24 ?
      false, //i_am_always_prepared25 ?
      false, //i_panic_easily26 ?
      false, //i_am_a_really_cheerful_person27 ?
      true, //i_tend_to_support_progress_and_reform28 ?
      false, //i_sympathise_with_the_homeless29 ?
      false, //i_am_very_spontaneous_i_act_without_thinking30 ?
      false, //i_fear_for_the_worst31 ?
      true, //i_feel_comfortable_around_people32 ?
      true, //i_enjoy_wild_flights_of_fantasy33 ?
      false, //i_believe_that_people_basically_have_good_intentions34 ?
      true, //when_i_do_something_i_always_do_it_well35 ?
      false, //i_get_irritated_easily36?
      false, //i_always_chat_to_lots_of_different_people_at_parties37 ?
      true, //i_see_beauty_in_things_that_others_might_not_notice38 ?
      true, //i_dont_mind_cheating_to_get_ahead39 ?
      false, //i_often_forget_to_put_things_back_in_their_proper_place40 ?
      true, //i_sometimes_dislike_myself41 ?
      true, //i_try_to_be_in_charge_to_lead_others42 ?
      false, //i_am_empathetic_i_feel_others_emotions43 ?
      true, //i_am_concerned_about_others44 ?
      true, //i_tell_the_truth45 ?
      false, //i_am_afraid_to_draw_attention_to_myself46 ?
      true, //i_never_sit_still_im_always_on_the_go47 ?
      false, //i_prefer_to_stick_with_things_that_i_know48 ?
      false, //i_shout_and_yell_at_people49 ?
      true, //i_do_more_than_whats_expected_of_me50 ?
      true, //i_rarely_over_indulge51 ?
      false, //i_go_out_of_my_way_to_seek_adventure52 ?
      false, //i_avoid_philosophical_discussions53 ?
      true, //i_think_highly_of_myself54 ?
      true, //i_get_the_job_done_and_carry_out_my_plans55 ?
      false, //i_become_overwhelmed_by_events56 ?
      true, //i_have_a_lot_of_fun57 ?
      false, //i_believe_that_there_is_no_absolute_right_or_wrong58 ?
      false, //i_feel_sympathy_for_those_who_are_worse_off_than_myself59 ?
      false, //i_make_rash_decisions60 ?
      false, //i_am_afraid_of_many_things61 ?
      true, //i_avoid_coming_into_contact_with_people_if_i_can_help_it62 ?
      true, //i_love_to_daydream ?
      false, //i_trust_what_people_say64 ?
      true, //i_handle_tasks_methodically65 ?
      false, //i_frequently_lose_my_temper66 ?
      false, //i_prefer_to_be_alone67 ?
      true, //i_do_not_like_poetry68 ?
      true, //i_sometimes_take_advantage_of_others69 ?
      false, //i_sometimes_leave_the_place_in_a_mess70 ?
    {
      accounts: {
        answer: answer.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [answer],
    });
    console.log("The result of your personality test is here:", txe);
  });


  /*it('can delete a tweet', async () => {
    const del = await program.rpc.deleteQuiz({
        accounts: {
            answer: answer.publicKey,
            authority: provider.wallet.publicKey,
        },
    });
    console.log("You deleted your account",del)
  });*/

});
