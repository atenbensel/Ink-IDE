use ink::prelude::*;
use ink_storage::StorageValue;

#[derive(Debug, PartialEq, Eq)]
pub struct NFT {
    id: u64,
    owner: AccountId,
    metadata: String,
}

#[ink(contract)]
pub mod nft_contract {
    #[ink(storage)]
    pub struct NftManager {
        next_id: u64,
        nfts: StorageValue<HashMap<u64, NFT>>,
    }

    impl NftManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                next_id: 1,
                nfts: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint_nft(&mut self, metadata: String) -> Result<u64, ink::Error> {
            let id = self.next_id;
            self.next_id += 1;

            self.nfts.insert(id, NFT {
                id,
                owner: env::caller(),
                metadata,
            });

            Ok(id)
        }

        #[ink(message)]
        pub fn get_nft(&self, id: u64) -> Option<NFT> {
            self.nfts.get(&id).copied()
        }

        #[ink(message)]
        pub fn transfer_nft(&mut self, from: AccountId, to: AccountId, id: u64) -> Result<(), ink::Error> {
            if let Some(nft) = self.nfts.get(&id) {
                if nft.owner != from {
                    return Err(ink::Error::Unauthorized);
                }

                self.nfts.insert(id, NFT {
                    id,
                    owner: to,
                    metadata: nft.metadata,
                });

                Ok(())
            } else {
                Err(ink::Error::NFTNotFound)
            }
        }
    }
}
