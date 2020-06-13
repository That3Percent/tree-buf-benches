use hex;
use std::convert::TryInto;
use std::str::FromStr;
use tree_buf::prelude::*;

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Response {
    pub data: Data,
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Data {
    pub count: Count,
    pub orders: Vec<Order>,
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Count {
    pub id: String,
    pub order_total: u64,
    pub wearable_total: u64,
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Order {
    pub created_at: u64,
    pub nft: NFT,
    pub price: u64,
    pub status: Status,
}

// TODO: Manually put in unit type here, but need to
// just support data-less enums
#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub enum Status {
    Cancelled,
    Sold,
    Open,
}

impl FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cancelled" => Ok(Self::Cancelled),
            "sold" => Ok(Self::Sold),
            "open" => Ok(Self::Open),
            _ => Err(s.to_owned()),
        }
    }
}

type Id = [u8; 20];

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct NFT {
    pub wearable: Wearable,
    pub bids: Vec<Bid>,
    pub search_is_land: bool,
    pub search_is_wearable_accessory: bool,
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Bid {
    pub bidder: Id,
    pub status: BidStatus,
    pub block_number: u64,
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub enum BidStatus {
    Sold,
    Cancelled,
    Open,
}

impl FromStr for BidStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(Self::Open),
            "sold" => Ok(Self::Sold),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(s.to_owned()),
        }
    }
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Wearable {
    pub body_shapes: Vec<BodyShape>,
    pub category: Category,
    pub collection: String,
    pub name: String,
    pub owner: Owner,
    pub rarity: Rarity,
    pub representation_id: String,
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub enum BodyShape {
    BaseMale,
    BaseFemale,
}

impl FromStr for BodyShape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BaseFemale" => Ok(Self::BaseFemale),
            "BaseMale" => Ok(Self::BaseMale),
            _ => Err(s.to_owned()),
        }
    }
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub enum Category {
    Mask,
    UpperBody,
    LowerBody,
    Hat,
    Earring,
    Feet,
    TopHead,
    Helmet,
    EyeWear,
    Hair,
    Tiara,
}

impl FromStr for Category {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mask" => Ok(Self::Mask),
            "upper_body" => Ok(Self::UpperBody),
            "lower_body" => Ok(Self::LowerBody),
            "hat" => Ok(Self::UpperBody),
            "earring" => Ok(Self::Earring),
            "feet" => Ok(Self::Feet),
            "top_head" => Ok(Self::TopHead),
            "helmet" => Ok(Self::Helmet),
            "eyewear" => Ok(Self::EyeWear),
            "hair" => Ok(Self::Hair),
            "tiara" => Ok(Self::Tiara),
            _ => Err(s.to_owned()),
        }
    }
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub enum Rarity {
    Mythic,
    Legendary,
    Epic,
    Swanky,
    Uncommon,
    Rare,
}

impl FromStr for Rarity {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mythic" => Ok(Self::Epic),
            "legendary" => Ok(Self::Legendary),
            "epic" => Ok(Self::Epic),
            "swanky" => Ok(Self::Swanky),
            "uncommon" => Ok(Self::Uncommon),
            "rare" => Ok(Self::Rare),
            _ => Err(s.to_owned()),
        }
    }
}

pub fn parse_id(id: &str) -> Id {
    let without_prefix = id.trim_start_matches("0x");
    let bytes = hex::decode(without_prefix).unwrap();
    (bytes[..]).try_into().unwrap()
}

#[derive(Decode, Encode, Debug, PartialEq, Eq)]
pub struct Owner {
    pub mana: u64,
}
