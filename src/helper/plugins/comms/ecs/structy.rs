use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReceiveCommsMessageType {
    BrowserBlockchainData,
    BrowserGameTiles,
    ServerBlockchainData,
    GameTilesByTs,
    GameTilesByHeight,
    CheckLnPayment,
    LnInvoice,
    UserInventory,
    BlockData,
}

#[derive(Deserialize)]
pub struct MessageTypeExtract {
    pub msg_type: String,
}
