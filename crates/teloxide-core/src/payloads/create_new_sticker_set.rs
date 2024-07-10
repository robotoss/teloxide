//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{InputSticker, StickerFormat, StickerType, True, UserId};

impl_payload! {
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns True on success.
    #[derive(Debug, Clone, Serialize)]
    pub CreateNewStickerSet (CreateNewStickerSetSetters) => True {
        required {
            /// User identifier of sticker file owner
            pub user_id: UserId,
            /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., _animals_). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in _“\_by\_<bot username>”. <bot\_username>_ is case insensitive. 1-64 characters.
            pub name: String [into],
            /// Sticker set title, 1-64 characters
            pub title: String [into],
            /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
            pub stickers: Vec<InputSticker> [collect],
            /// Format of the sticker, must be one of “static”, “animated”, “video”
            pub sticker_format: StickerFormat,
        }
        optional {
            /// Type of stickers in the set, pass “regular”, “mask”, or “custom_emoji”. By default, a regular sticker set is created.
            #[serde(flatten)]
            pub sticker_type: StickerType,
            /// Pass _True_ if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
            pub needs_repainting: bool,
        }
    }
}
