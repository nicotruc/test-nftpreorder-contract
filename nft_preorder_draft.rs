#![no_std]

elrond_wasm::imports!();.

const NFT_LIMIT = 4800;
const NFT_PRICE = 1;

#[elrond_wasm::derive::contract]
pub trait NFT_Preordering{
    #[view(getremaining)]
    #[storage_mapper("remaining_nft")]
    fn remaining_nft(&self) -> SingleValueMapper<BigInt>;

    #[init]
    fn init(&self, initial_value: BigInt) {
        self.remaining_nft().set(NFT_LIMIT);
    }

    /// Pre order NFT
    #[payable("EGLD")]
    #[endpoint]
    fn preorder(&self, #[payment] amount: BigInt) -> SCResult<()> {
	let remaining = self.remaining_nft().get();
	require!(remaining > 0, "Can't order NFT : limit reached");
	
	let number = amount / NFT_PRICE;

	require!(remaining - number > 0, "Not enough remaining to buy NFT");
	// TODO  : Synchronously check NFT availability, so someone which empty the counter with his amount get his remaining egld back
        self.remaining_nft().update(|rem| *rem -= number);

        Ok(())
    }
}
