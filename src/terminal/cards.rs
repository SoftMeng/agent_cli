#[derive(Debug, Clone, PartialEq)]
pub enum CardType {
    System,
    User,
    Assistant,
    Thinking,
    Tool,
    Error,
}

#[derive(Debug, Clone)]
pub struct ChatCard {
    pub card_type: CardType,
    pub title: String,
    pub content: String,
    pub tool_call_id: Option<String>,
    pub tool_response: Option<String>,
    pub expanded: bool,
}

// ============ Constructors ============

impl ChatCard {
    pub fn new(card_type: CardType, title: impl Into<String>, content: impl Into<String>) -> Self {
        let expanded = !matches!(card_type, CardType::Thinking);
        Self {
            card_type,
            title: title.into(),
            content: content.into(),
            tool_call_id: None,
            tool_response: None,
            expanded,
        }
    }

    pub fn new_tool(
        name: impl Into<String>,
        internal_id: impl Into<String>,
        args: impl Into<String>,
    ) -> Self {
        Self {
            card_type: CardType::Tool,
            title: name.into(),
            content: args.into(),
            tool_call_id: Some(internal_id.into()),
            tool_response: None,
            expanded: false,
        }
    }
}

// ============ Mutators ============

impl ChatCard {
    pub fn append_content(&mut self, s: &str) {
        self.content.push_str(s);
    }

    pub fn set_response(&mut self, response: impl Into<String>) {
        self.tool_response = Some(response.into());
    }

    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}
