use serde::{Serialize, Deserialize};
use std::{collections::{BTreeMap,HashMap}, hash::Hash};


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct LimitOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: Option<i64>,
    pub order_quantity:i32,
    pub order_side: String,
    pub expiration:String, //tsisy raha market
    pub price: i32, //tsisy raha market
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: Option<i64>,
    pub order_quantity:i32,
    pub order_side: String,
    pub expiration:String,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StopOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_quantity:i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32, 
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StopLimitOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_quantity:i32,
    pub order_side: String,
    pub expiration:String, 
    pub trigger_price: i32, 
    pub price:i32
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifyOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
    pub new_quantity:i32,
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeleteOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MarketConf {
    pub market_name: String,
    pub price_increment: i32,
    pub asset1: String,
    pub asset2: String,
    pub exchange: String,
    pub iprcv:String,
    pub ipmarket:String,
    pub ipbroker:String,
    pub ipexchangedb:String,
    pub ipmarketob:String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TraderOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub price: i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32,
    pub price: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeletedOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub price: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeletedStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeletedStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32,
    pub price :i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ModifiedOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: i32,
    pub new_order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub price: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ModifiedStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: i32,
    pub new_order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ModifiedStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: i32,
    pub new_order_quantity: i32,
    pub order_side: String,
    pub expiration:String,
    pub trigger_price: i32,
    pub price: i32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct TradeStruct {
    pub market: String,
    pub broker_identifier_taker: String,
    pub broker_identifier_maker: String,
    pub unix_time: i64,
    pub trade_identifier:i64,
    pub trader_identifier_taker: i64,
    pub order_identifier_taker: i64,
    pub trader_identifier_maker: i64,
    pub order_identifier_maker: i64,
    pub taker_type:String,
    pub expiration_taker:String,
    pub expiration_maker:String,
    pub order_quantity: i32,
    pub order_side: String,
    pub price: i32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PostTraderInf {
    pub unix_time:i64,
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub asset1:String,
    pub asset2:String,
    pub trader_calcbalance_asset1:i32,
    pub trader_calcbalance_asset2:i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NBBO {
   
    pub unix_time: i64,
    pub ask_price: Option<i32>,
    pub bid_price:Option<i32>,
    pub ask_size:Option<i32>,
    pub bid_size:Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSale {
    
    pub market: String,
    pub exchange:String,
    pub unix_time: i64,
    pub order_quantity: i32,
    pub order_side: String,
    pub price:i32,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Last {
    pub unix_time:i64,
    pub price:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBPData {
    pub sidembp : String,
    pub mbp: BTreeMap<i32, i32>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBOData {
    pub sidembo : String,
    pub mbo: BTreeMap<i32, Vec<i32>>,//key is price and value is quantity
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Volume {
   
    pub unix_time: i64,
    pub volume:i32,
    pub value:i32,
    pub price:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPData {
    pub sidemap : String,
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPStopData {
  
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPStopLimitData {
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Messaging {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub message : String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MBPEvents {
    pub unix_time : i64,
    pub side : String,
    pub event_value:i32,
    pub event_price:i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Structs {
    LimitOrder(LimitOrder),
    MarketOrder(MarketOrder),
    StopOrder(StopOrder),
    StopLimitOrder(StopLimitOrder),
    ModifyOrder(ModifyOrder),
    DeleteOrder(DeleteOrder),
    TraderOrderStruct(TraderOrderStruct),
    TraderStopOrderStruct(TraderStopOrderStruct),
    TraderStopLimitOrderStruct(TraderStopLimitOrderStruct),
    DeletedOrderStruct(DeletedOrderStruct),
    DeletedStopOrderStruct(DeletedStopOrderStruct),
    DeletedStopLimitOrderStruct(DeletedStopLimitOrderStruct),
    ModifiedStopOrderStruct(ModifiedStopOrderStruct),
    ModifiedStopLimitOrderStruct(ModifiedStopLimitOrderStruct),
    ModifiedOrderStruct(ModifiedOrderStruct),
    TradeStruct(TradeStruct),
    PostTraderInf(PostTraderInf),
    NBBO(NBBO),
    TimeSale(TimeSale),
    Last(Last),
    MBPData(MBPData),
    MBOData(MBOData),
    Volume(Volume),
    MAPData(MAPData),
    MBPEvents(MBPEvents),
    MarketConf(MarketConf),
    Messaging(Messaging),
}

