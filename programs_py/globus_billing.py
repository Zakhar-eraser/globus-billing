# globus_billing
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

author_key='4ci8V8pQs9seJZvw66KBRhv56yJPERPRUFurKb98DE3x'

# Account storing info about globes rates to 1 SOL
class Rate(Account):
    owner: Pubkey
    red: u16
    green: u16
    blue: u16

# Player accounts storing SOLs
class Player(Account):
    owner: Pubkey

@instruction
def init_player(owner: Signer, player: Empty[Player]):
    player = player.init(payer = owner, seeds = ['Player', owner])
    player.owner = owner.key()

@instruction
def init_rate(owner: Signer, rate: Empty[Rate], red: u16, green: u16, blue: u16):
    rate = rate.init(payer = owner, seeds = ['Rates', owner])
    rate.owner = owner.key()
    rate.red = red
    rate.green = green
    rate.blue = blue

@instruction
def change_rates(sign: Signer, rate: Rate, red: u16, green: u16, blue: u16):
    assert rate.key() == Pubkey.find_program_address(['Rates', author_key])[0], "This is anyother exchage rates account`s address"
    assert sign.key() == rate.owner, 'You can`t change exchange rates'
    rate.red = red
    rate.green = green
    rate.blue = blue
