use crate::messages::prelude::*;

use serde::{Deserialize, Serialize};

#[impl_message(Message, PortfolioMessage, MenuBar)]
#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum MenuBarMessage {
	// Messages
	SendLayout,
}
