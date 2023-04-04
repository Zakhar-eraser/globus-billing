#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

seahorse_const! { author_key , "4ci8V8pQs9seJZvw66KBRhv56yJPERPRUFurKb98DE3x" . to_string () }

#[account]
#[derive(Debug)]
pub struct Player {
    pub owner: Pubkey,
}

impl<'info, 'entrypoint> Player {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedPlayer<'info, 'entrypoint>> {
        let owner = account.owner.clone();

        Mutable::new(LoadedPlayer {
            __account__: account,
            __programs__: programs_map,
            owner,
        })
    }

    pub fn store(loaded: Mutable<LoadedPlayer>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;
    }
}

#[derive(Debug)]
pub struct LoadedPlayer<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Player>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
}

#[account]
#[derive(Debug)]
pub struct Rate {
    pub owner: Pubkey,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl<'info, 'entrypoint> Rate {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedRate<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let red = account.red;
        let green = account.green;
        let blue = account.blue;

        Mutable::new(LoadedRate {
            __account__: account,
            __programs__: programs_map,
            owner,
            red,
            green,
            blue,
        })
    }

    pub fn store(loaded: Mutable<LoadedRate>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let red = loaded.red;

        loaded.__account__.red = red;

        let green = loaded.green;

        loaded.__account__.green = green;

        let blue = loaded.blue;

        loaded.__account__.blue = blue;
    }
}

#[derive(Debug)]
pub struct LoadedRate<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Rate>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

pub fn change_rates_handler<'info>(
    mut sign: SeahorseSigner<'info, '_>,
    mut rate: Mutable<LoadedRate<'info, '_>>,
    mut red: u16,
    mut green: u16,
    mut blue: u16,
) -> () {
    if !(rate.borrow().__account__.key()
        == Pubkey::find_program_address(
            Mutable::new(vec![
                "Rates".to_string().as_bytes().as_ref(),
                author_key!().as_bytes().as_ref(),
            ])
            .borrow()
            .as_slice(),
            &id(),
        )
        .0)
    {
        panic!("This is anyother exchage rates account`s address");
    }

    if !(sign.key() == rate.borrow().owner) {
        panic!("You can`t change exchange rates");
    }

    assign!(rate.borrow_mut().red, red);

    assign!(rate.borrow_mut().green, green);

    assign!(rate.borrow_mut().blue, blue);
}

pub fn init_player_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut player: Empty<Mutable<LoadedPlayer<'info, '_>>>,
) -> () {
    let mut player = player.account.clone();

    assign!(player.borrow_mut().owner, owner.key());
}

pub fn init_rate_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut rate: Empty<Mutable<LoadedRate<'info, '_>>>,
    mut red: u16,
    mut green: u16,
    mut blue: u16,
) -> () {
    let mut rate = rate.account.clone();

    assign!(rate.borrow_mut().owner, owner.key());

    assign!(rate.borrow_mut().red, red);

    assign!(rate.borrow_mut().green, green);

    assign!(rate.borrow_mut().blue, blue);
}
