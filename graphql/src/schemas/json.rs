use crate::schemas::treebuf;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub count: Count,
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub id: String,
    pub order_total: u64,
    pub wearable_total: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub created_at: String,
    pub nft: NFT,
    pub price: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NFT {
    pub bids: Vec<Bid>,
    pub wearable: Wearable,
    pub search_is_land: bool,
    pub search_is_wearable_accessory: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub block_number: String,
    pub status: String,
    pub bidder: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Wearable {
    pub body_shapes: Vec<String>,
    pub category: String,
    pub collection: String,
    pub name: String,
    pub owner: Owner,
    pub rarity: String,
    pub representation_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub mana: String,
}

impl From<Response> for treebuf::Response {
    fn from(response: Response) -> Self {
        Self {
            data: response.data.into(),
        }
    }
}

impl From<Data> for treebuf::Data {
    fn from(data: Data) -> Self {
        Self {
            count: data.count.into(),
            orders: data.orders.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Count> for treebuf::Count {
    fn from(count: Count) -> Self {
        Self {
            id: count.id,
            order_total: count.order_total,
            wearable_total: count.wearable_total,
        }
    }
}

impl From<Order> for treebuf::Order {
    fn from(order: Order) -> Self {
        Self {
            created_at: order.created_at.parse().unwrap(),
            nft: order.nft.into(),
            price: order.created_at.parse().unwrap(),
            status: order.status.parse().unwrap(),
        }
    }
}

impl From<NFT> for treebuf::NFT {
    fn from(nft: NFT) -> Self {
        Self {
            wearable: nft.wearable.into(),
            bids: nft.bids.into_iter().map(|b| b.into()).collect(),
            search_is_land: nft.search_is_land,
            search_is_wearable_accessory: nft.search_is_wearable_accessory,
        }
    }
}

impl From<Bid> for treebuf::Bid {
    fn from(bid: Bid) -> Self {
        Self {
            bidder: treebuf::parse_id(&bid.bidder),
            block_number: bid.block_number.parse().unwrap(),
            status: bid.status.parse().unwrap(),
        }
    }
}

impl From<Wearable> for treebuf::Wearable {
    fn from(wearable: Wearable) -> Self {
        Self {
            body_shapes: wearable
                .body_shapes
                .into_iter()
                .map(|w| w.parse().unwrap())
                .collect(),
            category: wearable.category.parse().unwrap(),
            collection: wearable.collection,
            name: wearable.name,
            owner: wearable.owner.into(),
            rarity: wearable.rarity.parse().unwrap(),
            representation_id: wearable.representation_id,
        }
    }
}

impl From<Owner> for treebuf::Owner {
    fn from(order: Owner) -> Self {
        Self {
            mana: order.mana.parse().unwrap(),
        }
    }
}
