mod structures {
    use crate::tokens::Tokens;

    pub fn has_open_str(tk: Tokens) -> bool {
        match tk {
            Tokens::DupleQuote | Tokens::SimpleQuote => true,
            _ => false
        }
    }

    pub fn open_str(tk: Tokens, opened: Tokens) -> bool {
        match tk {
            Tokens::SimpleQuote | Tokens::DupleQuote => tk.id() != opened.id(),
            _ => matches!(opened, Tokens::SimpleQuote | Tokens::DupleQuote),
        }
    }
    
    pub fn close_str(tk: Tokens, opened: Tokens) -> bool {
        matches!((tk, opened), 
            (Tokens::SimpleQuote, Tokens::SimpleQuote) | 
            (Tokens::DupleQuote, Tokens::DupleQuote)
        )
    }
    
    pub fn is_opened_brace(tk: Tokens) -> bool {
        
        match tk {
            Tokens::LeftBrace => true,
            _ => false
        }
    }

    pub fn is_closed_brace(tk: Tokens) -> bool {
        match tk {
            Tokens::RightBrace => true,
            _ => false
        }
    }
    
    pub fn is_opened(tk: Tokens) -> bool{
        match tk {
            Tokens::LeftBrace |
            Tokens::LeftParenthesis | 
            Tokens::LeftBracket => true,
            _ => false
        }
    }

    pub fn is_close(tk: Tokens) -> bool{
        match tk {
            Tokens::RightBrace |
            Tokens::RightParenthesis | 
            Tokens::RightBracket => true,
            _ => false
        }
    }
}

pub use structures::*;